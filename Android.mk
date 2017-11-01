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
