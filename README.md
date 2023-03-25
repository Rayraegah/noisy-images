# Perlin noise art generator

This Rust program generates a black-and-white image using [Perlin noise](https://en.wikipedia.org/wiki/Perlin_noise).

## Usage

```bash
cargo run --release -- <width> <height> <output_file>
```

where:

- `width`: the width of the generated image, in pixels.
- `height`: the height of the generated image, in pixels.
- `output_file`: the name of the file where the generated image will be saved. Must end in `.png`.

Example:

```bash
cargo run --release -- 800 600 art.png
```

This will generate an `800x600` PNG image named `art.png`.

## Seed values
The program generates a random seed value for each image. If you want to generate the same image again, you can use the same seed value. The seed values are stored in the `seeds.tsv` file, in tab-separated value (TSV) format. Each line in the file contains a seed value and a status flag ("saved" or "unsaved"). The status flag indicates whether the image generated with that seed value has been saved to a file.

You can view the seed values by opening the `seeds.tsv` file in a text editor. If you want to generate an image with a specific seed value, you can add the seed value to the `seeds.tsv` file and run the program again. For example:

```bash
cargo run --release -- 800 600 art.png
```

will generate a new image with a random seed value, and add that seed value to the seeds.tsv file. If you want to generate the same image again, you can add the seed value to the seeds.tsv file and run the program again:

```bash
echo "12345	saved" >> seeds.tsv
cargo run --release -- 800 600 art.png
```

This will generate an image with the seed value `12345` and save it to the file `art.png`.


## Dependencies
This program uses the following external crates:

- `image`: for image manipulation and file I/O.
- `noise`: for Perlin noise generation.
- `rand`: for generating random seed values.

## License

See `License` file.