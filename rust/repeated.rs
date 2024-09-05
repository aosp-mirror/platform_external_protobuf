// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use std::fmt::{self, Debug};
use std::iter;
use std::iter::FusedIterator;
/// Repeated scalar fields are implemented around the runtime-specific
/// `RepeatedField` struct. `RepeatedField` stores an opaque pointer to the
/// runtime-specific representation of a repeated scalar (`upb_Array*` on upb,
/// and `RepeatedField<T>*` on cpp).
use std::marker::PhantomData;

use crate::{
    Mut, MutProxy, Proxied, SettableValue, View, ViewProxy,
    __internal::{Private, RawRepeatedField},
    __runtime::InnerRepeatedMut,
};

/// Views the elements in a `repeated` field of `T`.
#[repr(transparent)]
pub struct RepeatedView<'msg, T: ?Sized> {
    // This does not need to carry an arena in upb, so it can be just the raw repeated field
    raw: RawRepeatedField,
    _phantom: PhantomData<&'msg T>,
}

impl<'msg, T: ?Sized> Copy for RepeatedView<'msg, T> {}
impl<'msg, T: ?Sized> Clone for RepeatedView<'msg, T> {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl<'msg, T: ?Sized> Sync for RepeatedView<'msg, T> {}
unsafe impl<'msg, T: ?Sized> Send for RepeatedView<'msg, T> {}

impl<'msg, T: ?Sized> Debug for RepeatedView<'msg, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepeatedView").field("raw", &self.raw).finish()
    }
}

/// Mutates the elements in a `repeated` field of `T`.
pub struct RepeatedMut<'msg, T: ?Sized> {
    pub(crate) inner: InnerRepeatedMut<'msg>,
    _phantom: PhantomData<&'msg mut T>,
}

unsafe impl<'msg, T: ?Sized> Sync for RepeatedMut<'msg, T> {}

impl<'msg, T: ?Sized> Debug for RepeatedMut<'msg, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepeatedMut").field("raw", &self.inner.raw).finish()
    }
}

impl<'msg, T> RepeatedView<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    #[doc(hidden)]
    pub fn as_raw(&self, _private: Private) -> RawRepeatedField {
        self.raw
    }

    /// # Safety
    /// - `inner` must be valid to read from for `'msg`
    #[doc(hidden)]
    pub unsafe fn from_raw(_private: Private, raw: RawRepeatedField) -> Self {
        Self { raw, _phantom: PhantomData }
    }

    /// Gets the length of the repeated field.
    pub fn len(&self) -> usize {
        T::repeated_len(*self)
    }

    /// Returns true if the repeated field has no values.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the value at `index`.
    ///
    /// Returns `None` if `index > len`.
    pub fn get(self, index: usize) -> Option<View<'msg, T>> {
        if index >= self.len() {
            return None;
        }
        // SAFETY: `index` has been checked to be in-bounds
        Some(unsafe { self.get_unchecked(index) })
    }

    /// Gets the value at `index` without bounds-checking.
    ///
    /// # Safety
    /// Undefined behavior if `index >= len`
    pub unsafe fn get_unchecked(self, index: usize) -> View<'msg, T> {
        // SAFETY: in-bounds as promised
        unsafe { T::repeated_get_unchecked(self, index) }
    }

    /// Iterates over the values in the repeated field.
    pub fn iter(self) -> RepeatedIter<'msg, T> {
        self.into_iter()
    }
}

