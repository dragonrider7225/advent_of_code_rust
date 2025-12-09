use std::{
    fmt::{self, Debug, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Eq, PartialEq)]
struct Vec3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Vec3 {
    pub fn squared_distance(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3{{{},{},{}}}", self.x, self.y, self.z)
    }
}

impl FromStr for Vec3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or_else(|| unreachable!()).and_then(|s| {
            s.parse::<usize>()
                .map_err(|e| format!("{e:?}: {s:?} is not a number"))
        })?;
        let y = parts
            .next()
            .ok_or_else(|| format!("{s:?} is not a Vec3: not enough commas"))
            .and_then(|s| {
                s.parse::<usize>()
                    .map_err(|e| format!("{e:?}: {s:?} is not a number"))
            })?;
        let z = parts
            .next()
            .ok_or_else(|| format!("{s:?} is not a Vec3: not enough commas"))
            .and_then(|s| {
                s.parse::<usize>()
                    .map_err(|e| format!("{e:?}: {s:?} is not a number"))
            })?;
        if parts.next().is_some() {
            return Err(format!("{s:?} is not a Vec3: too many commas"));
        };
        Ok(Self { x, y, z })
    }
}

// fn print_distances(distances: &[((usize, usize), usize)]) {
//     eprintln!("[");
//     if !distances.is_empty() {
//         for &((i, j), distance) in distances {
//             eprintln!("    (({i}, {j}), {distance}),");
//         }
//     }
//     eprintln!("]");
// }

fn read_boxes(input: &mut dyn BufRead) -> io::Result<Vec<Vec3>> {
    input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                line.parse::<Vec3>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
        .collect()
}

fn part1(input: &mut dyn BufRead, num_pairs: usize) -> io::Result<usize> {
    let boxes = read_boxes(input)?;
    dbg!(boxes.len());
    let shortest_distances = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, box1)| {
            boxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, box2)| ((i, j), box1.squared_distance(box2)))
        })
        .fold(vec![], |mut acc, (idxs, dist)| {
            acc.push((idxs, dist));
            // eprint!("acc = ");
            // print_distances(&acc);
            for i in (0..(acc.len() - 1)).rev() {
                if acc[i + 1].1 < acc[i].1 {
                    acc.swap(i, i + 1);
                } else {
                    break;
                }
            }
            if acc.len() > num_pairs {
                acc.pop();
            }
            acc
        })
        .into_iter()
        .collect::<Vec<_>>();
    // eprint!("shortest_distances = ");
    // print_distances(&shortest_distances);
    let mut circuits = boxes.iter().copied().map(|v| vec![v]).collect::<Vec<_>>();
    for ((i, j), _distance) in shortest_distances {
        // eprint!("Connecting boxes {i} and {j} with squared-distance {_distance} ");
        let i = circuits
            .iter()
            .position(|circuit| circuit.contains(&boxes[i]))
            .unwrap();
        let j = circuits
            .iter()
            .position(|circuit| circuit.contains(&boxes[j]))
            .unwrap();
        // eprintln!("in circuits {i} and {j}");
        if i != j {
            let old_circuit = circuits.swap_remove(i.max(j));
            circuits[j.min(i)].extend(old_circuit);
        }
    }
    circuits.sort_unstable_by_key(|circuit| circuit.len());
    Ok(circuits
        .into_iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.len())
        .product())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let boxes = read_boxes(input)?;
    let distances_size =
        (boxes.len() - 1) * boxes.len() / 2 * std::mem::size_of::<((usize, usize), usize)>();
    eprintln!("The list of distances takes {distances_size} B");
    if distances_size > 4_000_000_000 {
        return Err(io::Error::new(
            io::ErrorKind::FileTooLarge,
            "Too many boxes",
        ));
    }
    let mut distances = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, box1)| {
            boxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, box2)| ((i, j), box1.squared_distance(box2)))
        })
        .collect::<Vec<_>>();
    distances.sort_unstable_by_key(|&(_, distance)| distance);
    let mut circuits = boxes.iter().map(|&r#box| vec![r#box]).collect::<Vec<_>>();
    for ((i, j), _distance) in distances {
        let box1 = boxes[i];
        let box2 = boxes[j];
        let i = circuits
            .iter()
            .position(|circuit| circuit.contains(&box1))
            .unwrap();
        let j = circuits
            .iter()
            .position(|circuit| circuit.contains(&box2))
            .unwrap();
        if i != j {
            let sub = circuits.swap_remove(i.max(j));
            circuits[j.min(i)].extend(sub);
        }
        if circuits.len() == 1 {
            return Ok(box1.x * box2.x);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "Couldn't connect all boxes into a single circuit: {} circuits remaining",
            circuits.len()
        ),
    ))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 8 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_08.txt")?), 1000)?
        );
    }
    {
        println!("Year 2025 Day 8 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_08.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "162,817,812\n",
        "57,618,57\n",
        "906,360,560\n",
        "592,479,940\n",
        "352,342,300\n",
        "466,668,158\n",
        "542,29,236\n",
        "431,825,988\n",
        "739,650,466\n",
        "52,470,668\n",
        "216,146,977\n",
        "819,987,18\n",
        "117,168,530\n",
        "805,96,715\n",
        "346,949,466\n",
        "970,615,88\n",
        "941,993,340\n",
        "862,61,35\n",
        "984,92,344\n",
        "425,690,689\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 40;
        let actual = part1(&mut Cursor::new(TEST_DATA), 10)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 25_272;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
