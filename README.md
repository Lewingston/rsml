
# RSML

RSML is a simple 2d and 3d graphics library for rust. The implementation uses
[winit](https://github.com/rust-windowing/winit) and [wgpu](https://github.com/gfx-rs/wgpu).
The name and the concept is inspired by the C++ graphics library [SFML](https://github.com/sfml/sfml).

## Building for Web

Install [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen):

```
cargo install wasm-bindgen-cli
```

Build one of the examples:

```
cargo build --example cubes_and_camera --target wasm32-unknown-unknown
```

Create JavaScript bindings:

```
wasm-bindgen \
    target/wasm32-unknown-unknown/debug/examples/cubes_and_camera.wasm \
    --out-dir pkg \
    --target web
```

Run the server application:

```
node test.js
```

Open the web page in the browser:

```
localhost:3000/test.html
```