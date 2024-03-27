help:
  just --list

# Run the app & restart on chages; uses mold as the linker
dev:
  RUST_LOG=debug cargo watch -s 'mold -run cargo build; cargo run'
