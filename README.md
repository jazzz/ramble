![Status](https://img.shields.io/badge/Project_status-PreAlpha-purple)
[![Build](https://github.com/jazzz/ramble/actions/workflows/cargo_build.yml/badge.svg)](https://github.com/jazzz/ramble/actions/workflows/cargo_build.yml)
[![Target(C++)](https://github.com/jazzz/ramble/actions/workflows/test_target_cpp.yml/badge.svg)](https://github.com/jazzz/ramble/actions/workflows/test_target_cpp.yml)
[![Target(Rust)](https://github.com/jazzz/ramble/actions/workflows/test_target_rust.yml/badge.svg)](https://github.com/jazzz/ramble/actions/workflows/test_target_rust.yml)
[![Lint(Clippy)](https://github.com/jazzz/ramble/actions/workflows/lint.yml/badge.svg)](https://github.com/jazzz/ramble/actions/workflows/lint.yml)


# Ramble

Ramble is a tool for bootstrapping code to serialize and deserialize binary messages in restrictive/embedded environments. This differs from a serialization library in that no assumptions are made about the underlying payloads.

The focus of this project is on microcontrollers and other environments where the existing solutions are not well suited. Specifically aims for:
- minimizing footprint size
- zero runtime dependencies
- determinisitic binaries
- wireformat flexibility


## Alternatives
- Need forward and backward compatibility use [protobufs](https://protobuf.dev/)
- Need fast zerocopy reads, use [Flatbuffers](https://flatbuffers.dev/)

# Message Schema

Messages must be well defined with concrete types so they can be parsed. Since no assumptions can be made about the messages, developers must provide unambigious definitions about the messages at compile time.

```yaml
#Example Ramble File
---
version: "1"                    # Required
namespace: thisnamespace        # namespace not implemented
packets:
  - name: register_sensor       # Required
    fields:
      - sensor_id: U8
      - sensor_name: String     # Type:String not implemented
  - name: sensor_update         # Required
    fields:
      - timestamp: U64          # Type:u64 not implemented
      - sensor_id: U8
      - sensor_value: U8

tagged_unions:                  # This is the default configuration
  - name: packets               # for now. A wrapping struct with a 
    auto_all: true              # tag is generated for all packets

```

# Types
The following types are currently supported in `ramble.yaml` files

| Category          |  Types   |
| :---------------- | :------: |
| numeric           |  `U8`, `U16` ,`U32`, `U64`, `I8`, `I16` ,`I32`, `I64`|
