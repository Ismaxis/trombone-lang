FROM ubuntu:24.04

USER root
ARG LLVM_VER=18

RUN apt update

RUN apt install -y \
        build-essential \
        cmake \
        curl \
        gnupg \
        lsb-release \
        ninja-build \
        software-properties-common \
        wget \
        python3-venv \
        python3-pip

ENV VENV_PATH="/opt/venv"
RUN python3 -m venv $VENV_PATH

RUN $VENV_PATH/bin/pip install --upgrade pip \
 && $VENV_PATH/bin/pip install conan

ENV PATH="$VENV_PATH/bin:$PATH"

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN wget -qO- https://apt.llvm.org/llvm.sh | bash -s -- ${LLVM_VER}

RUN apt install -y \
        libpolly-${LLVM_VER}-dev \
        zlib1g-dev \
        libzstd-dev

RUN export LLVM_SYS_${LLVM_VER}0_PREFIX=$(llvm-config --prefix)

WORKDIR /trombone/compiler

COPY ./compiler/conanfile.txt .
RUN conan profile detect
RUN conan install --build=missing .

COPY ./compiler/src ./src/
COPY ./compiler/tests/CMakeLists.txt ./tests/CMakeLists.txt
COPY ./compiler/tests/test.cpp ./tests/test.cpp
COPY ./compiler/CMakeLists.txt .
COPY ./compiler/CMakePresets.json .
COPY ./compiler/conanfile.txt .
COPY ./compiler/Makefile .
RUN make

COPY ./compiler/tests/programs ./tests/programs