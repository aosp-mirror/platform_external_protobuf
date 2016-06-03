# Copyright (C) 2009 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
#

LOCAL_PATH := $(call my-dir)

IGNORED_WARNINGS := -Wno-sign-compare -Wno-unused-parameter -Wno-sign-promo -Wno-error=return-type

CC_LITE_SRC_FILES := \
    src/google/protobuf/stubs/atomicops_internals_x86_gcc.cc \
    src/google/protobuf/stubs/atomicops_internals_x86_msvc.cc \
    src/google/protobuf/stubs/bytestream.cc \
    src/google/protobuf/stubs/common.cc \
    src/google/protobuf/stubs/int128.cc \
    src/google/protobuf/stubs/once.cc \
    src/google/protobuf/stubs/status.cc \
    src/google/protobuf/stubs/statusor.cc \
    src/google/protobuf/stubs/stringpiece.cc \
    src/google/protobuf/stubs/stringprintf.cc \
    src/google/protobuf/stubs/structurally_valid.cc \
    src/google/protobuf/stubs/strutil.cc \
    src/google/protobuf/stubs/time.cc \
    src/google/protobuf/arena.cc \
    src/google/protobuf/arenastring.cc \
    src/google/protobuf/extension_set.cc \
    src/google/protobuf/generated_message_util.cc \
    src/google/protobuf/message_lite.cc \
    src/google/protobuf/repeated_field.cc \
    src/google/protobuf/wire_format_lite.cc \
    src/google/protobuf/io/coded_stream.cc \
    src/google/protobuf/io/zero_copy_stream.cc \
    src/google/protobuf/io/zero_copy_stream_impl_lite.cc

JAVA_LITE_SRC_FILES := \
    java/core/src/main/java/com/google/protobuf/AbstractMessageLite.java \
    java/core/src/main/java/com/google/protobuf/AbstractParser.java \
    java/core/src/main/java/com/google/protobuf/AbstractProtobufList.java \
    java/core/src/main/java/com/google/protobuf/ByteOutput.java \
    java/core/src/main/java/com/google/protobuf/ByteString.java \
    java/core/src/main/java/com/google/protobuf/BooleanArrayList.java \
    java/core/src/main/java/com/google/protobuf/CodedInputStream.java \
    java/core/src/main/java/com/google/protobuf/CodedOutputStream.java \
    java/core/src/main/java/com/google/protobuf/DoubleArrayList.java \
    java/core/src/main/java/com/google/protobuf/ExperimentalApi.java \
    java/core/src/main/java/com/google/protobuf/ExtensionLite.java \
    java/core/src/main/java/com/google/protobuf/ExtensionRegistryLite.java \
    java/core/src/main/java/com/google/protobuf/FieldSet.java \
    java/core/src/main/java/com/google/protobuf/FloatArrayList.java \
    java/core/src/main/java/com/google/protobuf/GeneratedMessageLite.java \
    java/core/src/main/java/com/google/protobuf/IntArrayList.java \
    java/core/src/main/java/com/google/protobuf/Internal.java \
    java/core/src/main/java/com/google/protobuf/InvalidProtocolBufferException.java \
    java/core/src/main/java/com/google/protobuf/LazyField.java \
    java/core/src/main/java/com/google/protobuf/LazyFieldLite.java \
    java/core/src/main/java/com/google/protobuf/LazyStringArrayList.java \
    java/core/src/main/java/com/google/protobuf/LazyStringList.java \
    java/core/src/main/java/com/google/protobuf/LongArrayList.java \
    java/core/src/main/java/com/google/protobuf/MapFieldLite.java \
    java/core/src/main/java/com/google/protobuf/MessageLite.java \
    java/core/src/main/java/com/google/protobuf/MessageLiteToString.java \
    java/core/src/main/java/com/google/protobuf/MessageLiteOrBuilder.java \
    java/core/src/main/java/com/google/protobuf/MutabilityOracle.java \
    java/core/src/main/java/com/google/protobuf/Parser.java \
    java/core/src/main/java/com/google/protobuf/ProtobufArrayList.java \
    java/core/src/main/java/com/google/protobuf/ProtocolStringList.java \
    java/core/src/main/java/com/google/protobuf/RopeByteString.java \
    java/core/src/main/java/com/google/protobuf/SmallSortedMap.java \
    java/core/src/main/java/com/google/protobuf/TextFormatEscaper.java \
    java/core/src/main/java/com/google/protobuf/UninitializedMessageException.java \
    java/core/src/main/java/com/google/protobuf/UnknownFieldSetLite.java \
    java/core/src/main/java/com/google/protobuf/UnmodifiableLazyStringList.java \
    java/core/src/main/java/com/google/protobuf/Utf8.java \
    java/core/src/main/java/com/google/protobuf/WireFormat.java