impl<'msg, T> RepeatedMut<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    /// # Safety
    /// - `inner` must be valid to read and write from for `'msg`
    /// - There must be no aliasing references or mutations on the same
    ///   underlying object.
    #[doc(hidden)]
    pub unsafe fn from_inner(_private: Private, inner: InnerRepeatedMut<'msg>) -> Self {
        Self { inner, _phantom: PhantomData }
    }

    /// # Safety
    /// - The return value must not be mutated through without synchronization.
    #[allow(dead_code)]
    pub(crate) unsafe fn into_inner(self) -> InnerRepeatedMut<'msg> {
        self.inner
    }

    #[doc(hidden)]
    pub fn as_raw(&mut self, _private: Private) -> RawRepeatedField {
        self.inner.raw
    }

    /// Gets the length of the repeated field.
    pub fn len(&self) -> usize {
        self.as_view().len()
    }

    /// Returns true if the repeated field has no values.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the value at `index`.
    ///
    /// Returns `None` if `index > len`.
    pub fn get(&self, index: usize) -> Option<View<T>> {
        self.as_view().get(index)
    }

    /// Gets the value at `index` without bounds-checking.
    ///
    /// # Safety
    /// Undefined behavior if `index >= len`
    pub unsafe fn get_unchecked(&self, index: usize) -> View<T> {
        // SAFETY: in-bounds as promised
        unsafe { self.as_view().get_unchecked(index) }
    }

    /// Appends `val` to the end of the repeated field.
    pub fn push(&mut self, val: View<T>) {
        // TODO: b/320936046 - Use SettableValue instead of View for added ergonomics.
        T::repeated_push(self.as_mut(), val);
    }

    /// Sets the value at `index` to the value `val`.
    ///
    /// # Panics
    /// Panics if `index >= len`
    pub fn set(&mut self, index: usize, val: View<T>) {
        let len = self.len();
        if index >= len {
            panic!("index {index} >= repeated len {len}");
        }
        // TODO: b/320936046 - Use SettableValue instead of View for added ergonomics.
        // SAFETY: `index` has been checked to be in-bounds.
        unsafe { self.set_unchecked(index, val) }
    }

    /// Sets the value at `index` to the value `val`.
    ///
    /// # Safety
    /// Undefined behavior if `index >= len`
    pub unsafe fn set_unchecked(&mut self, index: usize, val: View<T>) {
        // TODO: b/320936046 - Use SettableValue instead of View for added ergonomics.
        // SAFETY: `index` is in-bounds as promised by the caller.
        unsafe { T::repeated_set_unchecked(self.as_mut(), index, val) }
    }

    /// Iterates over the values in the repeated field.
    pub fn iter(&self) -> RepeatedIter<T> {
        self.as_view().into_iter()
    }

    /// Copies from the `src` repeated field into this one.
    ///
    /// Also provided by [`MutProxy::set`].
    pub fn copy_from(&mut self, src: RepeatedView<'_, T>) {
        T::repeated_copy_from(src, self.as_mut())
    }

    /// Clears the repeated field.
    pub fn clear(&mut self) {
        T::repeated_clear(self.as_mut())
    }
}

/// Types that can appear in a `Repeated<T>`.
///
/// This trait is implemented by generated code to communicate how the proxied
/// type can be manipulated for a repeated field.
///
/// Scalars and messages implement `ProxiedInRepeated`.
///
/// # Safety
/// - It must be sound to call `*_unchecked*(x)` with an `index` less than
///   `repeated_len(x)`.
pub unsafe trait ProxiedInRepeated: Proxied {
    /// Constructs a new owned `Repeated` field.
    #[doc(hidden)]
    fn repeated_new(_private: Private) -> Repeated<Self> {
        unimplemented!("not required")
    }

    /// Frees the repeated field in-place, for use in `Drop`.
    ///
    /// # Safety
    /// - After `repeated_free`, no other methods on the input are safe to call.
    #[doc(hidden)]
    unsafe fn repeated_free(_private: Private, _repeated: &mut Repeated<Self>) {
        unimplemented!("not required")
    }

    /// Gets the length of the repeated field.
    fn repeated_len(repeated: View<Repeated<Self>>) -> usize;

    /// Appends a new element to the end of the repeated field.
    fn repeated_push(repeated: Mut<Repeated<Self>>, val: View<Self>);

    /// Clears the repeated field of elements.
    fn repeated_clear(repeated: Mut<Repeated<Self>>);

