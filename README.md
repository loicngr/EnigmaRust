# EnigmaRust


### Build
- > cargo build --target=x86_64-unknown-linux-musl --release

### Add linux target
- > rustup target add x86_64-unknown-linux-musl

### Start 
- > ./enigma-rust

### Start with min and max value range
- > ./enigma-rust (min) (max) (max try count)
- > ./enigma-rust 0 100 10

### Permission denied ?
- > sudo chmod +x enigma-rust
- > ./enigma-rust
