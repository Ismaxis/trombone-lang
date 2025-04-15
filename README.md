

```shell
sudo wget -qO- https://apt.llvm.org/llvm.sh | sudo bash -s -- 18
sudo apt install libpolly-18-dev
export LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix)
```
https://gitlab.com/taricorp/llvm-sys.rs/-/issues/13

Using docker on x86_64 machine:
```shell
docker build --tag "trombone-lang-env" -f ./docker/ubuntu/Dockerfile . && \
docker run --rm trombone-lang-env 
```
