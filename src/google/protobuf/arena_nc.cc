// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Negative compilation test for arena usage.

#include <google/protobuf/arena.h>
#include <google/protobuf/unittest.pb.h>

#ifdef TEST_ARENA_PRIVATE_CONSTRUCTOR

namespace google {
void ArenaPrivateConstructor() {
  google::protobuf::Arena arena;
  protobuf_unittest::TestAllTypes message(&arena);
}

#endif
}  // namespace google