# This contains more source files than needed for the full version, but the
# additional files should not create any conflict.
JAVA_FULL_SRC_FILES := \
    $(call all-java-files-under, java/core/src/main/java) \
    src/google/protobuf/descriptor.proto

COMPILER_SRC_FILES :=  \
    src/google/protobuf/any.cc \
    src/google/protobuf/arena.cc \
    src/google/protobuf/arenastring.cc \
    src/google/protobuf/descriptor.cc \
    src/google/protobuf/descriptor.pb.cc \
    src/google/protobuf/descriptor_database.cc \
    src/google/protobuf/dynamic_message.cc \
    src/google/protobuf/extension_set.cc \
    src/google/protobuf/extension_set_heavy.cc \
    src/google/protobuf/generated_message_reflection.cc \
    src/google/protobuf/generated_message_util.cc \
    src/google/protobuf/map_field.cc \
    src/google/protobuf/message.cc \
    src/google/protobuf/message_lite.cc \
    src/google/protobuf/reflection_ops.cc \
    src/google/protobuf/repeated_field.cc \
    src/google/protobuf/service.cc \
    src/google/protobuf/text_format.cc \
    src/google/protobuf/unknown_field_set.cc \
    src/google/protobuf/wire_format.cc \
    src/google/protobuf/wire_format_lite.cc \
    src/google/protobuf/compiler/code_generator.cc \
    src/google/protobuf/compiler/command_line_interface.cc \
    src/google/protobuf/compiler/cpp/cpp_enum.cc \
    src/google/protobuf/compiler/cpp/cpp_enum_field.cc \
    src/google/protobuf/compiler/cpp/cpp_extension.cc \
    src/google/protobuf/compiler/cpp/cpp_field.cc \
    src/google/protobuf/compiler/cpp/cpp_file.cc \
    src/google/protobuf/compiler/cpp/cpp_generator.cc \
    src/google/protobuf/compiler/cpp/cpp_helpers.cc \
    src/google/protobuf/compiler/cpp/cpp_map_field.cc \
    src/google/protobuf/compiler/cpp/cpp_message.cc \
    src/google/protobuf/compiler/cpp/cpp_message_field.cc \
    src/google/protobuf/compiler/cpp/cpp_primitive_field.cc \
    src/google/protobuf/compiler/cpp/cpp_service.cc \
    src/google/protobuf/compiler/cpp/cpp_string_field.cc \
    src/google/protobuf/compiler/csharp/csharp_doc_comment.cc \
    src/google/protobuf/compiler/csharp/csharp_enum.cc \
    src/google/protobuf/compiler/csharp/csharp_enum_field.cc \
    src/google/protobuf/compiler/csharp/csharp_field_base.cc \
    src/google/protobuf/compiler/csharp/csharp_generator.cc \
    src/google/protobuf/compiler/csharp/csharp_helpers.cc \
    src/google/protobuf/compiler/csharp/csharp_map_field.cc \
    src/google/protobuf/compiler/csharp/csharp_message.cc \
    src/google/protobuf/compiler/csharp/csharp_message_field.cc \
    src/google/protobuf/compiler/csharp/csharp_primitive_field.cc \
    src/google/protobuf/compiler/csharp/csharp_reflection_class.cc \
    src/google/protobuf/compiler/csharp/csharp_repeated_enum_field.cc \
    src/google/protobuf/compiler/csharp/csharp_repeated_message_field.cc \
    src/google/protobuf/compiler/csharp/csharp_repeated_primitive_field.cc \
    src/google/protobuf/compiler/csharp/csharp_source_generator_base.cc \
    src/google/protobuf/compiler/csharp/csharp_wrapper_field.cc \
    src/google/protobuf/compiler/importer.cc \
    src/google/protobuf/compiler/java/java_context.cc \
    src/google/protobuf/compiler/java/java_doc_comment.cc \
    src/google/protobuf/compiler/java/java_enum.cc \
    src/google/protobuf/compiler/java/java_enum_field.cc \
    src/google/protobuf/compiler/java/java_enum_field_lite.cc \
    src/google/protobuf/compiler/java/java_enum_lite.cc \
    src/google/protobuf/compiler/java/java_extension.cc \
    src/google/protobuf/compiler/java/java_extension_lite.cc \
    src/google/protobuf/compiler/java/java_field.cc \
    src/google/protobuf/compiler/java/java_file.cc \
    src/google/protobuf/compiler/java/java_generator.cc \
    src/google/protobuf/compiler/java/java_generator_factory.cc \
    src/google/protobuf/compiler/java/java_helpers.cc \
    src/google/protobuf/compiler/java/java_lazy_message_field.cc \
    src/google/protobuf/compiler/java/java_lazy_message_field_lite.cc \
    src/google/protobuf/compiler/java/java_map_field.cc \
    src/google/protobuf/compiler/java/java_map_field_lite.cc \
    src/google/protobuf/compiler/java/java_message_builder.cc \
    src/google/protobuf/compiler/java/java_message_builder_lite.cc \
    src/google/protobuf/compiler/java/java_message.cc \
    src/google/protobuf/compiler/java/java_message_field.cc \
    src/google/protobuf/compiler/java/java_message_field_lite.cc \
    src/google/protobuf/compiler/java/java_message_lite.cc \
    src/google/protobuf/compiler/java/java_name_resolver.cc \
    src/google/protobuf/compiler/java/java_primitive_field.cc \
    src/google/protobuf/compiler/java/java_primitive_field_lite.cc \
    src/google/protobuf/compiler/java/java_service.cc \
    src/google/protobuf/compiler/java/java_shared_code_generator.cc \
    src/google/protobuf/compiler/java/java_string_field.cc \
    src/google/protobuf/compiler/java/java_string_field_lite.cc \
    src/google/protobuf/compiler/javamicro/javamicro_enum.cc \
    src/google/protobuf/compiler/javamicro/javamicro_enum_field.cc \
    src/google/protobuf/compiler/javamicro/javamicro_field.cc \
    src/google/protobuf/compiler/javamicro/javamicro_file.cc \
    src/google/protobuf/compiler/javamicro/javamicro_generator.cc \
    src/google/protobuf/compiler/javamicro/javamicro_helpers.cc \
    src/google/protobuf/compiler/javamicro/javamicro_message.cc \
    src/google/protobuf/compiler/javamicro/javamicro_message_field.cc \
    src/google/protobuf/compiler/javamicro/javamicro_primitive_field.cc \
    src/google/protobuf/compiler/javanano/javanano_enum.cc \
    src/google/protobuf/compiler/javanano/javanano_enum_field.cc \
    src/google/protobuf/compiler/javanano/javanano_extension.cc \
    src/google/protobuf/compiler/javanano/javanano_field.cc \
    src/google/protobuf/compiler/javanano/javanano_file.cc \
    src/google/protobuf/compiler/javanano/javanano_generator.cc \
    src/google/protobuf/compiler/javanano/javanano_helpers.cc \
    src/google/protobuf/compiler/javanano/javanano_map_field.cc \
    src/google/protobuf/compiler/javanano/javanano_message.cc \
    src/google/protobuf/compiler/javanano/javanano_message_field.cc \
    src/google/protobuf/compiler/javanano/javanano_primitive_field.cc \
    src/google/protobuf/compiler/js/js_generator.cc \
    src/google/protobuf/compiler/main.cc \
    src/google/protobuf/compiler/objectivec/objectivec_enum.cc \
    src/google/protobuf/compiler/objectivec/objectivec_enum_field.cc \
    src/google/protobuf/compiler/objectivec/objectivec_extension.cc \
    src/google/protobuf/compiler/objectivec/objectivec_field.cc \
    src/google/protobuf/compiler/objectivec/objectivec_file.cc \
    src/google/protobuf/compiler/objectivec/objectivec_generator.cc \
    src/google/protobuf/compiler/objectivec/objectivec_helpers.cc \
    src/google/protobuf/compiler/objectivec/objectivec_map_field.cc \
    src/google/protobuf/compiler/objectivec/objectivec_message.cc \
    src/google/protobuf/compiler/objectivec/objectivec_message_field.cc \
    src/google/protobuf/compiler/objectivec/objectivec_oneof.cc \
    src/google/protobuf/compiler/objectivec/objectivec_primitive_field.cc \
    src/google/protobuf/compiler/parser.cc \
    src/google/protobuf/compiler/plugin.cc \
    src/google/protobuf/compiler/plugin.pb.cc \
    src/google/protobuf/compiler/python/python_generator.cc \
    src/google/protobuf/compiler/ruby/ruby_generator.cc \
    src/google/protobuf/compiler/subprocess.cc \
    src/google/protobuf/compiler/zip_writer.cc \
    src/google/protobuf/io/coded_stream.cc \
    src/google/protobuf/io/gzip_stream.cc \
    src/google/protobuf/io/printer.cc \
    src/google/protobuf/io/strtod.cc \
    src/google/protobuf/io/tokenizer.cc \
    src/google/protobuf/io/zero_copy_stream.cc \
    src/google/protobuf/io/zero_copy_stream_impl.cc \
    src/google/protobuf/io/zero_copy_stream_impl_lite.cc \
    src/google/protobuf/stubs/atomicops_internals_x86_gcc.cc \
    src/google/protobuf/stubs/atomicops_internals_x86_msvc.cc \
    src/google/protobuf/stubs/common.cc \
    src/google/protobuf/stubs/int128.cc \
    src/google/protobuf/stubs/once.cc \
    src/google/protobuf/stubs/status.cc \
    src/google/protobuf/stubs/stringpiece.cc \
    src/google/protobuf/stubs/structurally_valid.cc \
    src/google/protobuf/stubs/strutil.cc \
    src/google/protobuf/stubs/substitute.cc \
    src/google/protobuf/stubs/stringprintf.cc

