// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#include "text_format_conformance_suite.h"

#include <cstddef>
#include <string>
#include <vector>

#include "absl/log/absl_log.h"
#include "absl/log/die_if_null.h"
#include "absl/strings/str_cat.h"
#include "conformance_test.h"
#include "google/protobuf/editions/golden/test_messages_proto2_editions.pb.h"
#include "google/protobuf/editions/golden/test_messages_proto3_editions.pb.h"
#include "google/protobuf/test_messages_proto2.pb.h"
#include "google/protobuf/test_messages_proto3.pb.h"
#include "google/protobuf/text_format.h"

using conformance::ConformanceRequest;
using conformance::ConformanceResponse;
using conformance::WireFormat;
using protobuf_test_messages::proto2::TestAllTypesProto2;
using protobuf_test_messages::proto2::UnknownToTestAllTypes;
using protobuf_test_messages::proto3::TestAllTypesProto3;
using TestAllTypesProto2Editions =
    protobuf_test_messages::editions::proto2::TestAllTypesProto2;
using TestAllTypesProto3Editions =
    protobuf_test_messages::editions::proto3::TestAllTypesProto3;

namespace google {
namespace protobuf {

// The number of repetitions to use for performance tests.
// Corresponds approx to 500KB wireformat bytes.
static const size_t kPerformanceRepeatCount = 50000;

TextFormatConformanceTestSuite::TextFormatConformanceTestSuite() {
  SetFailureListFlagName("--text_format_failure_list");
}

bool TextFormatConformanceTestSuite::ParseTextFormatResponse(
    const ConformanceResponse& response,
    const ConformanceRequestSetting& setting, Message* test_message) {
  TextFormat::Parser parser;
  const ConformanceRequest& request = setting.GetRequest();
  if (request.print_unknown_fields()) {
    parser.AllowFieldNumber(true);
  }
  if (!parser.ParseFromString(response.text_payload(), test_message)) {
    ABSL_LOG(ERROR) << "INTERNAL ERROR: internal text->protobuf transcode "
                    << "yielded unparseable proto. Text payload: "
                    << response.text_payload();
    return false;
  }

  return true;
}

bool TextFormatConformanceTestSuite::ParseResponse(
    const ConformanceResponse& response,
    const ConformanceRequestSetting& setting, Message* test_message) {
  const ConformanceRequest& request = setting.GetRequest();
  WireFormat requested_output = request.requested_output_format();
  const std::string& test_name = setting.GetTestName();
  ConformanceLevel level = setting.GetLevel();

  switch (response.result_case()) {
    case ConformanceResponse::kProtobufPayload: {
      if (requested_output != conformance::PROTOBUF) {
        ReportFailure(test_name, level, request, response,
                      absl::StrCat("Test was asked for ",
                                   WireFormatToString(requested_output),
                                   " output but provided PROTOBUF instead."));
        return false;
      }

      if (!test_message->ParseFromString(response.protobuf_payload())) {
        ReportFailure(test_name, level, request, response,
                      "Protobuf output we received from test was unparseable.");
        return false;
      }

      break;
    }

    case ConformanceResponse::kTextPayload: {
      if (requested_output != conformance::TEXT_FORMAT) {
        ReportFailure(
            test_name, level, request, response,
            absl::StrCat("Test was asked for ",
                         WireFormatToString(requested_output),
                         " output but provided TEXT_FORMAT instead."));
        return false;
      }

      if (!ParseTextFormatResponse(response, setting, test_message)) {
        ReportFailure(
            test_name, level, request, response,
            "TEXT_FORMAT output we received from test was unparseable.");
        return false;
      }

      break;
    }

    default:
      ABSL_LOG(FATAL) << test_name
                      << ": unknown payload type: " << response.result_case();
  }

  return true;
}

void TextFormatConformanceTestSuite::RunSuiteImpl() {
  TextFormatConformanceTestSuiteImpl<TestAllTypesProto2>(this);
  TextFormatConformanceTestSuiteImpl<TestAllTypesProto3>(this);
  if (maximum_edition_ >= Edition::EDITION_2023) {
    TextFormatConformanceTestSuiteImpl<TestAllTypesProto2Editions>(this);
    TextFormatConformanceTestSuiteImpl<TestAllTypesProto3Editions>(this);
  }
}

template <typename MessageType>
TextFormatConformanceTestSuiteImpl<MessageType>::
    TextFormatConformanceTestSuiteImpl(TextFormatConformanceTestSuite* suite)
    : suite_(*ABSL_DIE_IF_NULL(suite)) {
  // Flag control performance tests to keep them internal and opt-in only
  if (suite_.performance_) {
    RunTextFormatPerformanceTests();
  } else {
    if (MessageType::GetDescriptor()->name() == "TestAllTypesProto2") {
      RunGroupTests();
    }
    if (MessageType::GetDescriptor()->name() == "TestAllTypesProto3") {
      RunAnyTests();
      // TODO Run these over proto2 also.
      RunAllTests();
    }
  }
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::ExpectParseFailure(
    const std::string& test_name, ConformanceLevel level,
    const std::string& input) {
  MessageType prototype;
  // We don't expect output, but if the program erroneously accepts the protobuf
  // we let it send its response as this.  We must not leave it unspecified.
  ConformanceRequestSetting setting(
      level, conformance::TEXT_FORMAT, conformance::TEXT_FORMAT,
      conformance::TEXT_FORMAT_TEST, prototype, test_name, input);
  const ConformanceRequest& request = setting.GetRequest();
  ConformanceResponse response;
  std::string effective_test_name = absl::StrCat(
      setting.ConformanceLevelToString(level), ".",
      setting.GetSyntaxIdentifier(), ".TextFormatInput.", test_name);

  suite_.RunTest(effective_test_name, request, &response);
  if (response.result_case() == ConformanceResponse::kParseError) {
    suite_.ReportSuccess(effective_test_name);
  } else if (response.result_case() == ConformanceResponse::kSkipped) {
    suite_.ReportSkip(effective_test_name, request, response);
  } else {
    suite_.ReportFailure(effective_test_name, level, request, response,
                         "Should have failed to parse, but didn't.");
  }
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::RunValidTextFormatTest(
    const std::string& test_name, ConformanceLevel level,
    const std::string& input_text) {
  MessageType prototype;
  RunValidTextFormatTestWithMessage(test_name, level, input_text, prototype);
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::
    RunValidTextFormatTestWithMessage(const std::string& test_name,
                                      ConformanceLevel level,
                                      const std::string& input_text,
                                      const Message& message) {
  ConformanceRequestSetting setting1(
      level, conformance::TEXT_FORMAT, conformance::PROTOBUF,
      conformance::TEXT_FORMAT_TEST, message, test_name, input_text);
  suite_.RunValidInputTest(setting1, input_text);
  ConformanceRequestSetting setting2(
      level, conformance::TEXT_FORMAT, conformance::TEXT_FORMAT,
      conformance::TEXT_FORMAT_TEST, message, test_name, input_text);
  suite_.RunValidInputTest(setting2, input_text);
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::
    RunValidTextFormatTestWithExpected(const std::string& test_name,
                                       ConformanceLevel level,
                                       const std::string& input_text,
                                       const std::string& expected_text) {
  MessageType prototype;
  ConformanceRequestSetting setting1(
      level, conformance::TEXT_FORMAT, conformance::PROTOBUF,
      conformance::TEXT_FORMAT_TEST, prototype, test_name, input_text);
  suite_.RunValidInputTest(setting1, expected_text);
  ConformanceRequestSetting setting2(
      level, conformance::TEXT_FORMAT, conformance::TEXT_FORMAT,
      conformance::TEXT_FORMAT_TEST, prototype, test_name, input_text);
  suite_.RunValidInputTest(setting2, expected_text);
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<
    MessageType>::RunValidUnknownTextFormatTest(const std::string& test_name,
                                                const Message& message) {
  std::string serialized_input;
  message.SerializeToString(&serialized_input);
  MessageType prototype;
  ConformanceRequestSetting setting1(
      RECOMMENDED, conformance::PROTOBUF, conformance::TEXT_FORMAT,
      conformance::TEXT_FORMAT_TEST, prototype,
      absl::StrCat(test_name, "_Drop"), serialized_input);
  setting1.SetPrototypeMessageForCompare(message);
  suite_.RunValidBinaryInputTest(setting1, "");

  ConformanceRequestSetting setting2(
      RECOMMENDED, conformance::PROTOBUF, conformance::TEXT_FORMAT,
      conformance::TEXT_FORMAT_TEST, prototype,
      absl::StrCat(test_name, "_Print"), serialized_input);
  setting2.SetPrototypeMessageForCompare(message);
  setting2.SetPrintUnknownFields(true);
  suite_.RunValidBinaryInputTest(setting2, serialized_input);
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::RunGroupTests() {
  RunValidTextFormatTest("GroupFieldNoColon", REQUIRED,
                         "Data { group_int32: 1 }");
  RunValidTextFormatTest("GroupFieldWithColon", REQUIRED,
                         "Data: { group_int32: 1 }");
  RunValidTextFormatTest("GroupFieldEmpty", REQUIRED, "Data {}");
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::RunAllTests() {
  RunValidTextFormatTest("HelloWorld", REQUIRED,
                         "optional_string: 'Hello, World!'");
  // Integer fields.
  RunValidTextFormatTest("Int32FieldMaxValue", REQUIRED,
                         "optional_int32: 2147483647");
  RunValidTextFormatTest("Int32FieldMinValue", REQUIRED,
                         "optional_int32: -2147483648");
  RunValidTextFormatTest("Uint32FieldMaxValue", REQUIRED,
                         "optional_uint32: 4294967295");
  RunValidTextFormatTest("Int64FieldMaxValue", REQUIRED,
                         "optional_int64: 9223372036854775807");
  RunValidTextFormatTest("Int64FieldMinValue", REQUIRED,
                         "optional_int64: -9223372036854775808");
  RunValidTextFormatTest("Uint64FieldMaxValue", REQUIRED,
                         "optional_uint64: 18446744073709551615");

  // Parsers reject out-of-bound integer values.
  ExpectParseFailure("Int32FieldTooLarge", REQUIRED,
                     "optional_int32: 2147483648");
  ExpectParseFailure("Int32FieldTooSmall", REQUIRED,
                     "optional_int32: -2147483649");
  ExpectParseFailure("Uint32FieldTooLarge", REQUIRED,
                     "optional_uint32: 4294967296");
  ExpectParseFailure("Int64FieldTooLarge", REQUIRED,
                     "optional_int64: 9223372036854775808");
  ExpectParseFailure("Int64FieldTooSmall", REQUIRED,
                     "optional_int64: -9223372036854775809");
  ExpectParseFailure("Uint64FieldTooLarge", REQUIRED,
                     "optional_uint64: 18446744073709551616");

  // Floating point fields
  RunValidTextFormatTest("FloatField", REQUIRED, "optional_float: 3.192837");
  RunValidTextFormatTest("FloatFieldWithVeryPreciseNumber", REQUIRED,
                         "optional_float: 3.123456789123456789");
  RunValidTextFormatTest("FloatFieldMaxValue", REQUIRED,
                         "optional_float: 3.4028235e+38");
  RunValidTextFormatTest("FloatFieldMinValue", REQUIRED,
                         "optional_float: 1.17549e-38");
  RunValidTextFormatTest("FloatFieldNaNValue", REQUIRED, "optional_float: NaN");
  RunValidTextFormatTest("FloatFieldPosInfValue", REQUIRED,
                         "optional_float: inf");
  RunValidTextFormatTest("FloatFieldNegInfValue", REQUIRED,
                         "optional_float: -inf");
  RunValidTextFormatTest("FloatFieldWithInt32Max", REQUIRED,
                         "optional_float: 4294967296");
  RunValidTextFormatTest("FloatFieldLargerThanInt64", REQUIRED,
                         "optional_float: 9223372036854775808");
  RunValidTextFormatTest("FloatFieldTooLarge", REQUIRED,
                         "optional_float: 3.4028235e+39");
  RunValidTextFormatTest("FloatFieldTooSmall", REQUIRED,
                         "optional_float: 1.17549e-39");
  RunValidTextFormatTest("FloatFieldLargerThanUint64", REQUIRED,
                         "optional_float: 18446744073709551616");

  // String literals x {Strings, Bytes}
  for (const auto& field_type : std::vector<std::string>{"String", "Bytes"}) {
    const std::string field_name =
        field_type == "String" ? "optional_string" : "optional_bytes";
    RunValidTextFormatTest(
        absl::StrCat("StringLiteralConcat", field_type), REQUIRED,
        absl::StrCat(field_name, ": 'first' \"second\"\n'third'"));
    RunValidTextFormatTest(
        absl::StrCat("StringLiteralBasicEscapes", field_type), REQUIRED,
        absl::StrCat(field_name, ": '\\a\\b\\f\\n\\r\\t\\v\\?\\\\\\'\\\"'"));
    RunValidTextFormatTest(
        absl::StrCat("StringLiteralOctalEscapes", field_type), REQUIRED,
        absl::StrCat(field_name, ": '\\341\\210\\264'"));
    RunValidTextFormatTest(absl::StrCat("StringLiteralHexEscapes", field_type),
                           REQUIRED,
                           absl::StrCat(field_name, ": '\\xe1\\x88\\xb4'"));
    RunValidTextFormatTest(
        absl::StrCat("StringLiteralShortUnicodeEscape", field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\u1234'"));
    RunValidTextFormatTest(
        absl::StrCat("StringLiteralLongUnicodeEscapes", field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\U00001234\\U00010437'"));
    // String literals don't include line feeds.
    ExpectParseFailure(absl::StrCat("StringLiteralIncludesLF", field_type),
                       REQUIRED,
                       absl::StrCat(field_name, ": 'first line\nsecond line'"));
    // Unicode escapes don't include code points that lie beyond the planes
    // (> 0x10ffff).
    ExpectParseFailure(
        absl::StrCat("StringLiteralLongUnicodeEscapeTooLarge", field_type),
        REQUIRED, absl::StrCat(field_name, ": '\\U00110000'"));
    // Unicode escapes don't include surrogates.
    ExpectParseFailure(
        absl::StrCat("StringLiteralShortUnicodeEscapeSurrogatePair",
                     field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\ud801\\udc37'"));
    ExpectParseFailure(
        absl::StrCat("StringLiteralShortUnicodeEscapeSurrogateFirstOnly",
                     field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\ud800'"));
    ExpectParseFailure(
        absl::StrCat("StringLiteralShortUnicodeEscapeSurrogateSecondOnly",
                     field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\udc00'"));
    ExpectParseFailure(
        absl::StrCat("StringLiteralLongUnicodeEscapeSurrogateFirstOnly",
                     field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\U0000d800'"));
    ExpectParseFailure(
        absl::StrCat("StringLiteralLongUnicodeEscapeSurrogateSecondOnly",
                     field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\U0000dc00'"));
    ExpectParseFailure(
        absl::StrCat("StringLiteralLongUnicodeEscapeSurrogatePair", field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\U0000d801\\U00000dc37'"));
    ExpectParseFailure(
        absl::StrCat("StringLiteralUnicodeEscapeSurrogatePairLongShort",
                     field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\U0000d801\\udc37'"));
    ExpectParseFailure(
        absl::StrCat("StringLiteralUnicodeEscapeSurrogatePairShortLong",
                     field_type),
        RECOMMENDED, absl::StrCat(field_name, ": '\\ud801\\U0000dc37'"));

    // The following method depend on the type of field, as strings have extra
    // validation.
    const auto test_method =
        field_type == "String"
            ? &TextFormatConformanceTestSuiteImpl::ExpectParseFailure
            : &TextFormatConformanceTestSuiteImpl::RunValidTextFormatTest;

    // String fields reject invalid UTF-8 byte sequences; bytes fields don't.
    (this->*test_method)(absl::StrCat(field_type, "FieldBadUTF8Octal"),
                         REQUIRED, absl::StrCat(field_name, ": '\\300'"));
    (this->*test_method)(absl::StrCat(field_type, "FieldBadUTF8Hex"), REQUIRED,
                         absl::StrCat(field_name, ": '\\xc0'"));
  }

  // Unknown Fields
  UnknownToTestAllTypes message;
  // Unable to print unknown Fixed32/Fixed64 fields as if they are known.
  // Fixed32/Fixed64 fields are not added in the tests.
  message.set_optional_int32(123);
  message.set_optional_string("hello");
  message.set_optional_bool(true);
  RunValidUnknownTextFormatTest("ScalarUnknownFields", message);

  message.Clear();
  message.mutable_nested_message()->set_c(111);
  RunValidUnknownTextFormatTest("MessageUnknownFields", message);

  message.Clear();
  message.mutable_optionalgroup()->set_a(321);
  RunValidUnknownTextFormatTest("GroupUnknownFields", message);

  message.add_repeated_int32(1);
  message.add_repeated_int32(2);
  message.add_repeated_int32(3);
  RunValidUnknownTextFormatTest("RepeatedUnknownFields", message);

  // Map fields
  MessageType prototype;
  (*prototype.mutable_map_string_string())["c"] = "value";
  (*prototype.mutable_map_string_string())["b"] = "value";
  (*prototype.mutable_map_string_string())["a"] = "value";
  RunValidTextFormatTestWithMessage("AlphabeticallySortedMapStringKeys",
                                    REQUIRED,
                                    R"(
        map_string_string {
          key: "a"
          value: "value"
        }
        map_string_string {
          key: "b"
          value: "value"
        }
        map_string_string {
          key: "c"
          value: "value"
        }
        )",
                                    prototype);

  prototype.Clear();
  (*prototype.mutable_map_int32_int32())[3] = 0;
  (*prototype.mutable_map_int32_int32())[2] = 0;
  (*prototype.mutable_map_int32_int32())[1] = 0;
  RunValidTextFormatTestWithMessage("AlphabeticallySortedMapIntKeys", REQUIRED,
                                    R"(
        map_int32_int32 {
          key: 1
          value: 0
        }
        map_int32_int32 {
          key: 2
          value: 0
        }
        map_int32_int32 {
          key: 3
          value: 0
        }
        )",
                                    prototype);

  prototype.Clear();
  (*prototype.mutable_map_bool_bool())[true] = false;
  (*prototype.mutable_map_bool_bool())[false] = false;
  RunValidTextFormatTestWithMessage("AlphabeticallySortedMapBoolKeys", REQUIRED,
                                    R"(
        map_bool_bool {
          key: false
          value: false
        }
        map_bool_bool {
          key: true
          value: false
        }
        )",
                                    prototype);

  prototype.Clear();
  ConformanceRequestSetting setting_map(
      REQUIRED, conformance::TEXT_FORMAT, conformance::PROTOBUF,
      conformance::TEXT_FORMAT_TEST, prototype, "DuplicateMapKey", R"(
        map_string_nested_message {
          key: "duplicate"
          value: { a: 123 }
        }
        map_string_nested_message {
          key: "duplicate"
          value: { corecursive: {} }
        }
        )");
  // The last-specified value will be retained in a parsed map
  suite_.RunValidInputTest(setting_map, R"(
        map_string_nested_message {
          key: "duplicate"
          value: { corecursive: {} }
        }
        )");
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::RunAnyTests() {
  // Any fields
  RunValidTextFormatTest("AnyField", REQUIRED,
                         R"(
        optional_any: {
          [type.googleapis.com/protobuf_test_messages.proto3.TestAllTypesProto3]
  { optional_int32: 12345
          }
        }
        )");
  RunValidTextFormatTest("AnyFieldWithRawBytes", REQUIRED,
                         R"(
        optional_any: {
          type_url:
  "type.googleapis.com/protobuf_test_messages.proto3.TestAllTypesProto3" value:
  "\b\271`"
        }
        )");
  ExpectParseFailure("AnyFieldWithInvalidType", REQUIRED,
                     R"(
        optional_any: {
          [type.googleapis.com/unknown] {
            optional_int32: 12345
          }
        }
        )");
}

template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<
    MessageType>::RunTextFormatPerformanceTests() {
  TestTextFormatPerformanceMergeMessageWithRepeatedField("Bool",
                                                         "repeated_bool: true");
  TestTextFormatPerformanceMergeMessageWithRepeatedField(
      "Double", "repeated_double: 123");
  TestTextFormatPerformanceMergeMessageWithRepeatedField(
      "Int32", "repeated_uint32: 123");
  TestTextFormatPerformanceMergeMessageWithRepeatedField(
      "Int64", "repeated_uint64: 123");
  TestTextFormatPerformanceMergeMessageWithRepeatedField(
      "String", R"(repeated_string: "foo")");
  TestTextFormatPerformanceMergeMessageWithRepeatedField(
      "Bytes", R"(repeated_bytes: "foo")");
}

// This is currently considered valid input by some languages but not others
template <typename MessageType>
void TextFormatConformanceTestSuiteImpl<MessageType>::
    TestTextFormatPerformanceMergeMessageWithRepeatedField(
        const std::string& test_type_name, const std::string& message_field) {
  std::string recursive_message =
      absl::StrCat("recursive_message { ", message_field, " }");

  std::string input;
  for (size_t i = 0; i < kPerformanceRepeatCount; i++) {
    absl::StrAppend(&input, recursive_message);
  }

  std::string expected = "recursive_message { ";
  for (size_t i = 0; i < kPerformanceRepeatCount; i++) {
    absl::StrAppend(&expected, message_field, " ");
  }
  absl::StrAppend(&expected, "}");

  RunValidTextFormatTestWithExpected(
      absl::StrCat("TestTextFormatPerformanceMergeMessageWithRepeatedField",
                   test_type_name),
      RECOMMENDED, input, expected);
}

}  // namespace protobuf
}  // namespace google