    /// # Safety
    /// `index` must be less than `Self::repeated_len(repeated)`
    unsafe fn repeated_get_unchecked(repeated: View<Repeated<Self>>, index: usize) -> View<Self>;

    /// # Safety
    /// `index` must be less than `Self::repeated_len(repeated)`
    unsafe fn repeated_set_unchecked(repeated: Mut<Repeated<Self>>, index: usize, val: View<Self>);

    /// Copies the values in the `src` repeated field into `dest`.
    fn repeated_copy_from(src: View<Repeated<Self>>, dest: Mut<Repeated<Self>>);
}

/// An iterator over the values inside of a [`View<Repeated<T>>`](RepeatedView).
pub struct RepeatedIter<'msg, T: ?Sized> {
    view: RepeatedView<'msg, T>,
    current_index: usize,
}

impl<'msg, T: ?Sized> Debug for RepeatedIter<'msg, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepeatedIter")
            .field("view", &self.view)
            .field("current_index", &self.current_index)
            .finish()
    }
}

/// An iterator over the mutators inside of a [`Mut<Repeated<T>>`](RepeatedMut).
pub struct RepeatedIterMut<'msg, T: ?Sized> {
    mutator: RepeatedMut<'msg, T>,
    current_index: usize,
}

impl<'msg, T: ?Sized> Debug for RepeatedIterMut<'msg, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepeatedIterMut")
            .field("mutator", &self.mutator)
            .field("current_index", &self.current_index)
            .finish()
    }
}

/// A `repeated` field of `T`, used as the owned target for `Proxied`.
///
/// Users will generally write [`View<Repeated<T>>`](RepeatedView) or
/// [`Mut<Repeated<T>>`](RepeatedMut) to access the repeated elements
pub struct Repeated<T: ?Sized + ProxiedInRepeated> {
    inner: InnerRepeatedMut<'static>,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized + ProxiedInRepeated> Repeated<T> {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        T::repeated_new(Private)
    }

    pub(crate) unsafe fn from_inner(inner: InnerRepeatedMut<'static>) -> Self {
        Self { inner, _phantom: PhantomData }
    }

    #[allow(dead_code)]
    pub(crate) fn inner(&mut self) -> InnerRepeatedMut<'static> {
        self.inner
    }

    pub(crate) fn as_mut(&mut self) -> RepeatedMut<'_, T> {
        RepeatedMut { inner: self.inner, _phantom: PhantomData }
    }
}

impl<T: ?Sized + ProxiedInRepeated> Drop for Repeated<T> {
    fn drop(&mut self) {
        // SAFETY: only called once
        unsafe { T::repeated_free(Private, self) }
    }
}

// SAFETY: `Repeated` does not allow for shared mutability.
unsafe impl<T: ProxiedInRepeated> Sync for Repeated<T> {}

impl<T> Proxied for Repeated<T>
where
    T: ProxiedInRepeated + ?Sized,
{
    type View<'msg> = RepeatedView<'msg, T> where Repeated<T>: 'msg;
    type Mut<'msg> = RepeatedMut<'msg, T> where Repeated<T>: 'msg;
}

impl<'msg, T> ViewProxy<'msg> for RepeatedView<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    type Proxied = Repeated<T>;

    fn as_view(&self) -> View<'_, Self::Proxied> {
        *self
    }

    fn into_view<'shorter>(self) -> View<'shorter, Self::Proxied>
    where
        'msg: 'shorter,
    {
        RepeatedView { raw: self.raw, _phantom: PhantomData }
    }
}

impl<'msg, T> ViewProxy<'msg> for RepeatedMut<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    type Proxied = Repeated<T>;

    fn as_view(&self) -> View<'_, Self::Proxied> {
        RepeatedView { raw: self.inner.raw, _phantom: PhantomData }
    }

    fn into_view<'shorter>(self) -> View<'shorter, Self::Proxied>
    where
        'msg: 'shorter,
    {
        RepeatedView { raw: self.inner.raw, _phantom: PhantomData }
    }
}

