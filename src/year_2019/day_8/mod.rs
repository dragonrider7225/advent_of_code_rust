use std::io;

use aoc_util::nom_extended::NomParse;

use nom::{character::complete as character, combinator as comb, multi, IResult};

#[derive(Clone, Copy)]
struct SIFLayer {
    pixels: [[u8; 25]; 6],
}

impl<'s> NomParse<&'s str> for SIFLayer {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            multi::count(multi::count(character::one_of("0123456789"), 25), 6),
            |pixels_vec| {
                let mut pixels = [[0; 25]; 6];
                for row in 0..6 {
                    for col in 0..25 {
                        let mut s = String::new();
                        s.push(pixels_vec[row][col]);
                        pixels[row][col] = s.parse().unwrap();
                    }
                }
                Self { pixels }
            },
        )(s)
    }
}

#[derive(Clone)]
struct SpaceImageFormat {
    layers: Vec<SIFLayer>,
}

impl<'s> NomParse<&'s str> for SpaceImageFormat {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(multi::many1(SIFLayer::nom_parse), |layers| Self { layers })(s)
    }
}

aoc_util::impl_from_str_for_nom_parse!(SpaceImageFormat);

pub(super) fn run() -> io::Result<()> {
    let pic = String::from_utf8(std::fs::read("2019_8.txt")?)
        .unwrap()
        .parse::<SpaceImageFormat>()
        .unwrap();
    {
        println!("Year 2019 Day 8 Part 1");
        let mut pic = pic.clone();
        pic.layers.sort_by_cached_key(|layer| {
            let mut ret = 0;
            for row in &layer.pixels {
                for pixel in row {
                    if 0 == *pixel {
                        ret += 1;
                    }
                }
            }
            ret
        });
        let layer = pic.layers[0];
        let mut num_ones = 0;
        let mut num_twos = 0;
        for row in &layer.pixels {
            for &pixel in row {
                match pixel {
                    1 => num_ones += 1,
                    2 => num_twos += 1,
                    _ => {}
                }
            }
        }
        println!(
            "The checksum for the layer with the fewest 0s is {}",
            num_ones * num_twos,
        );
    }
    {
        println!("Year 2019 Day 8 Part 2");
        let mut result = [[2; 25]; 6];
        for layer in pic.layers {
            for (result_row, layer_row) in result.iter_mut().zip(&layer.pixels) {
                result_row
                    .iter_mut()
                    .zip(layer_row)
                    .take(25)
                    .filter(|(result_cell, _)| **result_cell == 2)
                    .for_each(|(result_cell, layer_cell)| *result_cell = *layer_cell)
            }
        }
        for row in &result {
            for pixel in row {
                match pixel {
                    0 => print!(" "),
                    1 => print!("X"),
                    _ => panic!("Invalid pixel: {pixel}"),
                }
            }
            println!();
        }
    }
    Ok(())
}
