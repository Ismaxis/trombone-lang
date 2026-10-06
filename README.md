## Update cargo
```shell
rustup update stable
```

## Run with LLVM [DOCKER]
Installation on Ubuntu 22.04:
```shell
sudo wget -qO- https://apt.llvm.org/llvm.sh | sudo bash -s -- 18
sudo apt install libpolly-18-dev zlib1g-dev libzstd-dev
export LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix)
```
https://gitlab.com/taricorp/llvm-sys.rs/-/issues/13

Using docker on x86_64 machine:
```shell
docker build --tag "trombone-lang-env" -f ./docker/ubuntu/Dockerfile . && \
docker run --rm trombone-lang-env
```

Installation on Fedora 41:
```shell
sudo dnf install llvm18-devel polly-devel
export LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix)
```

## Test
```shell
cargo test
```

### Coverage
install coverage tool
```shell
cargo +stable install cargo-llvm-cov --locked
```
run tests
```shell
cargo llvm-cov
```

#### Display coverage in VsCode
- install [extension](https://marketplace.visualstudio.com/items/?itemName=ryanluker.vscode-coverage-gutters)
- run `cargo llvm-cov --lcov --output-path lcov.info`
- press `ctrl + shift + p` and select `Coverage Gutters: Toggle Coverage`

#### Other editors
see [link](https://lib.rs/crates/cargo-llvm-cov)
