// Protocol Buffers - Google's data interchange format
<<<<<<<< HEAD:javanano/src/device/main/java/com/google/protobuf/nano/android/ParcelableMessageNano.java
// Copyright 2014 Google Inc.  All rights reserved.
// http://code.google.com/p/protobuf/
========
// Copyright 2008 Google Inc.  All rights reserved.
// https://developers.google.com/protocol-buffers/
>>>>>>>> aosp/upstream-master:php/ext/google/protobuf/names.h
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

<<<<<<<< HEAD:javanano/src/device/main/java/com/google/protobuf/nano/android/ParcelableMessageNano.java
package com.google.protobuf.nano.android;

import android.os.Parcel;
import android.os.Parcelable;

import com.google.protobuf.nano.MessageNano;

/**
 * Base class for Parcelable Protocol Buffer messages.
 */
public abstract class ParcelableMessageNano extends MessageNano implements Parcelable {

    @Override
    public int describeContents() {
        return 0;
    }

    @Override
    public void writeToParcel(Parcel out, int flags) {
        ParcelableMessageNanoCreator.writeToParcel(getClass(), this, out);
    }
}
========
#ifndef PHP_PROTOBUF_NAMES_H_
#define PHP_PROTOBUF_NAMES_H_

#include "php-upb.h"

// Translates a protobuf symbol name (eg. foo.bar.Baz) into a PHP class name
// (eg. \Foo\Bar\Baz).
char *GetPhpClassname(const upb_filedef *file, const char *fullname);

#endif  // PHP_PROTOBUF_NAMES_H_
>>>>>>>> aosp/upstream-master:php/ext/google/protobuf/names.h
