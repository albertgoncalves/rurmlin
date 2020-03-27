#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

/* NOTE: Based on `https://github.com/nsf/pnoise/blob/d957b0adee46f6362f008c0cda6f8184a4333c76/test.rs`.
 */

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::env;
use std::f32;
use std::f32::consts::PI;
use std::path::Path;
use std::u8;

const PI_2: f32 = PI * 2.0;
const N: usize = 2048;
const NN: usize = N * N;
const RESOLUTION: f32 = 256.0;

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

struct Noise2dContext {
    gradients: [Vec2; N],
    permutations: [usize; N],
}

fn get_random_gradient(rng: &mut ThreadRng) -> Vec2 {
    let theta: f32 = PI_2 * rng.gen::<f32>();
    Vec2 {
        x: theta.cos(),
        y: theta.sin(),
    }
}

fn init_context() -> Noise2dContext {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut gradients: [Vec2; N] = [Vec2 { x: 0.0, y: 0.0 }; N];
    for x in gradients.iter_mut() {
        *x = get_random_gradient(&mut rng)
    }
    let mut permutations: [usize; N] = [0; N];
    for (i, x) in permutations.iter_mut().enumerate() {
        *x = i;
    }
    permutations.shuffle(&mut rng);
    Noise2dContext {
        gradients,
        permutations,
    }
}

fn get_gradient(origin: Vec2, gradient: Vec2, point: Vec2) -> f32 {
    ((point.x - origin.x) * gradient.x) + ((point.y - origin.y) * gradient.y)
}

fn get_gradient_index(context: &Noise2dContext, x: usize, y: usize) -> usize {
    (context.permutations[x % N] + context.permutations[y % N]) % N
}

fn smooth(x: f32) -> f32 {
    x * x * x * ((x * ((x * 6.0) - 15.0)) + 10.0)
}

fn lerp(a: f32, b: f32, weight: f32) -> f32 {
    a + (weight * (b - a))
}

fn get_noise(context: &Noise2dContext, x: f32, y: f32) -> f32 {
    let point: Vec2 = Vec2 { x, y };
    let x_0f: f32 = x.floor();
    let y_0f: f32 = y.floor();
    let x_1f: f32 = x_0f + 1.0;
    let y_1f: f32 = y_0f + 1.0;
    let x_0: usize = x_0f as usize;
    let y_0: usize = y_0f as usize;
    let x_1: usize = x_1f as usize;
    let y_1: usize = y_1f as usize;
    let w_0: f32 = get_gradient(
        Vec2 { x: x_0f, y: y_0f },
        context.gradients[get_gradient_index(context, x_0, y_0)],
        point,
    );
    let w_1: f32 = get_gradient(
        Vec2 { x: x_1f, y: y_0f },
        context.gradients[get_gradient_index(context, x_1, y_0)],
        point,
    );
    let w_2: f32 = get_gradient(
        Vec2 { x: x_0f, y: y_1f },
        context.gradients[get_gradient_index(context, x_0, y_1)],
        point,
    );
    let w_3: f32 = get_gradient(
        Vec2 { x: x_1f, y: y_1f },
        context.gradients[get_gradient_index(context, x_1, y_1)],
        point,
    );
    let smooth_x: f32 = smooth(x - x_0f);
    let smooth_y: f32 = smooth(y - y_0f);
    lerp(lerp(w_0, w_1, smooth_x), lerp(w_2, w_3, smooth_x), smooth_y)
}

fn main() {
    let wd: String = env::var("WD").unwrap();
    let filepath: &Path = &Path::new(&wd).join("out").join("main.png");
    let pixels: Vec<u8> = {
        let context: Noise2dContext = init_context();
        let mut buffer: Vec<f32> = vec![0.0; NN];
        let mut max: f32 = f32::MIN;
        let mut min: f32 = f32::MAX;
        for y in 0..N {
            let y_noise: f32 = (y as f32) / RESOLUTION;
            let y_index: usize = y * N;
            for x in 0..N {
                let value: f32 =
                    get_noise(&context, (x as f32) / RESOLUTION, y_noise);
                buffer[y_index + x] = value;
                if value < min {
                    min = value;
                }
                if max < value {
                    max = value;
                }
            }
        }
        let scale: f32 = u8::max_value() as f32;
        let mut pixels: Vec<u8> = vec![0; NN];
        let delta: f32 = max - min;
        for (i, p) in pixels.iter_mut().enumerate() {
            *p = (((buffer[i] - min) / delta) * scale) as u8;
        }
        pixels
    };
    image::save_buffer(filepath, &pixels, N as u32, N as u32, image::Gray(8))
        .unwrap();
}
