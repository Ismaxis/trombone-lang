docker build --tag "trombone-lang-compiler-env" -f docker/compiler/Dockerfile .
docker run --rm \
    -v ./out/:/trombone/compiler/out/ \
    trombone-lang-compiler-env:latest \
    /bin/bash -c "/trombone/compiler/build/debug/src/trombone-compiler tests/programs/2.tromb"
