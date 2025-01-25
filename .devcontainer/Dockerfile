FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Install dependencies
RUN apt-get update \
    && apt-get install -y git curl llvm-dev libclang-dev clang unzip \
    libusb-1.0-0 libssl-dev libudev-dev pkg-config \
    catch2 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

CMD [ "/bin/bash" ]
