const wasm_file = await fetch('./target/wasm32-unknown-unknown/release/wasm_mandelbrot.wasm');
const { instance } = await WebAssembly.instantiateStreaming(wasm_file);

const exports = instance.exports;
const wasmMemory = exports.memory;

const canvas = document.getElementById('image');
canvas.width = 6000;
canvas.height = 6000;

let imageDataSize = exports.image_data_size(canvas.width, canvas.height);
let imageDataArrayPtr = exports.create_array(imageDataSize);
let imageDataData = new Uint8ClampedArray(wasmMemory.buffer, imageDataArrayPtr, imageDataSize);

exports.generate(-2.0, 1.0, -1.5, 1.5, imageDataArrayPtr, canvas.width, canvas.height);

const imageData = new ImageData(imageDataData, canvas.width, canvas.height);

const ctx = canvas.getContext('2d');
ctx.putImageData(imageData, 0, 0);
