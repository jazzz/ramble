[![Build](https://github.com/jazzz/ramble/actions/workflows/cargo_build.yml/badge.svg)](https://github.com/jazzz/ramble/actions/workflows/cargo_build.yml)
![Status](https://img.shields.io/badge/Project_status-PreAlpha-purple)

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

Message definitions 

```yaml
#Example Ramble File
---
version: "1"                    # Required
namespace: thisnamespace        # namespace not implemented
packets:
  - name: register_sensor       # Required
    fields:
      - sensor_id: u8
      - sensor_name: String     # Type:String not implemented
  - name: sensor_update         # Required
    fields:
      - timestamp: u64          # Type:u64 not implemented
      - sensor_id: u8
      - sensor_value: u8

tagged_unions:                  # This is the default configuratio
  - name: packets               # for now. A wrapping struct with a 
    auto_all: true              # tag is generated for all packets

```

# Types
The following types are currently supported in `ramble.yaml` files

| Category          |  Types   |
| :---------------- | :------: |
| numeric           |  `u8`    |
