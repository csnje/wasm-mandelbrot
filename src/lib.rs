// An implementation of the Mandelbrot set.

/// Return pointer to allocated memory of specified size.
#[unsafe(no_mangle)]
pub extern "C" fn create_array(size: usize) -> *mut u8 {
    let mut data = Vec::with_capacity(size);
    let ptr = data.as_mut_ptr();
    std::mem::forget(data);
    ptr
}

/// Calculate size of allocated memory needed for image data.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_data_size(width: usize, height: usize) -> usize {
    width * height * 4 // RGBA per pixel
}

/// Calculate image data for the Mandelbrot set.
///
/// # Safety
///
/// The memory pointer must previously have been allocated for the specified image size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn generate(
    x_range_min: f64,
    x_range_max: f64,
    y_range_min: f64,
    y_range_max: f64,
    image_data_ptr: *mut u8,
    image_width: usize,
    image_height: usize,
) {
    const MAX_ITERATIONS: usize = 40;
    const HUE_RANGE: [f64; 2] = [240.0, 60.0];

    let image = unsafe {
        std::slice::from_raw_parts_mut(image_data_ptr, image_data_size(image_width, image_height))
    };

    for x_idx in 0..image_width {
        let tmp = x_idx as f64 / (image_width - 1) as f64;
        let x0 = x_range_min + (x_range_max - x_range_min) * tmp;

        for y_idx in 0..image_height {
            let tmp = y_idx as f64 / (image_height - 1) as f64;
            let y0 = y_range_min + (y_range_max - y_range_min) * tmp;

            // determine escape iteration
            let mut iteration = 0;
            let (mut x, mut y, mut x2, mut y2) = (0.0, 0.0, 0.0, 0.0);
            while x2 + y2 <= 4.0 && iteration < MAX_ITERATIONS {
                y = (x + x) * y + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                iteration += 1;
            }

            // determine colour for escape iteration
            let (r, g, b) = if iteration == MAX_ITERATIONS {
                (0.0, 0.0, 0.0)
            } else {
                let n = iteration as f64 / MAX_ITERATIONS as f64;
                let (h, s, v) = if n <= 0.5 {
                    (HUE_RANGE[0], 1.0 - 2.0 * n, 0.25 + 1.5 * n)
                } else {
                    (HUE_RANGE[1], 2.0 * n - 1.0, 1.75 - 1.5 * n)
                };
                hsv_to_rgb(h, s, v)
            };

            let image_offset = (image_width * y_idx + x_idx) * 4;
            image[image_offset] = (u8::MAX as f64 * r) as u8;
            image[image_offset + 1] = (u8::MAX as f64 * g) as u8;
            image[image_offset + 2] = (u8::MAX as f64 * b) as u8;
            image[image_offset + 3] = u8::MAX;
        }
    }
}

// Convert from HSV colour to RGB colour.
//
// Input HSV range is ([0,360], [0,1], [0,1]).
// Output RGB range is ([0,1], [0,1], [0,1]).
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let c = v * s;
    let h1 = h % 360.0 / 60.0;
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
