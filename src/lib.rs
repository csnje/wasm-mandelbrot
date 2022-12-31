// A WebAssembly implementation of the Mandelbrot set.

use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageData;

const CANVAS_WIDTH: u32 = 1400;
const CANVAS_HEIGHT: u32 = 1200;

// Range of fractal
const MIN_X: f64 = -2.1;
const MAX_X: f64 = 0.6;
const MIN_Y: f64 = -1.25;
const MAX_Y: f64 = 1.25;

const MAX_ITERATIONS: usize = 40;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have window");
    let document = window.document().expect("should have window");

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(CANVAS_WIDTH);
    canvas.set_height(CANVAS_HEIGHT);
    document.body().unwrap().append_child(&canvas)?;

    let context = canvas
        .get_context("2d")?
        .expect("should have 2d context")
        .dyn_into::<CanvasRenderingContext2d>()?;

    let mut data = vec![0; (CANVAS_WIDTH * CANVAS_HEIGHT * 4) as usize];
    let image_data = ImageData::new_with_u8_clamped_array(Clamped(data.as_slice()), CANVAS_WIDTH)?;
    context.put_image_data(&image_data, 0.0, 0.0)?;

    let (point_width, point_height) = (
        (MAX_X - MIN_X) / CANVAS_WIDTH as f64,
        (MAX_Y - MIN_Y) / CANVAS_HEIGHT as f64,
    );

    for i in 0..CANVAS_WIDTH {
        for j in 0..CANVAS_HEIGHT {
            // Determine escape time
            let (x0, y0) = (
                MIN_X + (i as f64 + 0.5) * point_width,
                MIN_Y + (j as f64 + 0.5) * point_height,
            );
            let (mut x, mut y, mut x2, mut y2) = (0.0, 0.0, 0.0, 0.0);
            let mut it = 0;
            while x2 + y2 <= 4.0 && it < MAX_ITERATIONS {
                y = (x + x) * y + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                it += 1;
            }

            // Plot
            let idx = j as usize * CANVAS_WIDTH as usize + i as usize;
            if it < MAX_ITERATIONS {
                let n = it as f64 / MAX_ITERATIONS as f64;

                let (h, s, v) = if n <= 0.5 {
                    (240.0, 1.0 - 2.0 * n, 0.25 + 1.5 * n)
                } else {
                    (60.0, 2.0 * n - 1.0, 1.75 - 1.5 * n)
                };

                let (r, g, b) = hsv_to_rgb(h, s, v);

                data[idx * 4] = (u8::MAX as f64 * r) as u8;
                data[idx * 4 + 1] = (u8::MAX as f64 * g) as u8;
                data[idx * 4 + 2] = (u8::MAX as f64 * b) as u8;
            }
            data[idx * 4 + 3] = u8::MAX;
        }
    }
    context.put_image_data(&image_data, 0.0, 0.0)?;

    Ok(())
}

/// Convert from HSV colour to RGB colour.
///
/// Input HSV range is ([0,360], [0,1], [0,1]).
/// Output RGB range is ([0,1], [0,1], [0,1]).
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let c = v * s;
    let h1 = h / 60.0;
    let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
    let (r1, g1, b1) = if h1 < 1.0 {
        (c, x, 0.0)
    } else if h1 < 2.0 {
        (x, c, 0.0)
    } else if h1 < 3.0 {
        (0.0, c, x)
    } else if h1 < 4.0 {
        (0.0, x, c)
    } else if h1 < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let m = v - c;
    (r1 + m, g1 + m, b1 + m)
}
