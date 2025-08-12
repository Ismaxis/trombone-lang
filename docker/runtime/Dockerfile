FROM ubuntu:24.04@sha256:7c06e91f61fa88c08cc74f7e1b7c69ae24910d745357e0dfe1d2c0322aaf20f9

USER root
ARG LLVM_VER=18

RUN apt-get update

RUN apt-get install -y \
	build-essential \
	curl \
	lsb-release wget software-properties-common gnupg

RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y && \
	/root/.cargo/bin/rustup install stable && \
	/root/.cargo/bin/rustup default stable
ENV PATH="/root/.cargo/bin:${PATH}"

RUN wget -qO- https://apt.llvm.org/llvm.sh | bash -s -- ${LLVM_VER} && \
	apt install -y \
	libpolly-${LLVM_VER}-dev \
	zlib1g-dev \
	libzstd-dev

RUN export LLVM_SYS_${LLVM_VER}0_PREFIX=$(llvm-config --prefix)

WORKDIR /trombone
COPY . /trombone

CMD [ "cargo", "test", "--all", "--workspace" ]
