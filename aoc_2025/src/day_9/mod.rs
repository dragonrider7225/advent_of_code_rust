#[cfg(feature = "write_images")]
use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
    time::Instant,
};

fn read_red_tiles(input: &mut dyn BufRead) -> io::Result<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                line.split_once(',')
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{line:?} does not contain a comma"),
                        )
                    })
                    .and_then(|(a, b)| {
                        a.parse::<usize>()
                            .map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("{e:?}: {a:?} is not a number"),
                                )
                            })
                            .and_then(|a| {
                                b.parse::<usize>().map(|b| (a, b)).map_err(|e| {
                                    io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        format!("{e:?}: {b:?} is not a number"),
                                    )
                                })
                            })
                    })
            })
        })
        .collect()
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let red_tiles = read_red_tiles(input)?;
    let (tile1, tile2, largest_size) = red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, tile1)| {
            red_tiles.iter().skip(i + 1).map(move |tile2| {
                (
                    tile1,
                    tile2,
                    (tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1),
                )
            })
        })
        .max_by_key(|&(_, _, size)| size)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No rectangles"))?;
    eprintln!("Largest rectangle is {tile1:?}x{tile2:?}, with area {largest_size}");
    Ok(largest_size)
}

/// Returns (a.min(b), a.max(b))
fn minimax(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Red,
    Green,
}

#[cfg(feature = "write_images")]
fn write_bitmap(mut out: impl Write, bitmap: &[Vec<Tile>]) -> io::Result<()> {
    writeln!(out, "P6")?;
    writeln!(out, "{} {}", bitmap[0].len(), bitmap.len())?;
    writeln!(out, "1")?;
    for row in bitmap {
        for tile in row {
            match tile {
                Tile::Empty => write!(out, "\x00\x00\x00")?,
                Tile::Red => write!(out, "\x01\x00\x00")?,
                Tile::Green => write!(out, "\x00\x01\x00")?,
            }
        }
    }
    eprintln!("Compressed image written");
    Ok(())
}

fn floodfill(tiles: &mut [Vec<Tile>]) {
    fn go(tiles: &mut [Vec<Tile>], start: (usize, usize)) {
        let mut frontier = HashSet::new();
        frontier.insert(start);
        while let Some((col, row)) = frontier.extract_if(|_| true).next() {
            frontier.extend(
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .map(|(delta_col, delta_row)| {
                        (
                            col.strict_add_signed(delta_col),
                            row.strict_add_signed(delta_row),
                        )
                    })
                    .filter(|&(col, row)| match &mut tiles[row][col] {
                        tile @ Tile::Empty => {
                            *tile = Tile::Green;
                            true
                        }
                        _ => false,
                    }),
            );
        }
    }

    for row in 0..tiles.len() {
        let mut num_greens = 0;
        for col in 0..tiles[row].len() {
            match tiles[row][col] {
                Tile::Empty => {
                    if num_greens % 2 != 0 {
                        return go(tiles, (col, row));
                    }
                }
                Tile::Red => break,
                Tile::Green => num_greens += 1,
            }
        }
    }
}

