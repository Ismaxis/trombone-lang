FROM rust:1.86

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y \
        git \
        build-essential \
        ninja-build \
        cmake

WORKDIR /
RUN git clone https://github.com/antlr/antlr4.git
WORKDIR /antlr4/runtime/Cpp/build
RUN cmake .. -DCMAKE_INSTALL_PREFIX=/usr/local -DANTLR4_INSTALL=ON && \
    make -j4 && make install

WORKDIR /trombone
COPY . /trombone
CMD [ "cargo", "test", "-p", "trombone-runner", "-p","trombone-common"]
