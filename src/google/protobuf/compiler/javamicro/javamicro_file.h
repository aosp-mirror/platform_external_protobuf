// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
// http://code.google.com/p/protobuf/
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

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_FILE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_FILE_H__

#include <string>
#include <vector>
#include <google/protobuf/stubs/common.h>
#include <google/protobuf/compiler/code_generator.h>
#include <google/protobuf/compiler/javamicro/javamicro_params.h>

#include <google/protobuf/port_def.inc>

namespace google {
namespace protobuf {
  class FileDescriptor;        // descriptor.h
  namespace io {
    class Printer;             // printer.h
  }
}

namespace protobuf {
namespace compiler {
namespace javamicro {

<<<<<<< HEAD:src/google/protobuf/compiler/javamicro/javamicro_file.h
class FileGenerator {
 public:
  explicit FileGenerator(const FileDescriptor* file, const Params& params);
  ~FileGenerator();

  // Checks for problems that would otherwise lead to cryptic compile errors.
  // Returns true if there are no problems, or writes an error description to
  // the given string and returns false otherwise.
  bool Validate(string* error);

  void Generate(io::Printer* printer);

  // If we aren't putting everything into one file, this will write all the
  // files other than the outer file (i.e. one for each message, enum, and
  // service type).
  void GenerateSiblings(const string& package_dir,
                        OutputDirectory* output_directory,
                        vector<string>* file_list);
=======
class PROTOC_EXPORT Generator
    : public google::protobuf::compiler::CodeGenerator {
  virtual bool Generate(
      const FileDescriptor* file,
      const string& parameter,
      GeneratorContext* generator_context,
      string* error) const;
};

// To skip reserved keywords in php, some generated classname are prefixed.
// Other code generators may need following API to figure out the actual
// classname.
PROTOC_EXPORT std::string GeneratedClassName(
    const google::protobuf::Descriptor* desc);
PROTOC_EXPORT std::string GeneratedClassName(
    const google::protobuf::EnumDescriptor* desc);
PROTOC_EXPORT std::string GeneratedClassName(
    const google::protobuf::ServiceDescriptor* desc);

inline bool IsWrapperType(const FieldDescriptor* descriptor) {
  return descriptor->cpp_type() == FieldDescriptor::CPPTYPE_MESSAGE &&
      descriptor->message_type()->file()->name() == "google/protobuf/wrappers.proto";
}
>>>>>>> github/3.7.x:src/google/protobuf/compiler/php/php_generator.h

  const string& java_package() { return java_package_; }
  const string& classname()    { return classname_;    }

 private:
  const FileDescriptor* file_;
  const Params& params_;
  string java_package_;
  string classname_;

  GOOGLE_DISALLOW_EVIL_CONSTRUCTORS(FileGenerator);
};

}  // namespace javamicro
}  // namespace compiler
}  // namespace protobuf

<<<<<<< HEAD:src/google/protobuf/compiler/javamicro/javamicro_file.h
}  // namespace google
#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_FILE_H__
=======
#include <google/protobuf/port_undef.inc>

#endif  // GOOGLE_PROTOBUF_COMPILER_PHP_GENERATOR_H__
>>>>>>> github/3.7.x:src/google/protobuf/compiler/php/php_generator.h
