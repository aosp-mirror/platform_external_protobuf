// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

//! Operating on borrowed data owned by a message is a central concept in
//! Protobuf (and Rust in general). The way this is normally accomplished in
//! Rust is to pass around references and operate on those. Unfortunately,
//! references come with two major drawbacks:
//!
//! * We must store the value somewhere in the memory to create a reference to
//!   it. The value must be readable by a single load. However for Protobuf
//!   fields it happens that the actual memory representation of a value differs
//!   from what users expect and it is an implementation detail that can change
//!   as more optimizations are implemented. For example, rarely accessed
//!   `int64` fields can be represented in a packed format with 32 bits for the
//!   value in the common case. Or, a single logical value can be spread across
//!   multiple memory locations. For example, presence information for all the
//!   fields in a protobuf message is centralized in a bitset.
//! * We cannot store extra data on the reference that might be necessary for
//!   correctly manipulating it (and custom-metadata DSTs do not exist yet in
//!   Rust). Concretely, messages, string, bytes, and repeated fields in UPB
//!   need to carry around an arena parameter separate from the data pointer to
//!   enable mutation (for example adding an element to a repeated field) or
//!   potentially to enable optimizations (for example referencing a string
//!   value using a Cord-like type instead of copying it if the source and
//!   target messages are on the same arena already). Mutable references to
//!   messages have one additional drawback: Rust allows users to
//!   indiscriminately run a bytewise swap() on mutable references, which could
//!   result in pointers to the wrong arena winding up on a message. For
//!   example, imagine swapping a submessage across two root messages allocated
//!   on distinct arenas A and B; after the swap, the message allocated in A may
//!   contain pointers from B by way of the submessage, because the swap does
//!   not know to fix up those pointers as needed. The C++ API uses
//!   message-owned arenas, and this ends up resembling self-referential types,
//!   which need `Pin` in order to be sound. However, `Pin` has much stronger
//!   guarantees than we need to uphold.
//!
//! These drawbacks put the "idiomatic Rust" goal in conflict with the
//! "performance", "evolvability", and "safety" goals. Given the project design
//! priorities we decided to not use plain Rust references. Instead, we
//! implemented the concept of "proxy" types. Proxy types are a reference-like
//! indirection between the user and the internal memory representation.

use crate::RepeatedMut;
use crate::__internal::Private;
use crate::repeated::ProxiedInRepeated;
use std::fmt::Debug;
use std::marker::{Send, Sync};

/// A type that can be accessed through a reference-like proxy.
///
/// An instance of a `Proxied` can be accessed
/// immutably via `Proxied::View` and mutably via `Proxied::Mut`.
///
/// All Protobuf field types implement `Proxied`.
pub trait Proxied {
    /// The proxy type that provides shared access to a `T`, like a `&'msg T`.
    ///
    /// Most code should use the type alias [`View`].
    type View<'msg>: ViewProxy<'msg, Proxied = Self> + Copy + Send + SettableValue<Self>
    where
        Self: 'msg;

    /// The proxy type that provides exclusive mutable access to a `T`, like a
    /// `&'msg mut T`.
    ///
    /// Most code should use the type alias [`Mut`].
    type Mut<'msg>: MutProxy<'msg, Proxied = Self>
    where
        Self: 'msg;
}

/// A proxy type that provides shared access to a `T`, like a `&'msg T`.
///
/// This is more concise than fully spelling the associated type.
#[allow(dead_code)]
pub type View<'msg, T> = <T as Proxied>::View<'msg>;

/// A proxy type that provides exclusive mutable access to a `T`, like a
/// `&'msg mut T`.
///
/// This is more concise than fully spelling the associated type.
#[allow(dead_code)]
pub type Mut<'msg, T> = <T as Proxied>::Mut<'msg>;

/// Declares conversion operations common to all views.
///
/// This trait is intentionally made non-object-safe to prevent a potential
/// future incompatible change.
pub trait ViewProxy<'msg>: 'msg + Sync + Unpin + Sized + Debug {
    type Proxied: 'msg + Proxied + ?Sized;

