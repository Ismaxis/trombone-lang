echo "Compiling with JIT disabled"
time echo "400000" | cargo run 2> /dev/null > /dev/null -- out/jit_bench.trbc  --jit-threshold 0
time echo "400000" | cargo run 2> /dev/null > /dev/null -- out/jit_bench.trbc  --jit-threshold 0
time echo "400000" | cargo run 2> /dev/null > /dev/null -- out/jit_bench.trbc  --jit-threshold 0

echo "Compiling with JIT threshold 10"
time echo "400000" | cargo run 2> /dev/null > /dev/null -- out/jit_bench.trbc  --jit-threshold 10
time echo "400000" | cargo run 2> /dev/null > /dev/null -- out/jit_bench.trbc  --jit-threshold 10
time echo "400000" | cargo run 2> /dev/null > /dev/null -- out/jit_bench.trbc  --jit-threshold 10