# Java nano library (for device-side users)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-java-nano
LOCAL_MODULE_TAGS := optional
LOCAL_SDK_VERSION := 9

LOCAL_SRC_FILES := $(call all-java-files-under, javanano/src/main/java/com/google/protobuf/nano)
LOCAL_SRC_FILES += $(call all-java-files-under, javanano/src/device/main/java/com/google/protobuf/nano)

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_STATIC_JAVA_LIBRARY)

# Java nano library (for host-side users)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := host-libprotobuf-java-nano
LOCAL_MODULE_TAGS := optional

LOCAL_SRC_FILES := $(call all-java-files-under, javanano/src/main/java/com/google/protobuf/nano)

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_HOST_JAVA_LIBRARY)

# Java micro library (for device-side users)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-java-micro
LOCAL_MODULE_TAGS := optional
LOCAL_SDK_VERSION := 8

LOCAL_SRC_FILES := $(call all-java-files-under, javamicro/src/main/java/com/google/protobuf/micro)

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_STATIC_JAVA_LIBRARY)

# Java micro library (for host-side users)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := host-libprotobuf-java-micro
LOCAL_MODULE_TAGS := optional

LOCAL_SRC_FILES := $(call all-java-files-under, javamicro/src/main/java/com/google/protobuf/micro)

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_HOST_JAVA_LIBRARY)

