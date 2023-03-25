use image::{ImageBuffer, Rgb};
use noise::{NoiseFn, Perlin, Seedable};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn generate_art(width: u32, height: u32, seed: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);
    let perlin = Perlin::new();
    perlin.set_seed(seed);

    for x in 0..width {
        for y in 0..height {
            let value = perlin.get([(x as f64) / 100.0, (y as f64) / 100.0, 0.0]);
            let pixel_value = (value.abs() * 255.0) as u8;
            let pixel = Rgb([pixel_value, pixel_value, pixel_value]);
            img.put_pixel(x, y, pixel);
        }
    }

    img
}

fn read_seed_values(seed_file: &str) -> Vec<u32> {
    let mut seed_values = Vec::new();
    if let Ok(file) = File::open(seed_file) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(seed_str) = line {
                if let Ok(seed) = seed_str.trim().parse() {
                    seed_values.push(seed);
                }
            }
        }
    }
    seed_values
}

fn export_seed_values(seed_file: &str, seeds: &[u32]) -> std::io::Result<()> {
    let mut file = File::create(seed_file)?;
    for seed in seeds {
        writeln!(
            &mut file,
            "{}\t{}",
            seed,
            if is_seed_saved(*seed, seed_file) {
                "saved"
            } else {
                "unsaved"
            }
        )?;
    }
    Ok(())
}

fn is_seed_saved(seed: u32, seed_file: &str) -> bool {
    let seed_str = seed.to_string();
    if let Ok(file) = File::open(seed_file) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_str) = line {
                if line_str.starts_with(&seed_str) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!(
            "Usage: {} <width> <height> <output_file>",
            &args[0]
        );
        return;
    }
    let width = args[1].parse().unwrap();
    let height = args[2].parse().unwrap();
    let output_file = &args[3];

    // Load the seed values from a file
    let seed_file = "seeds.tsv";
    let seed_values = read_seed_values(seed_file);

    // Generate a new seed value
    let seed = rand::random();

    // Generate the image
    let img = generate_art(width, height, seed);

    // Save the image to a file
    img.save(output_file).unwrap();

    // Export the seed values as TSV
    let mut all_seed_values = seed_values.clone();
    all_seed_values.push(seed);
    export_seed_values(seed_file, &all_seed_values).unwrap();
}
