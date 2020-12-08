use image::{Rgb, RgbImage};
use imageproc;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

const WIDTH_PX: u32 = 30000;
const HEIGHT_PX: u32 = 10000;

const MIN_X: f32 = 1.9;
const MAX_X: f32 = 4.0;

fn main() {
    let mut image = start_image(WIDTH_PX, HEIGHT_PX);

    let step_x: f32 = (MAX_X - MIN_X) / WIDTH_PX as f32;

    for x in 0..WIDTH_PX {
        let rate = MIN_X + step_x * x as f32;

        let populations = population_sizes(rate);

        for population in populations {
            // translate the population size into a y-coord
            let y = (population * HEIGHT_PX as f32) as u32;

            let pixel = Rgb([255, 255, 255]);

            // populate this pixel
            image.put_pixel(x, y, pixel);
        }
    }

    save_image(&image, "test.png");
}

fn logistic_map(rate: f32, xn: f32) -> f32 {
    rate * xn * (1. - xn)
}

fn population_sizes(rate: f32) -> Vec<f32> {
    const NUM_ITERATIONS: usize = 10_000;
    const STARTING_POPULATION: f32 = 0.5;
    const THRESHOLD: f32 = 0.00000001; // arbitrarily set
    const MAX_POPULATIONS: usize = 1000;

    // first, walk through many iterations to get to a (possibly) stable population size
    //let mut population = (0..NUM_ITERATIONS).fold(STARTING_POPULATION, |acc, xn| logistic_map(rate, xn));
    let mut population = STARTING_POPULATION;
    for _ in 0..NUM_ITERATIONS {
        population = rate * population * (1. - population);
    }

    // Now, detect the cycles of a population
    let mut results = vec![population];

    loop {
        // get the next population size
        population = logistic_map(rate, population);

        // see if this value has been seen since we reached a stable population (ie, we've
        // exhausted the cycle)
        if let Some(_) = results.iter().find(|p| (*p - population).abs() < THRESHOLD) {
            break;
        } else if results.len() > MAX_POPULATIONS {
            break;
        } else {
            results.push(population);
        }
    }

    results
}

fn start_image(width: u32, height: u32) -> RgbImage {
    const BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);
    let mut image = RgbImage::new(width, height);
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), BLACK);

    image
}

/// Saves the image to the specified `path`.
fn save_image(image: &RgbImage, path: &str) {
    image.save(path).unwrap();
}