# Java lite library (for device-side users)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-java-lite
LOCAL_MODULE_TAGS := optional

LOCAL_SRC_FILES := $(JAVA_LITE_SRC_FILES)

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_STATIC_JAVA_LIBRARY)

# Java lite library (for host-side users)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := host-libprotobuf-java-lite
LOCAL_MODULE_TAGS := optional

LOCAL_SRC_FILES := $(JAVA_LITE_SRC_FILES)

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_HOST_JAVA_LIBRARY)

# Java full library (for host-side users)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := host-libprotobuf-java-full
LOCAL_MODULE_TAGS := optional

LOCAL_SRC_FILES := $(JAVA_FULL_SRC_FILES)

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_HOST_JAVA_LIBRARY)

# C++ lite library for the NDK.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-lite-ndk

LOCAL_CPP_EXTENSION := .cc

LOCAL_SRC_FILES := $(CC_LITE_SRC_FILES)

LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)

# These are the minimum versions and don't need to be updated.
ifeq ($(TARGET_ARCH),arm)
LOCAL_SDK_VERSION := 8
else
# x86/mips support only available from API 9.
LOCAL_SDK_VERSION := 9
endif
LOCAL_NDK_STL_VARIANT := stlport_static

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_STATIC_LIBRARY)

# C++ lite library for the platform.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-lite

