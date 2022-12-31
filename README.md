# Mandelbrot set

## About

An implementation of the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) in Rust WebAssembly.

![Image of Mandelbrot set](./images/output.png)

## Prerequisites

Install [wasm-pack](https://github.com/rustwasm/wasm-pack).

## Compile

```bash
wasm-pack build --target web
```

## Serve and run

```bash
# Python 2.x
python -m SimpleHTTPServer
# Python 3.x
python3 -m http.server
```

Run in a browser at [http://localhost:8000](http://localhost:8000).
