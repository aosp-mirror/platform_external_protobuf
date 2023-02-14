// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
// https://developers.google.com/protocol-buffers/
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

<<<<<<<< HEAD:src/google/protobuf/compiler/javamicro/javamicro_generator.h
// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// Generates Java micro code for a given .proto file.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MICRO_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MICRO_GENERATOR_H__

#include <string>
#include <google/protobuf/compiler/code_generator.h>

namespace google {
namespace protobuf {
namespace compiler {
namespace javamicro {

// CodeGenerator implementation which generates Java micro code.  If you create your
// own protocol compiler binary and you want it to support Java output for the
// micro runtime, you can do so by registering an instance of this CodeGenerator with
// the CommandLineInterface in your main() function.
class LIBPROTOC_EXPORT JavaMicroGenerator : public CodeGenerator {
 public:
  JavaMicroGenerator();
  ~JavaMicroGenerator();

  // implements CodeGenerator ----------------------------------------
  bool Generate(const FileDescriptor* file,
                const string& parameter,
                OutputDirectory* output_directory,
                string* error) const;

 private:
  GOOGLE_DISALLOW_EVIL_CONSTRUCTORS(JavaMicroGenerator);
};

}  // namespace javamicro
}  // namespace compiler
}  // namespace protobuf
}  // namespace google
<<<<<<< HEAD:src/google/protobuf/compiler/javamicro/javamicro_generator.h
#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MICRO_GENERATOR_H__
=======

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_MESSAGE_LAYOUT_HELPER_H__
>>>>>>> github/3.7.x:src/google/protobuf/compiler/cpp/cpp_message_layout_helper.h
========
#ifndef RUBY_PROTOBUF_REPEATED_FIELD_H_
#define RUBY_PROTOBUF_REPEATED_FIELD_H_

#include <ruby/ruby.h>

#include "protobuf.h"
#include "ruby-upb.h"

// Returns a Ruby wrapper object for the given upb_Array, which will be created
// if one does not exist already.
VALUE RepeatedField_GetRubyWrapper(upb_Array* msg, TypeInfo type_info,
                                   VALUE arena);

// Gets the underlying upb_Array for this Ruby RepeatedField object, which must
// have a type that matches |f|. If this is not a repeated field or the type
// doesn't match, raises an exception.
const upb_Array* RepeatedField_GetUpbArray(VALUE value, const upb_FieldDef* f,
                                           upb_Arena* arena);

// Implements #inspect for this repeated field by appending its contents to |b|.
void RepeatedField_Inspect(StringBuilder* b, const upb_Array* array,
                           TypeInfo info);

// Returns a deep copy of this RepeatedField object.
VALUE RepeatedField_deep_copy(VALUE obj);

// Ruby class of Google::Protobuf::RepeatedField.
extern VALUE cRepeatedField;

// Call at startup to register all types in this module.
void RepeatedField_register(VALUE module);

#endif  // RUBY_PROTOBUF_REPEATED_FIELD_H_
>>>>>>>> aosp/upstream-master:ruby/ext/google/protobuf_c/repeated_field.h