LOCAL_CPP_EXTENSION := .cc

LOCAL_SRC_FILES := $(CC_LITE_SRC_FILES)

LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_SHARED_LIBRARY)

# C++ lite static library for host tools.
include $(CLEAR_VARS)
LOCAL_MODULE := libprotobuf-cpp-lite
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(CC_LITE_SRC_FILES)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)
LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src
include $(BUILD_STATIC_LIBRARY)

# C++ lite library for the host.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-lite_static

LOCAL_MODULE_HOST_OS := darwin linux windows

LOCAL_CPP_EXTENSION := .cc

LOCAL_SRC_FILES := $(CC_LITE_SRC_FILES)

LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_HOST_STATIC_LIBRARY)

# C++ lite library for the host.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-lite

LOCAL_MODULE_HOST_OS := darwin linux windows

LOCAL_WHOLE_STATIC_LIBRARIES := libprotobuf-cpp-lite_static

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_HOST_SHARED_LIBRARY)

include $(CLEAR_VARS)
LOCAL_MODULE := libprotobuf-cpp-lite
LOCAL_MODULE_HOST_OS := darwin linux windows
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(CC_LITE_SRC_FILES)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)
LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src
include $(BUILD_HOST_STATIC_LIBRARY)

# C++ lite library + rtti (libc++ flavored for the platform)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-lite-rtti
LOCAL_MODULE_TAGS := optional

LOCAL_CPP_EXTENSION := .cc

LOCAL_SRC_FILES := $(CC_LITE_SRC_FILES)

LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_RTTI_FLAG := -frtti
LOCAL_CFLAGS := $(IGNORED_WARNINGS)

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_SHARED_LIBRARY)

# C++ lite library + rtti (libc++ flavored for the host)
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-lite-rtti
LOCAL_MODULE_TAGS := optional

LOCAL_CPP_EXTENSION := .cc

LOCAL_SRC_FILES := $(CC_LITE_SRC_FILES)

LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_RTTI_FLAG := -frtti
LOCAL_CFLAGS := $(IGNORED_WARNINGS)

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_HOST_SHARED_LIBRARY)

# C++ full library
# =======================================================
protobuf_cc_full_src_files := \
    $(CC_LITE_SRC_FILES) \
    src/google/protobuf/any.pb.cc \
    src/google/protobuf/api.pb.cc \
    src/google/protobuf/stubs/mathlimits.cc \
    src/google/protobuf/any.cc \
    src/google/protobuf/descriptor.cc \
    src/google/protobuf/descriptor_database.cc \
    src/google/protobuf/descriptor.pb.cc \
    src/google/protobuf/duration.pb.cc \
    src/google/protobuf/dynamic_message.cc \
    src/google/protobuf/empty.pb.cc \
    src/google/protobuf/extension_set_heavy.cc \
    src/google/protobuf/field_mask.pb.cc \
    src/google/protobuf/generated_message_reflection.cc \
    src/google/protobuf/map_field.cc \
    src/google/protobuf/message.cc \
    src/google/protobuf/reflection_ops.cc \
    src/google/protobuf/service.cc \
    src/google/protobuf/source_context.pb.cc \
    src/google/protobuf/struct.pb.cc \
    src/google/protobuf/stubs/substitute.cc \
    src/google/protobuf/text_format.cc \
    src/google/protobuf/timestamp.pb.cc \
    src/google/protobuf/type.pb.cc \
    src/google/protobuf/unknown_field_set.cc \
    src/google/protobuf/wire_format.cc \
    src/google/protobuf/wrappers.pb.cc \
    src/google/protobuf/io/gzip_stream.cc \
    src/google/protobuf/io/printer.cc \
    src/google/protobuf/io/strtod.cc \
    src/google/protobuf/io/tokenizer.cc \
    src/google/protobuf/io/zero_copy_stream_impl.cc \
    src/google/protobuf/compiler/importer.cc \
    src/google/protobuf/compiler/parser.cc \
    src/google/protobuf/util/field_comparator.cc \
    src/google/protobuf/util/field_mask_util.cc \
    src/google/protobuf/util/internal/datapiece.cc \
    src/google/protobuf/util/internal/default_value_objectwriter.cc \
    src/google/protobuf/util/internal/error_listener.cc \
    src/google/protobuf/util/internal/field_mask_utility.cc \
    src/google/protobuf/util/internal/json_escaping.cc \
    src/google/protobuf/util/internal/json_objectwriter.cc \
    src/google/protobuf/util/internal/json_stream_parser.cc \
    src/google/protobuf/util/internal/object_writer.cc \
    src/google/protobuf/util/internal/protostream_objectsource.cc \
    src/google/protobuf/util/internal/protostream_objectwriter.cc \
    src/google/protobuf/util/internal/proto_writer.cc \
    src/google/protobuf/util/internal/type_info.cc \
    src/google/protobuf/util/internal/type_info_test_helper.cc \
    src/google/protobuf/util/internal/utility.cc \
    src/google/protobuf/util/json_util.cc \
    src/google/protobuf/util/message_differencer.cc \
    src/google/protobuf/util/time_util.cc \
    src/google/protobuf/util/type_resolver_util.cc

