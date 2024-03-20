
if [ $# -eq 0 ]; then
  cargo run
else
  cargo run --manifest-path=$1/Cargo.toml
fi
