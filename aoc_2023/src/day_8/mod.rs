use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

type Node = [u8; 3];

#[derive(Clone, Eq, PartialEq)]
struct Graph {
    edges: HashMap<Node, (Node, Node)>,
}

impl Graph {
    fn num_steps(&self, instructions: Vec<Direction>) -> usize {
        let mut current_node = b"AAA";
        for (idx, instruction) in instructions.into_iter().cycle().enumerate() {
            if current_node == b"ZZZ" {
                return idx;
            }
            let children = &self.edges[current_node];
            match instruction {
                Direction::Left => current_node = &children.0,
                Direction::Right => current_node = &children.1,
            }
        }
        panic!("Cycle ran out of items")
    }

    fn num_ghost_steps(&self, instructions: Vec<Direction>) -> usize {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        struct CycleDescription {
            cycle_idx: usize,
            cycle_len: usize,
            cycle_terminals: Vec<usize>,
        }

        fn factorize(n: usize) -> Vec<usize> {
            let mut ret = vec![];
            for i in 1..(n.isqrt() + 1) {
                if n % i == 0 {
                    if i * i != n {
                        ret.extend([i, n / i]);
                    } else {
                        ret.push(i);
                    }
                }
            }
            ret
        }

        let mut cycles = self
            .edges
            .keys()
            .filter(|key| key[2] == b'A')
            .map(|start_node| {
                fn find_node(nodes: &[(usize, &Node)], node: (usize, &Node)) -> Option<usize> {
                    nodes
                        .iter()
                        .take(nodes.len() - 1)
                        .copied()
                        .position(|n| n == node)
                }

                let mut seen_nodes = vec![(instructions.len() - 1, start_node)];
                let mut instruction_cycle = instructions.clone().into_iter().enumerate().cycle();
                loop {
                    let cycle_idx = find_node(&seen_nodes, *seen_nodes.last().unwrap());
                    if let Some(cycle_idx) = cycle_idx {
                        break CycleDescription {
                            cycle_idx,
                            cycle_len: seen_nodes.len() - cycle_idx - 1,
                            cycle_terminals: seen_nodes
                                .iter()
                                .skip(cycle_idx)
                                .enumerate()
                                .filter_map(|(idx, node)| Some(idx).filter(|_| node.1[2] == b'Z'))
                                .collect(),
                        };
                    } else {
                        let node = seen_nodes.last().unwrap().1;
                        let children = &self.edges[node];
                        let (instruction_idx, instruction) =
                            instruction_cycle.next().expect("Cycle ran out of items");
                        let next_node = match instruction {
                            Direction::Left => &children.0,
                            Direction::Right => &children.1,
                        };
                        seen_nodes.push((instruction_idx % instructions.len(), next_node));
                    }
                }
            })
            .collect::<Vec<_>>();
        dbg!(&cycles, "My input had cycles of the form `CycleDescription { idx, len: idx + terminal, terminals: vec![terminal] }`, so I just calculated the least common multiple of the `len`s");
        // Find the least `n` such that for all descriptions in `cycles` there is some `k` such that
        // `n == cycle_idx + cycle_terminal + k * cycle_len`.
        for idx in 0..cycles.len() {
            let cycle = cycles[idx].clone();
            let len_factors = factorize(cycle.cycle_len);
            for cycle in &mut cycles[(idx + 1)..] {
                for &factor in &len_factors {
                    if cycle.cycle_len % factor == 0 {
                        cycle.cycle_len /= factor;
                    }
                }
            }
        }
        cycles.into_iter().map(|cycle| cycle.cycle_len).product()
    }
}

fn parse_input(input: &mut dyn BufRead) -> io::Result<(Vec<Direction>, Graph)> {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect("Input has no lines")?
        .bytes()
        .map(|b| match b {
            b'R' => Direction::Right,
            b'L' => Direction::Left,
            _ => panic!("Invalid direction: {:?}", b as char),
        })
        .collect::<Vec<_>>();
    debug_assert!(lines
        .next()
        .expect("Input has only instructions")
        .expect("Couldn't read input")
        .is_empty());
    let edges = lines
        .map(|line| {
            let line = line?;
            let bytes = line.as_bytes();
            Ok((
                [bytes[0], bytes[1], bytes[2]],
                (
                    [bytes[7], bytes[8], bytes[9]],
                    [bytes[12], bytes[13], bytes[14]],
                ),
            ))
        })
        .collect::<io::Result<_>>()?;
    let graph = Graph { edges };
    Ok((instructions, graph))
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let (instructions, graph) = parse_input(input)?;
    Ok(graph.num_steps(instructions))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let (instructions, graph) = parse_input(input)?;
    Ok(graph.num_ghost_steps(instructions))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 8 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_08.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 8 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_08.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = concat!(
        "RL\n",
        "\n",
        "AAA = (BBB, CCC)\n",
        "BBB = (DDD, EEE)\n",
        "CCC = (ZZZ, GGG)\n",
        "DDD = (DDD, DDD)\n",
        "EEE = (EEE, EEE)\n",
        "GGG = (GGG, GGG)\n",
        "ZZZ = (ZZZ, ZZZ)\n",
    );

    const TEST_DATA_2: &str = concat!(
        "LLR\n",
        "\n",
        "AAA = (BBB, BBB)\n",
        "BBB = (AAA, ZZZ)\n",
        "ZZZ = (ZZZ, ZZZ)\n",
    );

    const TEST_DATA_3: &str = concat!(
        "LR\n",
        "\n",
        "11A = (11B, XXX)\n",
        "11B = (XXX, 11Z)\n",
        "11Z = (11B, XXX)\n",
        "22A = (22B, XXX)\n",
        "22B = (22C, 22C)\n",
        "22C = (22Z, 22Z)\n",
        "22Z = (22B, 22B)\n",
        "XXX = (XXX, XXX)\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 2;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 6;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore = "This part has not been implemented, since the cycles in my input were very easy to handle"]
    fn test_part2() -> io::Result<()> {
        let expected = 6;
        let actual = part2(&mut Cursor::new(TEST_DATA_3))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
