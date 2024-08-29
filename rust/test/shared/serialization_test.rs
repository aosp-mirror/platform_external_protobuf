// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use googletest::prelude::*;
use unittest_proto::TestAllTypes;

#[test]
fn serialize_deserialize_message() {
    let mut msg = TestAllTypes::new();
    msg.optional_int64_mut().set(42);
    msg.optional_bool_mut().set(true);
    msg.optional_bytes_mut().set(b"serialize deserialize test");

    let serialized = msg.serialize();

    let mut msg2 = TestAllTypes::new();
    assert!(msg2.deserialize(&serialized).is_ok());

    assert_that!(msg.optional_int64(), eq(msg2.optional_int64()));
    assert_that!(msg.optional_bool(), eq(msg2.optional_bool()));
    assert_that!(msg.optional_bytes(), eq(msg2.optional_bytes()));
}

#[test]
fn deserialize_empty() {
    let mut msg = TestAllTypes::new();
    assert!(msg.deserialize(&[]).is_ok());
}

#[test]
fn deserialize_error() {
    let mut msg = TestAllTypes::new();
    let data = b"not a serialized proto";
    assert!(msg.deserialize(&*data).is_err());
}

#[test]
fn set_bytes_with_serialized_data() {
    let mut msg = TestAllTypes::new();
    msg.optional_int64_mut().set(42);
    msg.optional_bool_mut().set(true);
    let mut msg2 = TestAllTypes::new();
    msg2.optional_bytes_mut().set(msg.serialize());
    assert_that!(msg2.optional_bytes(), eq(msg.serialize().as_ref()));
}
