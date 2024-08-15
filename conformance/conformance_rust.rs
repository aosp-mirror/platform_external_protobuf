// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use conformance_proto::{ConformanceRequest, ConformanceResponse};
use conformance_rust_overlay_hack_proto::ConformanceRequestRustOverlayHack;

#[cfg(cpp_kernel)]
use protobuf_cpp as kernel;

#[cfg(upb_kernel)]
use protobuf_upb as kernel;

use kernel::Optional::{Set, Unset};

use std::io::{self, ErrorKind, Read, Write};
use test_messages_proto2::TestAllTypesProto2;
use test_messages_proto2_editions_proto::TestAllTypesProto2 as EditionsTestAllTypesProto2;
use test_messages_proto3::TestAllTypesProto3;
use test_messages_proto3_editions_proto::TestAllTypesProto3 as EditionsTestAllTypesProto3;

/// Returns Some(i32) if a binary read can succeed from stdin.
/// Returns None if we have reached an EOF.
/// Panics for any other error reading.
fn read_little_endian_i32_from_stdin() -> Option<i32> {
    let mut buffer = [0_u8; 4];
    if let Err(e) = io::stdin().read_exact(&mut buffer) {
        match e.kind() {
            ErrorKind::UnexpectedEof => None,
            _ => panic!("failed to read i32 from stdin"),
        }
    } else {
        Some(i32::from_le_bytes(buffer))
    }
}

/// Returns Some of a conformance request read from stdin.
/// Returns None if we have hit an EOF that suggests the test suite is complete.
/// Panics in any other case (e.g. an EOF in a place that would imply a
/// programmer error in the conformance test suite).
fn read_request_from_stdin() -> Option<(ConformanceRequest, ConformanceRequestRustOverlayHack)> {
    let msg_len = read_little_endian_i32_from_stdin()?;
    let mut serialized = vec![0_u8; msg_len as usize];
    io::stdin().read_exact(&mut serialized).unwrap();
    let mut req = ConformanceRequest::new();
    req.deserialize(&serialized).unwrap();

    // TODO: b/318373255 - Since enum accessors aren't available yet, we parse an
    // overlay with int32 field instead of an enum so that we can check if the
    // requested output is binary or not. This will be deleted once enum
    // accessors are supported.
    let mut req_overlay_hack = ConformanceRequestRustOverlayHack::new();
    req_overlay_hack.deserialize(&serialized).unwrap();

    Some((req, req_overlay_hack))
}

fn write_response_to_stdout(resp: &ConformanceResponse) {
    let bytes = resp.serialize();
    let len = bytes.len() as u32;
    let mut handle = io::stdout();
    handle.write_all(&len.to_le_bytes()).unwrap();
    handle.write(&bytes).unwrap();
    handle.flush().unwrap();
}

fn do_test(
    req: &ConformanceRequest,
    req_overlay_hack: &ConformanceRequestRustOverlayHack,
) -> ConformanceResponse {
    let mut resp = ConformanceResponse::new();
    let message_type = req.message_type();

    // TODO: b/318373255 - Use the enum once its supported.
    // if req.requested_output_format() != WireFormat.PROTOBUF {
    if req_overlay_hack.requested_output_format() != 1 {
        resp.skipped_mut().set("only wire format output implemented");
        return resp;
    }

    let bytes = match req.protobuf_payload_opt() {
        Unset(_) => {
            resp.skipped_mut().set("only wire format input implemented");
            return resp;
        }
        Set(bytes) => bytes,
    };

    let serialized = match message_type.as_bytes() {
        b"protobuf_test_messages.proto2.TestAllTypesProto2" => {
            let mut proto = TestAllTypesProto2::new();
            if let Err(_) = proto.deserialize(bytes) {
                resp.parse_error_mut().set("failed to parse bytes");
                return resp;
            }
            proto.serialize()
        }
        b"protobuf_test_messages.proto3.TestAllTypesProto3" => {
            let mut proto = TestAllTypesProto3::new();
            if let Err(_) = proto.deserialize(bytes) {
                resp.parse_error_mut().set("failed to parse bytes");
                return resp;
            }
            proto.serialize()
        }
        b"protobuf_test_messages.editions.proto2.TestAllTypesProto2" => {
            let mut proto = EditionsTestAllTypesProto2::new();
            if let Err(_) = proto.deserialize(bytes) {
                resp.parse_error_mut().set("failed to parse bytes");
                return resp;
            }
            proto.serialize()
        }
        b"protobuf_test_messages.editions.proto3.TestAllTypesProto3" => {
            let mut proto = EditionsTestAllTypesProto3::new();
            if let Err(_) = proto.deserialize(bytes) {
                resp.parse_error_mut().set("failed to parse bytes");
                return resp;
            }
            proto.serialize()
        }
        _ => panic!("unexpected msg type {message_type}"),
    };

    resp.protobuf_payload_mut().set(serialized);
    return resp;
}

fn main() {
    let mut total_runs = 0;
    while let Some((req, req_overlay_hack)) = read_request_from_stdin() {
        let resp = do_test(&req, &req_overlay_hack);
        write_response_to_stdout(&resp);
        total_runs += 1;
    }
    eprintln!("conformance_rust: received EOF from test runner after {total_runs} tests");
}
