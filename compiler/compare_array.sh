echo "Compiling with JIT disabled"
time echo "10000000" | cargo run 2> /dev/null > /dev/null -- out/basic_block_no_output.trbc --jit-threshold 0
time echo "10000000" | cargo run 2> /dev/null > /dev/null -- out/basic_block_no_output.trbc --jit-threshold 0
time echo "10000000" | cargo run 2> /dev/null > /dev/null -- out/basic_block_no_output.trbc --jit-threshold 0

echo "Compiling with JIT threshold 1"
time echo "10000000" | cargo run 2> /dev/null > /dev/null -- out/basic_block_no_output.trbc --jit-threshold 1
time echo "10000000" | cargo run 2> /dev/null > /dev/null -- out/basic_block_no_output.trbc --jit-threshold 1
time echo "10000000" | cargo run 2> /dev/null > /dev/null -- out/basic_block_no_output.trbc --jit-threshold 1
