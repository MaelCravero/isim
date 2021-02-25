use std::io::prelude::*;
use std::{fs::File, io::BufWriter};

use crate::Color;

pub struct Image {
    // (0, 0) (1, 0)
    // (0, 1) (1, 1)
    grid: Vec<Color>,
    height: usize,
    width: usize,
}

/// Constructors and getters/setters
impl Image {
    pub fn new(height: usize, width: usize) -> Image {
        Image {
            grid: vec![Color(0, 0, 0); height * width],
            height,
            width,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.height {
            panic!("Invalid x coordinate: {}", x);
        } else if y >= self.width {
            panic!("Invalid y coordinate: {}", y);
        }
        self.grid[x * self.width + y] = color
    }
}

impl std::ops::Index<usize> for Image {
    type Output = [Color];
    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index * self.width..(index + 1) * self.width]
    }
}

impl Image {
    pub fn to_ppm(&self, file: &mut File) -> std::io::Result<()> {
        // Each PPM image consists of the following:
        // - A "magic number" for identifying the file type. A ppm image's magic
        //   number is the two characters "P6".
        // - Whitespace (blanks, TABs, CRs, LFs).
        // - A width, formatted as ASCII characters in decimal.
        // - Whitespace.
        // - A height, again in ASCII decimal.
        // - Whitespace.
        // - The maximum color value (Maxval), again in ASCII decimal. Must be
        //   less than 65536 and more than zero.
        // - A single whitespace character (usually a newline).
        // - A raster of Height rows, in order from top to bottom. Each row
        //   consists of Width pixels, in order from left to right. Each pixel
        //   is a triplet of red, green, and blue samples, in that order. Each
        //   sample is represented in pure binary by either 1 or 2 bytes. If the
        //   Maxval is less than 256, it is 1 byte. Otherwise, it is 2 bytes.
        //   The most significant byte is first.

        let mut stream = BufWriter::new(file);

        stream.write_fmt(format_args!("P3\n{} {} 255\n", self.width, self.height))?;
        for i in 0..self.height {
            for j in 0..self.width {
                let Color(r, g, b) = self[i][j];
                stream.write_fmt(format_args!("{} {} {}\n", r, g, b))?;
                ()
            }
            stream.write(b"\n")?;
            ()
        }
        stream.flush()
    }
}
