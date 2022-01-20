# Test for running verilog preprocessor written in rust in wasm
The repo uses https://github.com/dalance/sv-parser to do the preprocessing.

The page can be used online at https://jwagen.github.io/wasm-test/. 

Simply paste your verilog with macros to be preprocessed. The parsed is only set up to do preprocessing so the input/output does not need to be valid verilog. The `sv-parser` library which is used for preprocessing is also capable to parse SystemVerilog.

## Build instructions

I have only tested on linux. I roughly followed this guide https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

Make sure rust is installed. The simplest way is to use rustup https://rustup.rs.

Install wasm-pack. You may need to install additional prel packages depending on destro. Read the error messages.
```
cargo install wasm-pack
```

Build with
```
wasm-pack build --target web --dev
```

This will create a pkg folder with the build wasm which is loaded by `index.html`.

Serve the page with a web server. Loading the `index.html` directly in a browser does not work due to browser origin policy.
```
python3 -m http.server
```

