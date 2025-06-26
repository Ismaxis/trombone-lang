FROM ubuntu:22.04@sha256:d80997daaa3811b175119350d84305e1ec9129e1799bba0bd1e3120da3ff52c3

USER root
ARG LLVM_VER=18

RUN apt-get update

RUN apt-get install -y \
	build-essential \
	curl \
	lsb-release wget software-properties-common gnupg

RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN wget -qO- https://apt.llvm.org/llvm.sh | bash -s -- ${LLVM_VER} && \
	apt install -y \
	libpolly-${LLVM_VER}-dev \
	zlib1g-dev \
	libzstd-dev

RUN export LLVM_SYS_${LLVM_VER}0_PREFIX=$(llvm-config --prefix)

WORKDIR /trombone
COPY ./Cargo.toml /trombone/Cargo.toml
COPY ./crates/ /trombone/crates/

RUN cargo test --no-run
CMD [ "cargo", "test", "--all", "--workspace" ]
