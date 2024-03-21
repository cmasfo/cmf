
if [ $# -ne 0 ]; then
  cargo run --manifest-path=code/rs/$1/Cargo.toml
fi
