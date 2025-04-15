FROM fedora:latest@sha256:f84a7b765ce09163d11de44452a4b56c1b2f5571b6f640b3b973c6afc4e63212

USER root

ARG LLVM_VER=18

RUN dnf update -y \
	&& dnf upgrade -y \
	&& dnf install -y ca-certificates python3 \
	&& dnf clean all

RUN dnf install -y zip clang clangd lldb lld \
	libffi-devel zlib-devel libxml2-devel
	
RUN dnf install -y llvm${LLVM_VER}-devel polly-devel

RUN dnf install -y rust cargo

WORKDIR /trombone

COPY . /trombone

CMD [ "cargo", "r" ]
