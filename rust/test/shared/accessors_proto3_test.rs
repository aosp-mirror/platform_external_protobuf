// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

/// Tests covering accessors for singular bool, int32, int64, and bytes fields
/// on proto3.
use googletest::prelude::*;
use matchers::{is_set, is_unset};
use protobuf::Optional;
use unittest_proto3::{TestAllTypes, TestAllTypes_};
use unittest_proto3_optional::{TestProto3Optional, TestProto3Optional_};

#[test]
fn test_fixed32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_fixed32(), eq(0));
    assert_that!(msg.optional_fixed32_mut().get(), eq(0));

    msg.optional_fixed32_mut().set(42);
    assert_that!(msg.optional_fixed32_mut().get(), eq(42));
    assert_that!(msg.optional_fixed32(), eq(42));

    msg.optional_fixed32_mut().set(u32::default());
    assert_that!(msg.optional_fixed32(), eq(0));
    assert_that!(msg.optional_fixed32_mut().get(), eq(0));
}

#[test]
fn test_bool_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_bool(), eq(false));
    assert_that!(msg.optional_bool_mut().get(), eq(false));

    msg.optional_bool_mut().set(true);
    assert_that!(msg.optional_bool(), eq(true));
    assert_that!(msg.optional_bool_mut().get(), eq(true));

    msg.optional_bool_mut().set(bool::default());
    assert_that!(msg.optional_bool(), eq(false));
    assert_that!(msg.optional_bool_mut().get(), eq(false));
}

#[test]
fn test_bytes_accessors() {
    let mut msg = TestAllTypes::new();
    // Note: even though it's named 'optional_bytes', the field is actually not
    // proto3 optional, so it does not support presence.
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(*msg.optional_bytes_mut().get(), empty());

    msg.optional_bytes_mut().set(b"accessors_test");
    assert_that!(msg.optional_bytes(), eq(b"accessors_test"));
    assert_that!(msg.optional_bytes_mut().get(), eq(b"accessors_test"));

    {
        let s = Vec::from(&b"hello world"[..]);
        msg.optional_bytes_mut().set(&s[..]);
    }
    assert_that!(msg.optional_bytes(), eq(b"hello world"));
    assert_that!(msg.optional_bytes_mut().get(), eq(b"hello world"));

    msg.optional_bytes_mut().clear();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(*msg.optional_bytes_mut().get(), empty());

    msg.optional_bytes_mut().set(b"");
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(*msg.optional_bytes_mut().get(), empty());
}

#[test]
fn test_optional_bytes_accessors() {
    let mut msg = TestProto3Optional::new();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Unset(&b""[..])));
    assert_that!(*msg.optional_bytes_mut().get(), empty());
    assert_that!(msg.optional_bytes_mut(), is_unset());

    {
        let s = Vec::from(&b"hello world"[..]);
        msg.optional_bytes_mut().set(&s[..]);
    }
    assert_that!(msg.optional_bytes(), eq(b"hello world"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"hello world"[..])));
    assert_that!(msg.optional_bytes_mut(), is_set());
    assert_that!(msg.optional_bytes_mut().get(), eq(b"hello world"));

    msg.optional_bytes_mut().or_default().set(b"accessors_test");
    assert_that!(msg.optional_bytes(), eq(b"accessors_test"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"accessors_test"[..])));
    assert_that!(msg.optional_bytes_mut(), is_set());
    assert_that!(msg.optional_bytes_mut().get(), eq(b"accessors_test"));
    assert_that!(msg.optional_bytes_mut().or_default().get(), eq(b"accessors_test"));

    msg.optional_bytes_mut().clear();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Unset(&b""[..])));
    assert_that!(msg.optional_bytes_mut(), is_unset());

    msg.optional_bytes_mut().set(b"");
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b""[..])));

    msg.optional_bytes_mut().clear();
    msg.optional_bytes_mut().or_default();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b""[..])));

    msg.optional_bytes_mut().or_default().set(b"\xffbinary\x85non-utf8");
    assert_that!(msg.optional_bytes(), eq(b"\xffbinary\x85non-utf8"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"\xffbinary\x85non-utf8"[..])));
    assert_that!(msg.optional_bytes_mut(), is_set());
    assert_that!(msg.optional_bytes_mut().get(), eq(b"\xffbinary\x85non-utf8"));
    assert_that!(msg.optional_bytes_mut().or_default().get(), eq(b"\xffbinary\x85non-utf8"));
}

