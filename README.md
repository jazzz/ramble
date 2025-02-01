[![Build](https://github.com/jazzz/ramble/actions/workflows/cargo_build.yml/badge.svg)](https://github.com/jazzz/ramble/actions/workflows/cargo_build.yml)
![Status](https://img.shields.io/badge/Project_status-PreAlpha-purple)

# Ramble

Ramble is a tool for bootstrapping code to serialize and deserialize binary messages in restrictive/embedded environments.

The project focuses on microcontrollers and other environments where the existing solutions are not well suited. Specifically aims for:
- minimizing footprint size.
- zero runtime dependencies(depending on target)
- determinsitc binaries
- wireformat flexibility.


## Alternatives
- Need forward and backward compatibility use [protobufs](https://protobuf.dev/)
- Need fast zerocopy reads, use [Flatbuffers](https://flatbuffers.dev/)