    /// Converts a borrow into a `View` with the lifetime of that borrow.
    ///
    /// In non-generic code we don't need to use `as_view` because the proxy
    /// types are covariant over `'msg`. However, generic code conservatively
    /// treats `'msg` as [invariant], therefore we need to call
    /// `as_view` to explicitly perform the operation that in concrete code
    /// coercion would perform implicitly.
    ///
    /// For example, the call to `.as_view()` in the following snippet
    /// wouldn't be necessary in concrete code:
    /// ```
    /// fn reborrow<'a, 'b, T>(x: &'b View<'a, T>) -> View<'b, T>
    /// where 'a: 'b, T: Proxied
    /// {
    ///   x.as_view()
    /// }
    /// ```
    ///
    /// [invariant]: https://doc.rust-lang.org/nomicon/subtyping.html#variance
    fn as_view(&self) -> View<'_, Self::Proxied>;

    /// Converts into a `View` with a potentially shorter lifetime.
    ///
    /// In non-generic code we don't need to use `into_view` because the proxy
    /// types are covariant over `'msg`. However, generic code conservatively
    /// treats `'msg` as [invariant], therefore we need to call
    /// `into_view` to explicitly perform the operation that in concrete
    /// code coercion would perform implicitly.
    ///
    /// ```
    /// fn reborrow_generic_view_into_view<'a, 'b, T>(
    ///     x: View<'a, T>,
    ///     y: View<'b, T>,
    /// ) -> [View<'b, T>; 2]
    /// where
    ///     T: Proxied,
    ///     'a: 'b,
    /// {
    ///     // `[x, y]` fails to compile because `'a` is not the same as `'b` and the `View`
    ///     // lifetime parameter is (conservatively) invariant.
    ///     // `[x.as_view(), y]` fails because that borrow cannot outlive `'b`.
    ///     [x.into_view(), y]
    /// }
    /// ```
    ///
    /// [invariant]: https://doc.rust-lang.org/nomicon/subtyping.html#variance
    fn into_view<'shorter>(self) -> View<'shorter, Self::Proxied>
    where
        'msg: 'shorter;
}

/// Declares operations common to all mutators.
///
/// This trait is intentionally made non-object-safe to prevent a potential
/// future incompatible change.
pub trait MutProxy<'msg>: ViewProxy<'msg> {
    /// Gets an immutable view of this field. This is shorthand for `as_view`.
    ///
    /// This provides a shorter lifetime than `into_view` but can also be called
    /// multiple times - if the result of `get` is not living long enough
    /// for your use, use that instead.
    fn get(&self) -> View<'_, Self::Proxied> {
        self.as_view()
    }

    /// Sets this field to the given `val`.
    ///
    /// Any borrowed data from `val` will be cloned.
    fn set(&mut self, val: impl SettableValue<Self::Proxied>) {
        val.set_on(Private, self.as_mut())
    }

    /// Converts a borrow into a `Mut` with the lifetime of that borrow.
    ///
    /// This function enables calling multiple methods consuming `self`, for
    /// example:
    ///
    /// ```ignore
    ///   let mut sub: Mut<SubMsg> = msg.submsg_mut().or_default();
    ///   sub.as_mut().field_x_mut().set(10);  // field_x_mut is fn(self)
    ///   sub.field_y_mut().set(20);  // `sub` is now consumed
    /// ```
    ///
    /// `as_mut` is also useful in generic code to explicitly perform the
    /// operation that in concrete code coercion would perform implicitly.
    fn as_mut(&mut self) -> Mut<'_, Self::Proxied>;

    /// Converts into a `Mut` with a potentially shorter lifetime.
    ///
    /// In non-generic code we don't need to use `into_mut` because the proxy
    /// types are covariant over `'msg`. However, generic code conservatively
    /// treats `'msg` as [invariant], therefore we need to call
    /// `into_mut` to explicitly perform the operation that in concrete code
    /// coercion would perform implicitly.
    ///
    /// ```
    /// fn reborrow_generic_mut_into_mut<'a, 'b, T>(x: Mut<'a, T>, y: Mut<'b, T>) -> [Mut<'b, T>; 2]
    /// where
    ///     T: Proxied,
    ///     'a: 'b,
    /// {
    ///     // `[x, y]` fails to compile because `'a` is not the same as `'b` and the `Mut`
    ///     // lifetime parameter is (conservatively) invariant.
    ///     // `[x.as_mut(), y]` fails because that borrow cannot outlive `'b`.
    ///     [x.into_mut(), y]
    /// }
    /// ```
    ///
    /// [invariant]: https://doc.rust-lang.org/nomicon/subtyping.html#variance
    fn into_mut<'shorter>(self) -> Mut<'shorter, Self::Proxied>
    where
        'msg: 'shorter;
}