#[test]
fn test_string_accessors() {
    let mut msg = TestAllTypes::new();
    // Note: even though it's named 'optional_string', the field is actually not
    // proto3 optional, so it does not support presence.
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(*msg.optional_string_mut().get().as_bytes(), empty());

    msg.optional_string_mut().set("accessors_test");
    assert_that!(msg.optional_string(), eq("accessors_test"));
    assert_that!(msg.optional_string_mut().get(), eq("accessors_test"));

    {
        let s = String::from("hello world");
        msg.optional_string_mut().set(&s[..]);
    }
    assert_that!(msg.optional_string(), eq("hello world"));
    assert_that!(msg.optional_string_mut().get(), eq("hello world"));

    msg.optional_string_mut().clear();
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(*msg.optional_string_mut().get().as_bytes(), empty());

    msg.optional_string_mut().set("");
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(*msg.optional_string_mut().get().as_bytes(), empty());
}

#[test]
fn test_optional_string_accessors() {
    let mut msg = TestProto3Optional::new();
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(msg.optional_string_opt(), eq(Optional::Unset("".into())));
    assert_that!(*msg.optional_string_mut().get().as_bytes(), empty());
    assert_that!(msg.optional_string_mut(), is_unset());

    {
        let s = String::from("hello world");
        msg.optional_string_mut().set(&s[..]);
    }
    assert_that!(msg.optional_string(), eq("hello world"));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("hello world".into())));
    assert_that!(msg.optional_string_mut(), is_set());
    assert_that!(msg.optional_string_mut().get(), eq("hello world"));

    msg.optional_string_mut().or_default().set("accessors_test");
    assert_that!(msg.optional_string(), eq("accessors_test"));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("accessors_test".into())));
    assert_that!(msg.optional_string_mut(), is_set());
    assert_that!(msg.optional_string_mut().get(), eq("accessors_test"));
    assert_that!(msg.optional_string_mut().or_default().get(), eq("accessors_test"));

    msg.optional_string_mut().clear();
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(msg.optional_string_opt(), eq(Optional::Unset("".into())));
    assert_that!(msg.optional_string_mut(), is_unset());

    msg.optional_string_mut().set("");
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("".into())));

    msg.optional_string_mut().clear();
    msg.optional_string_mut().or_default();
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("".into())));
}

#[test]
fn test_nested_enum_accessors() {
    use TestAllTypes_::NestedEnum;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Zero));
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Zero));

    msg.optional_nested_enum_mut().set(NestedEnum::Baz);
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Baz));
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Baz));

    msg.optional_nested_enum_mut().set(NestedEnum::default());
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Zero));
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Zero));
}

#[test]
fn test_optional_nested_enum_accessors() {
    use TestProto3Optional_::NestedEnum;

    let mut msg = TestProto3Optional::new();
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Unspecified));
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Unset(NestedEnum::Unspecified)));
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Unspecified));

    msg.optional_nested_enum_mut().set(NestedEnum::Baz);
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Baz));
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Set(NestedEnum::Baz)));
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Baz));

    msg.optional_nested_enum_mut().set(NestedEnum::default());
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Unspecified));
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Set(NestedEnum::Unspecified)));
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Unspecified));

    msg.optional_nested_enum_mut().clear();
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Unspecified));
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Unset(NestedEnum::Unspecified)));
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Unspecified));

    let mut field_mut = msg.optional_nested_enum_mut().or_default();
    assert_that!(field_mut.get(), eq(NestedEnum::Unspecified));
    field_mut.set(NestedEnum::Bar);
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Bar));
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Set(NestedEnum::Bar)));
    assert_that!(msg.optional_nested_enum_mut().get(), eq(NestedEnum::Bar));
}