# C++ full library for the NDK.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-full-ndk
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(protobuf_cc_full_src_files)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    external/zlib \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)

# These are the minimum versions and don't need to be updated.
ifeq ($(TARGET_ARCH),arm)
LOCAL_SDK_VERSION := 8
else
# x86/mips support only available from API 9.
LOCAL_SDK_VERSION := 9
endif
LOCAL_NDK_STL_VARIANT := stlport_static

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_STATIC_LIBRARY)

# C++ full library for the NDK, Gnustl+rtti version.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-full-ndk-gnustl-rtti
LOCAL_MODULE_TAGS := optional
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(protobuf_cc_full_src_files)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    external/zlib \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -frtti $(IGNORED_WARNINGS)
LOCAL_SDK_VERSION := 14
LOCAL_NDK_STL_VARIANT := gnustl_static

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_STATIC_LIBRARY)

# C++ full library for the platform.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-full
LOCAL_MODULE_TAGS := optional
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(protobuf_cc_full_src_files)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)
LOCAL_SHARED_LIBRARIES := libz

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_SHARED_LIBRARY)

# C++ full library for the host
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-full
LOCAL_MODULE_TAGS := optional
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(protobuf_cc_full_src_files)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_CFLAGS := -DGOOGLE_PROTOBUF_NO_RTTI $(IGNORED_WARNINGS)
LOCAL_SHARED_LIBRARIES := libz-host

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_HOST_SHARED_LIBRARY)

# C++ full library + rtti for the platform.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-full-rtti
LOCAL_MODULE_TAGS := optional
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(protobuf_cc_full_src_files)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_RTTI_FLAG := -frtti
LOCAL_CFLAGS := $(IGNORED_WARNINGS)
LOCAL_SHARED_LIBRARIES := libz

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_SHARED_LIBRARY)

# C++ full library + rtti for the host.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-full-rtti
LOCAL_MODULE_TAGS := optional
LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(protobuf_cc_full_src_files)
LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_RTTI_FLAG := -frtti
LOCAL_CFLAGS := $(IGNORED_WARNINGS)
LOCAL_SHARED_LIBRARIES := libz-host

LOCAL_EXPORT_C_INCLUDE_DIRS := $(LOCAL_PATH)/src

include $(BUILD_HOST_SHARED_LIBRARY)

# Clean temp vars
protobuf_cc_full_src_files :=


# Android Protocol buffer compiler, aprotoc (host executable)
# used by the build systems as $(PROTOC) defined in
# build/core/config.mk
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := aprotoc
LOCAL_MODULE_HOST_OS := darwin linux windows

# Statically link libc++ because we copy aprotoc to unbundled projects where
# libc++.so may not be available.
LOCAL_CXX_STL := libc++_static

LOCAL_CPP_EXTENSION := .cc
LOCAL_SRC_FILES := $(COMPILER_SRC_FILES)

