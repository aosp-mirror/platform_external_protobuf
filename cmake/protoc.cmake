set(protoc_files
  ${protobuf_SOURCE_DIR}/src/google/protobuf/compiler/main.cc
)

android_add_executable(TARGET protoc NODISTRIBUTE SRC ${protoc_files} ${protobuf_version_rc_file})
target_link_libraries(protoc PRIVATE
  libprotoc
  libprotobuf
  ${protobuf_ABSL_USED_TARGETS}
)
add_executable(protobuf::protoc ALIAS protoc)

set_target_properties(protoc PROPERTIES
    VERSION ${protobuf_VERSION})

# Make sure emulator build can find protoc
set(protobuf_PROTOC_EXE protoc)
set(PROTOBUF_PROTOC_EXECUTABLE "$<TARGET_FILE:protoc>" CACHE PATH "Protocol buffer executable" FORCE)