#[test]
fn test_foreign_enum_accessors() {
    use unittest_proto3::ForeignEnum;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_foreign_enum(), eq(ForeignEnum::ForeignZero));
    assert_that!(msg.optional_foreign_enum_mut().get(), eq(ForeignEnum::ForeignZero));

    msg.optional_foreign_enum_mut().set(ForeignEnum::ForeignBaz);
    assert_that!(msg.optional_foreign_enum_mut().get(), eq(ForeignEnum::ForeignBaz));
    assert_that!(msg.optional_foreign_enum(), eq(ForeignEnum::ForeignBaz));

    msg.optional_foreign_enum_mut().set(ForeignEnum::default());
    assert_that!(msg.optional_foreign_enum(), eq(ForeignEnum::ForeignZero));
    assert_that!(msg.optional_foreign_enum_mut().get(), eq(ForeignEnum::ForeignZero));
}

#[test]
fn test_oneof_accessors() {
    use TestAllTypes_::OneofField::*;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));

    msg.oneof_uint32_mut().set(7);
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Set(7)));
    assert_that!(msg.oneof_field(), matches_pattern!(OneofUint32(eq(7))));

    msg.oneof_uint32_mut().clear();
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));

    // TODO: the submessage api is still in progress so we can't yet
    // cause a submsg to be set here.

    // msg.oneof_nested_message_mut().or_default(); // Cause the nested_message
    // field to become set.

    // assert_that!(msg.oneof_bytes_opt(),
    // eq(Optional::Unset(_))); assert_that!(msg.oneof_field(),
    // matches_pattern!(OneofNestedMessage(_)));

    msg.oneof_uint32_mut().set(7);
    msg.oneof_bytes_mut().set(b"123");
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.oneof_field(), matches_pattern!(OneofBytes(eq(b"123"))));

    msg.oneof_bytes_mut().clear();
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));
}

#[test]
fn test_oneof_enum_accessors() {
    use unittest_proto3::{
        TestOneof2,
        TestOneof2_::{Foo, NestedEnum},
    };

    let mut msg = TestOneof2::new();
    assert_that!(msg.foo_enum_opt(), eq(Optional::Unset(NestedEnum::Unknown)));
    assert_that!(msg.foo(), matches_pattern!(Foo::not_set(_)));

    msg.foo_enum_mut().set(NestedEnum::Bar);
    assert_that!(msg.foo_enum_opt(), eq(Optional::Set(NestedEnum::Bar)));
    assert_that!(msg.foo(), matches_pattern!(Foo::FooEnum(eq(NestedEnum::Bar))));
}

#[test]
fn test_oneof_mut_accessors() {
    use TestAllTypes_::OneofFieldMut::*;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.oneof_field_mut(), matches_pattern!(not_set(_)));

    msg.oneof_uint32_mut().set(7);

    match msg.oneof_field_mut() {
        OneofUint32(mut v) => {
            assert_that!(v.get(), eq(7));
            v.set(8);
            assert_that!(v.get(), eq(8));
        }
        f => panic!("unexpected field_mut type! {:?}", f),
    }

    // Confirm that the mut write above applies to both the field accessor and the
    // oneof view accessor.
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Set(8)));
    assert_that!(
        msg.oneof_field(),
        matches_pattern!(TestAllTypes_::OneofField::OneofUint32(eq(8)))
    );

    msg.oneof_uint32_mut().clear();
    assert_that!(msg.oneof_field_mut(), matches_pattern!(not_set(_)));

    msg.oneof_uint32_mut().set(7);
    msg.oneof_bytes_mut().set(b"123");
    assert_that!(msg.oneof_field_mut(), matches_pattern!(OneofBytes(_)));
}

#[test]
fn test_oneof_mut_enum_accessors() {
    use unittest_proto3::{
        TestOneof2,
        TestOneof2_::{FooMut, NestedEnum},
    };

    let mut msg = TestOneof2::new();
    assert_that!(msg.foo_enum_opt(), eq(Optional::Unset(NestedEnum::Unknown)));
    assert_that!(msg.foo_mut(), matches_pattern!(FooMut::not_set(_)));

    msg.foo_enum_mut().set(NestedEnum::Bar);
    assert_that!(msg.foo_enum_opt(), eq(Optional::Set(NestedEnum::Bar)));
    assert_that!(msg.foo_mut(), matches_pattern!(FooMut::FooEnum(_)));
}
