// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

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

// Recursively freeze RepeatedField.
VALUE RepeatedField_freeze(VALUE _self);

#endif  // RUBY_PROTOBUF_REPEATED_FIELD_H_
>>>>>>>> aosp/upstream-master:ruby/ext/google/protobuf_c/repeated_field.h