impl<'msg, T> MutProxy<'msg> for RepeatedMut<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    fn as_mut(&mut self) -> Mut<'_, Self::Proxied> {
        RepeatedMut { inner: self.inner, _phantom: PhantomData }
    }

    fn into_mut<'shorter>(self) -> Mut<'shorter, Self::Proxied>
    where
        'msg: 'shorter,
    {
        RepeatedMut { inner: self.inner, _phantom: PhantomData }
    }
}

impl<'msg, T> SettableValue<Repeated<T>> for RepeatedView<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    fn set_on<'b>(self, _private: Private, mutator: Mut<'b, Repeated<T>>)
    where
        Repeated<T>: 'b,
    {
        T::repeated_copy_from(self, mutator)
    }
}

// TODO: impl ExactSizeIterator
impl<'msg, T> iter::Iterator for RepeatedIter<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    type Item = View<'msg, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.view.get(self.current_index);
        if val.is_some() {
            self.current_index += 1;
        }
        val
    }
}

impl<'msg, T: ?Sized + ProxiedInRepeated> ExactSizeIterator for RepeatedIter<'msg, T> {
    fn len(&self) -> usize {
        self.view.len()
    }
}

impl<'msg, T: ?Sized + ProxiedInRepeated> FusedIterator for RepeatedIter<'msg, T> {}

impl<'msg, T> iter::IntoIterator for RepeatedView<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    type Item = View<'msg, T>;
    type IntoIter = RepeatedIter<'msg, T>;

    fn into_iter(self) -> Self::IntoIter {
        RepeatedIter { view: self, current_index: 0 }
    }
}

impl<'msg, T> iter::IntoIterator for &'_ RepeatedView<'msg, T>
where
    T: ProxiedInRepeated + ?Sized + 'msg,
{
    type Item = View<'msg, T>;
    type IntoIter = RepeatedIter<'msg, T>;

    fn into_iter(self) -> Self::IntoIter {
        RepeatedIter { view: *self, current_index: 0 }
    }
}

impl<'borrow, T> iter::IntoIterator for &'borrow RepeatedMut<'_, T>
where
    T: ProxiedInRepeated + ?Sized + 'borrow,
{
    type Item = View<'borrow, T>;
    type IntoIter = RepeatedIter<'borrow, T>;

    fn into_iter(self) -> Self::IntoIter {
        RepeatedIter { view: self.as_view(), current_index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn test_primitive_repeated() {
        macro_rules! primitive_repeated_tests {
            ($($t:ty => [$($vals:expr),* $(,)?]),* $(,)?) => {
                $({
                // Constructs a new, owned, `Repeated`, only used for tests.
                let mut r = Repeated::<$t>::new();
                let mut r = r.as_mut();
                assert_that!(r.len(), eq(0));
                assert!(r.iter().next().is_none(), "starts with empty iter");
                assert!(r.iter().next().is_none(), "starts with empty mut iter");
                assert!(r.is_empty(), "starts is_empty");

                let mut expected_len = 0usize;
                $(
                    let val: View<$t> = $vals;
                    r.push(val);
                    assert_that!(r.get(expected_len), eq(Some(val)));
                    expected_len += 1;
                    assert_that!(r.len(), eq(expected_len));

                )*
                assert_that!(
                    r.iter().collect::<Vec<$t>>(), elements_are![$(eq($vals)),*]);
                r.set(0, <$t as Default>::default());
                assert_that!(r.get(0).expect("elem 0"), eq(<$t as Default>::default()));

                r.clear();
                assert!(r.is_empty(), "is_empty after clear");
                assert!(r.iter().next().is_none(), "iter empty after clear");
                assert!(r.into_iter().next().is_none(), "mut iter empty after clear");
                })*
            }
        }
        primitive_repeated_tests!(
            u32 => [1,2,3],
            i32 => [1,2],
            f64 => [10.0, 0.1234f64],
            bool => [false, true, true, false],
        );
    }
}
