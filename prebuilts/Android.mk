# Copyright (C) 2014 The Android Open Source Project
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

# Prebuilts of the old 2.3.0 libprotobuf library.
# DEPRECATED: It is rare that you should need to depend on these libraries
# directly. Instead prefer setting LOCAL_PROTOC_OPTIMIZE_TYPE which will
# automatically pull in any dependent libraries, or using the versionless
# definitions of these libraries.

# Device Java prebuilts

include $(CLEAR_VARS)

LOCAL_PREBUILT_STATIC_JAVA_LIBRARIES := \
    libprotobuf-java-2.3.0-lite:prebuilts/libprotobuf-java-2.3.0-lite.jar \
    libprotobuf-java-2.3.0-micro:prebuilts/libprotobuf-java-2.3.0-micro.jar \
    libprotobuf-java-2.3.0-nano:prebuilts/libprotobuf-java-2.3.0-nano.jar

include $(BUILD_MULTI_PREBUILT)

# Host Java prebuilts

include $(CLEAR_VARS)

LOCAL_IS_HOST_MODULE := true

LOCAL_PREBUILT_JAVA_LIBRARIES := \
    host-libprotobuf-java-2.3.0-lite:prebuilts/host-libprotobuf-java-2.3.0-lite.jar \
    host-libprotobuf-java-2.3.0-micro:prebuilts/host-libprotobuf-java-2.3.0-micro.jar \
    host-libprotobuf-java-2.3.0-nano:prebuilts/host-libprotobuf-java-2.3.0-nano.jar

include $(BUILD_MULTI_PREBUILT)

# Device C++ static library prebuilts

include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-2.3.0-full
LOCAL_MODULE_CLASS := STATIC_LIBRARIES
LOCAL_MODULE_SUFFIX := .a
LOCAL_SRC_FILES_arm := prebuilts/arm/libprotobuf-cpp-2.3.0-full.a
LOCAL_SRC_FILES_arm64 := prebuilts/arm64/libprotobuf-cpp-2.3.0-full.a
LOCAL_SRC_FILES_mips := prebuilts/mips/libprotobuf-cpp-2.3.0-full.a
LOCAL_SRC_FILES_mips64 := prebuilts/mips64/libprotobuf-cpp-2.3.0-full.a
LOCAL_SRC_FILES_x86 := prebuilts/x86/libprotobuf-cpp-2.3.0-full.a
LOCAL_SRC_FILES_x86_64 := prebuilts/x86_64/libprotobuf-cpp-2.3.0-full.a
LOCAL_MODULE_TARGET_ARCH := arm arm64 mips mips64 x86 x86_64
LOCAL_MULTILIB := both

include $(BUILD_PREBUILT)

include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-2.3.0-full-gnustl-rtti
LOCAL_MODULE_CLASS := STATIC_LIBRARIES
LOCAL_MODULE_SUFFIX := .a
LOCAL_SRC_FILES_arm := prebuilts/arm/libprotobuf-cpp-2.3.0-full-gnustl-rtti.a
LOCAL_SRC_FILES_arm64 := prebuilts/arm64/libprotobuf-cpp-2.3.0-full-gnustl-rtti.a
LOCAL_SRC_FILES_mips := prebuilts/mips/libprotobuf-cpp-2.3.0-full-gnustl-rtti.a
LOCAL_SRC_FILES_mips64 := prebuilts/mips64/libprotobuf-cpp-2.3.0-full-gnustl-rtti.a
LOCAL_SRC_FILES_x86 := prebuilts/x86/libprotobuf-cpp-2.3.0-full-gnustl-rtti.a
LOCAL_SRC_FILES_x86_64 := prebuilts/x86_64/libprotobuf-cpp-2.3.0-full-gnustl-rtti.a
LOCAL_MODULE_TARGET_ARCH := arm arm64 mips mips64 x86 x86_64
LOCAL_MULTILIB := both

include $(BUILD_PREBUILT)

include $(CLEAR_VARS)

LOCAL_MODULE := libprotobuf-cpp-2.3.0-lite
LOCAL_MODULE_CLASS := STATIC_LIBRARIES
LOCAL_MODULE_SUFFIX := .a
LOCAL_SRC_FILES_arm := prebuilts/arm/libprotobuf-cpp-2.3.0-lite.a
LOCAL_SRC_FILES_arm64 := prebuilts/arm64/libprotobuf-cpp-2.3.0-lite.a
LOCAL_SRC_FILES_mips := prebuilts/mips/libprotobuf-cpp-2.3.0-lite.a
LOCAL_SRC_FILES_mips64 := prebuilts/mips64/libprotobuf-cpp-2.3.0-lite.a
LOCAL_SRC_FILES_x86 := prebuilts/x86/libprotobuf-cpp-2.3.0-lite.a
LOCAL_SRC_FILES_x86_64 := prebuilts/x86_64/libprotobuf-cpp-2.3.0-lite.a
LOCAL_MODULE_TARGET_ARCH := arm arm64 mips mips64 x86 x86_64
LOCAL_MULTILIB := both

include $(BUILD_PREBUILT)

