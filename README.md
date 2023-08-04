## [LFX Ffmpeg Plugin](https://github.com/WasmEdge/WasmEdge/issues/2689)

This repository is the solution to [Pretest #2703](https://github.com/WasmEdge/WasmEdge/discussions/2703). 


The solution is split into two repositories-
1. [WasmEdge-FFMPEG](https://github.com/Hrushi20/WasmEdge-FFMPEG) 
<br> Contains Ffmpeg Plugin that acts as a bridge between WasmEdge and Ffmpeg C API.

2. [ffmpeg-rust](https://github.com/Hrushi20/ffmpeg-rust)<br> Uses the WasmEdgeFfmpeg plugin built in `WasmEdge-FFMPEG` and executes Ffmpeg C API functions.

The wasm function reads the `assets/test.wav` file and calls the Ffmpeg C API to read the meta data of the file and logs this data in console.

### Note-
First, follow the steps in [WasmEdge-FFMPEG](https://github.com/Hrushi20/WasmEdge-FFMPEG#readme) repository to build the plugin shared library.

To use the FFMPEG Plugin, follow the below steps-

### 1. Dependecies-
- [Witc](https://github.com/second-state/witc)
- [Rust](https://www.rust-lang.org/tools/install)

Witc is a compiler which generates rust code for *.wit files.

### 2. Generate C API in Rust using Witc
```
    witc plugin wasmedge_ffmpeg.wit > src/generated.rs
```
A generated.rs file is created in src directory.

### 3. Build Project-
Use cargo to build the project into .wasm
```
cargo build --target wasm32-wasi --release
```

### 4. Execute .wsm file using WasmEdge-
```
WASMEDGE_PLUGIN_PATH=/usr/local/lib/wasmedge/libwasmedgePluginWasmEdgeFfmpeg.dylib wasmedge --dir ./. target/wasm32-wasi/release/ffmpegRust.wasm
```

WASMEDGE_PLUGIN_PATH takes the path of the shared plugin created in [WasmEdge-FFMPEG](https://github.com/Hrushi20/WasmEdge-FFMPEG#readme).

In linux, the path is WASMEDGE_PLUGIN_PATH=/usr/local/lib/wasmedge/libwasmedgePluginWasmEdgeFfmpeg.so

Result-
[Result](https://github.com/Hrushi20/ffmpeg-rust/blob/main/assets/results.png)

Reference-
[Witc](https://github.com/Hrushi20/ffmpeg-rust/blob/main/assets/witc.png)
