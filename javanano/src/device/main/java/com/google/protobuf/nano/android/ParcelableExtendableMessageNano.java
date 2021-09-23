// Protocol Buffers - Google's data interchange format
<<<<<<<< HEAD:javanano/src/device/main/java/com/google/protobuf/nano/android/ParcelableExtendableMessageNano.java
// Copyright 2014 Google Inc.  All rights reserved.
// http://code.google.com/p/protobuf/
========
// Copyright 2008 Google Inc.  All rights reserved.
// https://developers.google.com/protocol-buffers/
>>>>>>>> aosp/upstream-master:php/ext/google/protobuf/arena.h
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

<<<<<<<< HEAD:javanano/src/device/main/java/com/google/protobuf/nano/android/ParcelableExtendableMessageNano.java
package com.google.protobuf.nano.android;

import android.os.Parcel;
import android.os.Parcelable;

import com.google.protobuf.nano.ExtendableMessageNano;

/**
 * Base class for Parcelable Protocol Buffer messages which also need to store unknown
 * fields, such as extensions.
 */
public abstract class ParcelableExtendableMessageNano<M extends ExtendableMessageNano<M>>
        extends ExtendableMessageNano<M> implements Parcelable {

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
#ifndef PHP_PROTOBUF_ARENA_H_
#define PHP_PROTOBUF_ARENA_H_

#include <php.h>

#include "php-upb.h"

// Registers the PHP Arena class.
void Arena_ModuleInit();

// Creates and returns a new arena object that wraps a new upb_arena*.
void Arena_Init(zval *val);

// Gets the underlying upb_arena from this arena object.
upb_arena *Arena_Get(zval *arena);

#endif  // PHP_PROTOBUF_ARENA_H_
>>>>>>>> aosp/upstream-master:php/ext/google/protobuf/arena.h