fn part2(
    input: &mut dyn BufRead,
    #[cfg(feature = "write_images")] mut out_image: impl Write,
) -> io::Result<usize> {
    let red_tiles = read_red_tiles(input)?;
    let ((min_row, max_row), (min_col, max_col)) = red_tiles.iter().fold(
        ((usize::MAX, 0), (usize::MAX, 0)),
        |((min_row, max_row), (min_col, max_col)), tile| {
            (
                (min_row.min(tile.1), max_row.max(tile.1)),
                (min_col.min(tile.0), max_col.max(tile.0)),
            )
        },
    );
    let region_area = (max_col - min_col) * (max_row - min_row);
    eprintln!(
        "Region covered by ({min_col}, {min_row})x({max_col}, {max_row}) with area {region_area}",
    );
    eprintln!(
        "Greater than {} MB",
        region_area * std::mem::size_of::<usize>() / 1_000_000
    );
    let (mut decompress_col, mut decompress_row) =
        red_tiles
            .iter()
            .fold((vec![], vec![]), |(mut cols, mut rows), tile| {
                cols.extend([tile.0 - 1, tile.0, tile.0 + 1]);
                rows.extend([tile.1 - 1, tile.1, tile.1 + 1]);
                (cols, rows)
            });
    decompress_col.sort_unstable();
    decompress_col.dedup();
    let compress_col = decompress_col.iter().copied().enumerate().fold(
        HashMap::new(),
        |mut acc, (compressed, uncompressed)| {
            acc.insert(uncompressed, compressed);
            acc
        },
    );
    decompress_row.sort_unstable();
    decompress_row.dedup();
    let compress_row = decompress_row.iter().copied().enumerate().fold(
        HashMap::new(),
        |mut acc, (compressed, uncompressed)| {
            acc.insert(uncompressed, compressed);
            acc
        },
    );
    let compress_tile = |(col, row)| (compress_col[&col], compress_row[&row]);
    let compressed_area = decompress_col.len() * decompress_row.len();
    eprintln!(
        "Compressed region is {}x{} with area {compressed_area}",
        decompress_col.len(),
        decompress_row.len(),
    );
    eprintln!(
        "Greater than {} MB",
        compressed_area * std::mem::size_of::<usize>() / 1_000_000
    );
    let mut bitmap = vec![vec![Tile::Empty; decompress_col.len()]; decompress_row.len()];
    for &[tile1, tile2] in red_tiles
        .array_windows::<2>()
        .chain(iter::once(&[*red_tiles.last().unwrap(), red_tiles[0]]))
    {
        let compressed_tile1 = (compress_col[&tile1.0], compress_row[&tile1.1]);
        let compressed_tile2 = (compress_col[&tile2.0], compress_row[&tile2.1]);
        bitmap[compressed_tile1.1][compressed_tile1.0] = Tile::Red;
        bitmap[compressed_tile2.1][compressed_tile2.0] = Tile::Red;
        let (min_row, max_row) = minimax(compressed_tile1.1, compressed_tile2.1);
        let (min_col, max_col) = minimax(compressed_tile1.0, compressed_tile2.0);
        if min_row == max_row {
            for tile in &mut bitmap[min_row][(min_col + 1)..max_col] {
                *tile = Tile::Green;
            }
        } else {
            assert_eq!(min_col, max_col, "Edges must be vertical or horizontal");
            for row in &mut bitmap[(min_row + 1)..max_row] {
                row[min_col] = Tile::Green;
            }
        }
    }
    let floodfill_start = Instant::now();
    floodfill(&mut bitmap);
    eprintln!("Flood-fill in {:?}", Instant::now() - floodfill_start);
    #[cfg(feature = "write_images")]
    write_bitmap(&mut out_image, &bitmap)?;
    let search_start = Instant::now();
    let ret = red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &tile1)| {
            red_tiles
                .iter()
                .skip(i + 1)
                .map(move |&tile2| (tile1, tile2))
        })
        .filter_map(|(tile1, tile2)| {
            let compressed_tile1 = compress_tile(tile1);
            let compressed_tile2 = compress_tile(tile2);
            let (min_row, max_row) = minimax(compressed_tile1.1, compressed_tile2.1);
            let (min_col, max_col) = minimax(compressed_tile1.0, compressed_tile2.0);
            let is_valid_rectangle = (min_row..=max_row)
                .all(|row| (min_col..=max_col).all(|col| !matches!(bitmap[row][col], Tile::Empty)));
            if is_valid_rectangle {
                Some((tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1))
            } else {
                None
            }
        })
        .max()
        .unwrap();
    eprintln!("Search in {:?}", Instant::now() - search_start);
    Ok(ret)
}

pub(super) fn run() -> io::Result<()> {
    {
        let start = Instant::now();
        println!("Year 2025 Day 9 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_09.txt")?))?
        );
        eprintln!("Part 1 total time: {:?}", Instant::now() - start);
    }
    {
        let start = Instant::now();
        println!("Year 2025 Day 9 Part 2");
        #[cfg(feature = "write_images")]
        let out = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open("2025_09.ppm")?;
        println!(
            "{:?}",
            part2(
                &mut BufReader::new(File::open("2025_09.txt")?),
                #[cfg(feature = "write_images")]
                out,
            )?
        );
        eprintln!("Part 2 total time: {:?}", Instant::now() - start);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 50;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 24;
        let actual = part2(
            &mut Cursor::new(TEST_DATA),
            #[cfg(feature = "write_images")]
            io::sink(),
        )?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
