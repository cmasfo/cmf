
if [ $# -ne 0 ]; then
  cargo run --manifest-path=code/$1/Cargo.toml
fi
