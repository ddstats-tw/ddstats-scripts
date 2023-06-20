# ddstats/scripts


## master-parser
simd-json requries tweaking to the systems Cargo.toml file. Add the following in `~/.cargo/config.toml`

```
[build]
rustflags = ["-C", "target-cpu=native"]

[target.wasm32-unknown-unknown]
rustflags = ["-C", "target-feature=+simd128"]

[target.wasm32-wasi]
rustflags = ["-C", "target-feature=+simd128"]
```