// TODO: move this to `optional.rs` as it's only used for optionals
/// `Proxied` types that can be optionally set or unset.
///
/// All scalar and message types implement `ProxiedWithPresence`, while repeated
/// types don't.
pub trait ProxiedWithPresence: Proxied {
    /// The data necessary to store a present field mutator proxying `Self`.
    /// This is the contents of `PresentField<'msg, Self>`.
    type PresentMutData<'msg>: MutProxy<'msg, Proxied = Self>;

    /// The data necessary to store an absent field mutator proxying `Self`.
    /// This is the contents of `AbsentField<'msg, Self>`.
    type AbsentMutData<'msg>: ViewProxy<'msg, Proxied = Self>;

    /// Clears a present field.
    fn clear_present_field(present_mutator: Self::PresentMutData<'_>) -> Self::AbsentMutData<'_>;

    /// Sets an absent field to its default value.
    ///
    /// This can be more efficient than setting with a default value, e.g.
    /// a default submessage could share resources with the parent message.
    fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>) -> Self::PresentMutData<'_>;
}

/// Values that can be used to set a field of `T`.
pub trait SettableValue<T>: Sized
where
    T: Proxied + ?Sized,
{
    /// Consumes `self` to set the given mutator to the value of `self`.
    #[doc(hidden)]
    fn set_on<'msg>(self, _private: Private, mutator: Mut<'msg, T>)
    where
        T: 'msg;

    /// Consumes `self` and `absent_mutator` to set the given empty field to
    /// the value of `self`.
    #[doc(hidden)]
    fn set_on_absent(
        self,
        _private: Private,
        absent_mutator: T::AbsentMutData<'_>,
    ) -> T::PresentMutData<'_>
    where
        T: ProxiedWithPresence,
    {
        let mut present = T::set_absent_to_default(absent_mutator);
        self.set_on(Private, present.as_mut());
        present
    }

    /// Consumes `self` and `present_mutator` to set the given present field
    /// to the value of `self`.
    #[doc(hidden)]
    fn set_on_present(self, _private: Private, mut present_mutator: T::PresentMutData<'_>)
    where
        T: ProxiedWithPresence,
    {
        self.set_on(Private, present_mutator.as_mut())
    }

    /// Consumes `self` and `repeated_mutator` to set the value at the
    /// given index to the value of `self`.
    ///
    /// # Safety
    /// `index` must be less than `repeated_mutator.len()`
    #[doc(hidden)]
    unsafe fn set_on_repeated_unchecked(
        self,
        _private: Private,
        mut _repeated_mutator: RepeatedMut<T>,
        _index: usize,
    ) where
        T: ProxiedInRepeated,
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq)]
    struct MyProxied {
        val: String,
    }

    impl MyProxied {
        fn as_view(&self) -> View<'_, Self> {
            MyProxiedView { my_proxied_ref: self }
        }

        fn as_mut(&mut self) -> Mut<'_, Self> {
            MyProxiedMut { my_proxied_ref: self }
        }
    }

    impl Proxied for MyProxied {
        type View<'msg> = MyProxiedView<'msg>;
        type Mut<'msg> = MyProxiedMut<'msg>;
    }

    #[derive(Debug, Clone, Copy)]
    struct MyProxiedView<'msg> {
        my_proxied_ref: &'msg MyProxied,
    }

    impl MyProxiedView<'_> {
        fn val(&self) -> &str {
            &self.my_proxied_ref.val
        }
    }

    impl<'msg> ViewProxy<'msg> for MyProxiedView<'msg> {
        type Proxied = MyProxied;

        fn as_view(&self) -> View<'msg, MyProxied> {
            *self
        }

        fn into_view<'shorter>(self) -> View<'shorter, MyProxied>
        where
            'msg: 'shorter,
        {
            self
        }
    }

    #[derive(Debug)]
    struct MyProxiedMut<'msg> {
        my_proxied_ref: &'msg mut MyProxied,
    }

    impl<'msg> ViewProxy<'msg> for MyProxiedMut<'msg> {
        type Proxied = MyProxied;

        fn as_view(&self) -> View<'_, MyProxied> {
            MyProxiedView { my_proxied_ref: self.my_proxied_ref }
        }
        fn into_view<'shorter>(self) -> View<'shorter, MyProxied>
        where
            'msg: 'shorter,
        {
            MyProxiedView { my_proxied_ref: self.my_proxied_ref }
        }
    }

    impl<'msg> MutProxy<'msg> for MyProxiedMut<'msg> {
        fn as_mut(&mut self) -> Mut<'_, MyProxied> {
            MyProxiedMut { my_proxied_ref: self.my_proxied_ref }
        }

        fn into_mut<'shorter>(self) -> Mut<'shorter, MyProxied>
        where
            'msg: 'shorter,
        {
            self
        }
    }

    impl SettableValue<MyProxied> for MyProxiedView<'_> {
        fn set_on<'msg>(self, _private: Private, mutator: Mut<'msg, MyProxied>)
        where
            MyProxied: 'msg,
        {
            mutator.my_proxied_ref.val = self.my_proxied_ref.val.clone();
        }
    }

    impl SettableValue<MyProxied> for String {
        fn set_on<'msg>(self, _private: Private, mutator: Mut<'msg, MyProxied>)
        where
            MyProxied: 'msg,
        {
            mutator.my_proxied_ref.val = self;
        }
    }

    impl SettableValue<MyProxied> for &'_ str {
        fn set_on<'msg>(self, _private: Private, mutator: Mut<'msg, MyProxied>)
        where
            MyProxied: 'msg,
        {
            mutator.my_proxied_ref.val.replace_range(.., self);
        }
    }

    impl SettableValue<MyProxied> for Cow<'_, str> {
        fn set_on<'msg>(self, _private: Private, mutator: Mut<'msg, MyProxied>)
        where
            MyProxied: 'msg,
        {
            match self {
                Cow::Owned(x) => <String as SettableValue<MyProxied>>::set_on(x, Private, mutator),
                Cow::Borrowed(x) => <&str as SettableValue<MyProxied>>::set_on(x, Private, mutator),
            }
        }
    }

    #[test]
    fn test_as_view() {
        let my_proxied = MyProxied { val: "Hello World".to_string() };

        let my_view = my_proxied.as_view();

        assert_that!(my_view.val(), eq(&my_proxied.val));
    }

    #[test]
    fn test_as_mut() {
        let mut my_proxied = MyProxied { val: "Hello World".to_string() };

        let mut my_mut = my_proxied.as_mut();
        my_mut.set("Hello indeed".to_string());

        let val_after_set = my_mut.as_view().val().to_string();
        assert_that!(my_proxied.val, eq(val_after_set));
        assert_that!(my_proxied.val, eq("Hello indeed"));
    }

    fn reborrow_mut_into_view<'msg>(x: Mut<'msg, MyProxied>) -> View<'msg, MyProxied> {
        // x.as_view() fails to compile with:
        //   `ERROR: attempt to return function-local borrowed content`
        x.into_view() // OK: we return the same lifetime as we got in.
    }

    #[test]
    fn test_mut_into_view() {
        let mut my_proxied = MyProxied { val: "Hello World".to_string() };
        reborrow_mut_into_view(my_proxied.as_mut());
    }

    fn require_unified_lifetimes<'msg>(_x: Mut<'msg, MyProxied>, _y: View<'msg, MyProxied>) {}

    #[test]
    fn test_require_unified_lifetimes() {
        let mut my_proxied = MyProxied { val: "Hello1".to_string() };
        let my_mut = my_proxied.as_mut();

        {
            let other_proxied = MyProxied { val: "Hello2".to_string() };
            let other_view = other_proxied.as_view();
            require_unified_lifetimes(my_mut, other_view);
        }
    }

    fn reborrow_generic_as_view<'a, 'b, T>(
        x: &'b mut Mut<'a, T>,
        y: &'b View<'a, T>,
    ) -> [View<'b, T>; 2]
    where
        T: Proxied,
        'a: 'b,
    {
        // `[x, y]` fails to compile because `'a` is not the same as `'b` and the `View`
        // lifetime parameter is (conservatively) invariant.
        [x.as_view(), y.as_view()]
    }

    #[test]
    fn test_reborrow_generic_as_view() {
        let mut my_proxied = MyProxied { val: "Hello1".to_string() };
        let mut my_mut = my_proxied.as_mut();
        let my_ref = &mut my_mut;

        {
            let other_proxied = MyProxied { val: "Hello2".to_string() };
            let other_view = other_proxied.as_view();
            reborrow_generic_as_view::<MyProxied>(my_ref, &other_view);
        }
    }

    fn reborrow_generic_view_into_view<'a, 'b, T>(
        x: View<'a, T>,
        y: View<'b, T>,
    ) -> [View<'b, T>; 2]
    where
        T: Proxied,
        'a: 'b,
    {
        // `[x, y]` fails to compile because `'a` is not the same as `'b` and the `View`
        // lifetime parameter is (conservatively) invariant.
        // `[x.as_view(), y]` fails because that borrow cannot outlive `'b`.
        [x.into_view(), y]
    }

    #[test]
    fn test_reborrow_generic_into_view() {
        let my_proxied = MyProxied { val: "Hello1".to_string() };
        let my_view = my_proxied.as_view();

        {
            let other_proxied = MyProxied { val: "Hello2".to_string() };
            let other_view = other_proxied.as_view();
            reborrow_generic_view_into_view::<MyProxied>(my_view, other_view);
        }
    }

    fn reborrow_generic_mut_into_view<'a, 'b, T>(x: Mut<'a, T>, y: View<'b, T>) -> [View<'b, T>; 2]
    where
        T: Proxied,
        'a: 'b,
    {
        [x.into_view(), y]
    }

    #[test]
    fn test_reborrow_generic_mut_into_view() {
        let mut my_proxied = MyProxied { val: "Hello1".to_string() };
        let my_mut = my_proxied.as_mut();

        {
            let other_proxied = MyProxied { val: "Hello2".to_string() };
            let other_view = other_proxied.as_view();
            reborrow_generic_mut_into_view::<MyProxied>(my_mut, other_view);
        }
    }

    fn reborrow_generic_mut_into_mut<'a, 'b, T>(x: Mut<'a, T>, y: Mut<'b, T>) -> [Mut<'b, T>; 2]
    where
        T: Proxied,
        'a: 'b,
    {
        // `[x, y]` fails to compile because `'a` is not the same as `'b` and the `Mut`
        // lifetime parameter is (conservatively) invariant.
        // `[x.as_mut(), y]` fails because that borrow cannot outlive `'b`.
        [x.into_mut(), y]
    }

    #[test]
    fn test_reborrow_generic_mut_into_mut() {
        let mut my_proxied = MyProxied { val: "Hello1".to_string() };
        let my_mut = my_proxied.as_mut();

        {
            let mut other_proxied = MyProxied { val: "Hello2".to_string() };
            let other_mut = other_proxied.as_mut();
            // No need to reborrow, even though lifetime of &other_view is different
            // than the lifetiem of my_ref. Rust references are covariant over their
            // lifetime.
            reborrow_generic_mut_into_mut::<MyProxied>(my_mut, other_mut);
        }
    }

    #[test]
    fn test_set() {
        let mut my_proxied = MyProxied::default();
        my_proxied.as_mut().set("hello");
        assert_that!(my_proxied.as_view().val(), eq("hello"));

        my_proxied.as_mut().set(String::from("hello2"));
        assert_that!(my_proxied.as_view().val(), eq("hello2"));

        my_proxied.as_mut().set(Cow::Borrowed("hello3"));
        assert_that!(my_proxied.as_view().val(), eq("hello3"));
    }
}