LOCAL_C_INCLUDES := \
    $(LOCAL_PATH)/android \
    $(LOCAL_PATH)/src

LOCAL_STATIC_LIBRARIES += libz

LOCAL_LDLIBS_darwin := -lpthread
LOCAL_LDLIBS_linux := -lpthread

LOCAL_CFLAGS := $(IGNORED_WARNINGS)

include $(BUILD_HOST_EXECUTABLE)

# To test java proto params build rules.
# =======================================================
include $(CLEAR_VARS)

LOCAL_MODULE := aprotoc-test-nano-params
LOCAL_MODULE_TAGS := tests
LOCAL_SDK_VERSION := current

LOCAL_PROTOC_OPTIMIZE_TYPE := nano

LOCAL_SRC_FILES := \
        javanano/src/test/java/com/google/protobuf/nano/unittest_import_nano.proto \
        javanano/src/test/java/com/google/protobuf/nano/unittest_simple_nano.proto \
        javanano/src/test/java/com/google/protobuf/nano/unittest_stringutf8_nano.proto \
        javanano/src/test/java/com/google/protobuf/nano/unittest_recursive_nano.proto


LOCAL_PROTOC_FLAGS := --proto_path=$(LOCAL_PATH)/src

LOCAL_PROTO_JAVA_OUTPUT_PARAMS := \
        java_package = $(LOCAL_PATH)javanano/src/test/java/com/google/protobuf/nano/unittest_import_nano.proto|com.google.protobuf.nano, \
        java_outer_classname = $(LOCAL_PATH)/javanano/src/test/java/com/google/protobuf/nano/unittest_import_nano.proto|UnittestImportNano

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_STATIC_JAVA_LIBRARY)

# To test Android-specific nanoproto features.
# =======================================================
include $(CLEAR_VARS)

# Parcelable messages
LOCAL_MODULE := android-nano-test-parcelable
LOCAL_MODULE_TAGS := tests
LOCAL_SDK_VERSION := current
# Only needed at compile-time.
LOCAL_JAVA_LIBRARIES := android-support-annotations

LOCAL_PROTOC_OPTIMIZE_TYPE := nano

LOCAL_SRC_FILES := javanano/src/test/java/com/google/protobuf/nano/unittest_simple_nano.proto

LOCAL_PROTOC_FLAGS := --proto_path=$(LOCAL_PATH)/src

LOCAL_PROTO_JAVA_OUTPUT_PARAMS := \
        parcelable_messages = true, \
        generate_intdefs = true

include $(BUILD_STATIC_JAVA_LIBRARY)

include $(CLEAR_VARS)

# Parcelable and extendable messages
LOCAL_MODULE := android-nano-test-parcelable-extendable
LOCAL_MODULE_TAGS := tests
LOCAL_SDK_VERSION := current
# Only needed at compile-time.
LOCAL_JAVA_LIBRARIES := android-support-annotations

LOCAL_PROTOC_OPTIMIZE_TYPE := nano

LOCAL_SRC_FILES := javanano/src/test/java/com/google/protobuf/nano/unittest_extension_nano.proto

LOCAL_PROTOC_FLAGS := --proto_path=$(LOCAL_PATH)/src

LOCAL_PROTO_JAVA_OUTPUT_PARAMS := \
        parcelable_messages = true, \
        generate_intdefs = true, \
        store_unknown_fields = true

LOCAL_JAVA_LANGUAGE_VERSION := 1.7
include $(BUILD_STATIC_JAVA_LIBRARY)

include $(CLEAR_VARS)

# Test APK
LOCAL_PACKAGE_NAME := NanoAndroidTest

LOCAL_SDK_VERSION := 8

LOCAL_MODULE_TAGS := tests

LOCAL_SRC_FILES := $(call all-java-files-under, javanano/src/device/test/java/com/google/protobuf/nano)

LOCAL_MANIFEST_FILE := javanano/src/device/test/AndroidManifest.xml

LOCAL_STATIC_JAVA_LIBRARIES := libprotobuf-java-nano \
        android-nano-test-parcelable \
        android-nano-test-parcelable-extendable

LOCAL_DEX_PREOPT := false

include $(BUILD_PACKAGE)
