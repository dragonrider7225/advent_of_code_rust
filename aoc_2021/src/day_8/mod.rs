use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Segment {
    // 0, 2, 3, 5, 6, 7, 8, 9
    Top,
    // 0, 3, 4, 5, 6, 8, 9
    UpperLeft,
    // 0, 1, 2, 3, 4, 7, 8, 9
    UpperRight,
    // 2, 3, 4, 5, 6, 8, 9
    Middle,
    // 0, 2, 6, 8
    LowerLeft,
    // 0, 1, 3, 4, 5, 6, 7, 8, 9
    // *all* except 2
    LowerRight,
    // 0, 2, 3, 5, 6, 8, 9
    Bottom,
}

impl Segment {
    fn for_zero() -> &'static [Self] {
        &[
            Self::Top,
            Self::UpperLeft,
            Self::UpperRight,
            Self::LowerLeft,
            Self::LowerRight,
            Self::Bottom,
        ]
    }

    fn for_one() -> &'static [Self] {
        &[Self::UpperRight, Self::LowerRight]
    }

    fn for_two() -> &'static [Self] {
        &[
            Self::Top,
            Self::UpperRight,
            Self::Middle,
            Self::LowerLeft,
            Self::Bottom,
        ]
    }

    fn for_three() -> &'static [Self] {
        &[
            Self::Top,
            Self::UpperRight,
            Self::Middle,
            Self::LowerRight,
            Self::Bottom,
        ]
    }

    fn for_four() -> &'static [Self] {
        &[
            Self::UpperLeft,
            Self::UpperRight,
            Self::Middle,
            Self::LowerRight,
        ]
    }

    fn for_five() -> &'static [Self] {
        &[
            Self::Top,
            Self::UpperLeft,
            Self::Middle,
            Self::LowerRight,
            Self::Bottom,
        ]
    }

    fn for_six() -> &'static [Self] {
        &[
            Self::Top,
            Self::UpperLeft,
            Self::Middle,
            Self::LowerLeft,
            Self::LowerRight,
            Self::Bottom,
        ]
    }

    fn for_seven() -> &'static [Self] {
        &[Self::Top, Self::UpperRight, Self::LowerRight]
    }

    fn for_eight() -> &'static [Self] {
        &[
            Self::Top,
            Self::UpperLeft,
            Self::UpperRight,
            Self::Middle,
            Self::LowerLeft,
            Self::LowerRight,
            Self::Bottom,
        ]
    }

    fn for_nine() -> &'static [Self] {
        &[
            Self::Top,
            Self::UpperLeft,
            Self::UpperRight,
            Self::Middle,
            Self::LowerRight,
            Self::Bottom,
        ]
    }
}

fn unscramble(digit: &str, mappings: &HashMap<char, Segment>) -> usize {
    let mut lights = digit.chars().map(|c| mappings[&c]).collect::<Vec<_>>();
    lights.sort();
    match lights.len() {
        2 => {
            if lights == Segment::for_one() {
                1
            } else {
                unreachable!("Invalid set of segments: {:?}", lights)
            }
        }
        3 => {
            if lights == Segment::for_seven() {
                7
            } else {
                unreachable!("Invalid set of segments: {:?}", lights)
            }
        }
        4 => {
            if lights == Segment::for_four() {
                4
            } else {
                unreachable!("Invalid set of segments: {:?}", lights)
            }
        }
        5 => {
            if lights == Segment::for_two() {
                2
            } else if lights == Segment::for_three() {
                3
            } else if lights == Segment::for_five() {
                5
            } else {
                unreachable!("Invalid set of segments: {:?}", lights)
            }
        }
        6 => {
            if lights == Segment::for_zero() {
                0
            } else if lights == Segment::for_six() {
                6
            } else if lights == Segment::for_nine() {
                9
            } else {
                unreachable!("Invalid set of segments: {:?}", lights)
            }
        }
        7 => {
            if lights == Segment::for_eight() {
                8
            } else {
                unreachable!("Invalid set of segments: {:?}", lights)
            }
        }
        _ => unreachable!("Invalid set of segments: {:?}", lights),
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let (_, output) = line.split_once(" | ").ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Line {line:?} missing output"),
                )
            })?;
            let count = output
                .split_whitespace()
                .filter(|segments| [2, 3, 4, 7].contains(&segments.len()))
                .count();
            Ok(count)
        })
        .sum()
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let (digits, output) = line.split_once(" | ").ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Line {line:?} missing output"),
                )
            })?;
            let mut digits = digits
                .split_whitespace()
                .map(|digit| digit.trim())
                .collect::<Vec<_>>();
            digits.sort_by_key(|s| s.len());
            // There should be exactly one of each digit.
            assert_eq!(digits.len(), 10);
            // `digits[0]` should be 1
            assert_eq!(digits[0].len(), 2);
            // `digits[1]` should be 7
            assert_eq!(digits[1].len(), 3);
            // `digits[2]` should be 4;
            assert_eq!(digits[2].len(), 4);
            // `digits[3]`, `digits[4]`, and `digits[5]` should be 2, 3, and 5 in some order;
            assert_eq!(digits[3].len(), 5);
            assert_eq!(digits[4].len(), 5);
            assert_eq!(digits[5].len(), 5);
            // `digits[6]`, `digits[7]`, and `digits[8]` should be 0, 6, and 9 in some order;
            assert_eq!(digits[6].len(), 6);
            assert_eq!(digits[7].len(), 6);
            assert_eq!(digits[8].len(), 6);
            // `digits[9]` should be 8.
            assert_eq!(digits[9].len(), 7);
            let mut definites = HashMap::new();
            let mut definites_reverse = HashMap::new();
            let ur_lr_light_segments = {
                let mut chars = digits[0].chars();
                [chars.next().unwrap(), chars.next().unwrap()]
            };
            let mut chars = digits[1]
                .chars()
                .filter(|c| !ur_lr_light_segments.contains(c));
            let segment = chars.next().unwrap();
            assert!(chars.next().is_none());
            definites.insert(segment, Segment::Top);
            definites_reverse.insert(Segment::Top, segment);
            let ul_m_light_segments = {
                let mut chars = digits[2]
                    .chars()
                    .filter(|c| !ur_lr_light_segments.contains(c))
                    .filter(|c| c != definites_reverse.get(&Segment::Top).unwrap());
                let distinct = [chars.next().unwrap(), chars.next().unwrap()];
                assert!(chars.next().is_none());
                distinct
            };
            // At this point, `Top` is in `definites`, `UpperRight` and `LowerRight` are
            // `ur_lr_light_segments` in some order, and `UpperRight` and `Middle` are
            // `ul_m_light_segments` in some order.
            //
            // `UpperLeft` is in 5 but not 2 or 3 while `Middle` appears in all three.
            let number_in_three = digits[3]
                .chars()
                .filter(|c| ul_m_light_segments.contains(c))
                .count();
            if number_in_three == 1 {
                // `digits[3]` is either 2 or 3.
                if digits[3].contains(ul_m_light_segments[0]) {
                    definites.insert(ul_m_light_segments[0], Segment::Middle);
                    definites_reverse.insert(Segment::Middle, ul_m_light_segments[0]);
                    definites.insert(ul_m_light_segments[1], Segment::UpperLeft);
                    definites_reverse.insert(Segment::UpperLeft, ul_m_light_segments[1]);
                } else {
                    definites.insert(ul_m_light_segments[0], Segment::UpperLeft);
                    definites_reverse.insert(Segment::UpperLeft, ul_m_light_segments[0]);
                    definites.insert(ul_m_light_segments[1], Segment::Middle);
                    definites_reverse.insert(Segment::Middle, ul_m_light_segments[1]);
                }
            } else {
                // `digits[3]` is 5 so `digits[4]` is either 2 or 3.
                if digits[4].contains(ul_m_light_segments[0]) {
                    definites.insert(ul_m_light_segments[0], Segment::Middle);
                    definites_reverse.insert(Segment::Middle, ul_m_light_segments[0]);
                    definites.insert(ul_m_light_segments[1], Segment::UpperLeft);
                    definites_reverse.insert(Segment::UpperLeft, ul_m_light_segments[1]);
                } else {
                    definites.insert(ul_m_light_segments[0], Segment::UpperLeft);
                    definites_reverse.insert(Segment::UpperLeft, ul_m_light_segments[0]);
                    definites.insert(ul_m_light_segments[1], Segment::Middle);
                    definites_reverse.insert(Segment::Middle, ul_m_light_segments[1]);
                }
            }
            // `UpperRight` is in 0 and 9 but not 6 while `LowerRight` appears in all three.
            let ur_lr_light_segments =
                match digits[6]
                    .chars()
                    .chain(digits[7].chars())
                    .chain(digits[8].chars())
                    .filter(|&c| ur_lr_light_segments[0] == c)
                    .count()
                {
                    2 => ur_lr_light_segments,
                    3 => [ur_lr_light_segments[1], ur_lr_light_segments[0]],
                    _ => unreachable!(
                        "The sixth, seventh, and eighth dimmest digits have an incorrect number of occurrences of {}, which is part of the dimmest digit",
                        ur_lr_light_segments[0],
                    ),
                };
            definites.insert(ur_lr_light_segments[0], Segment::UpperRight);
            definites_reverse.insert(Segment::UpperRight, ur_lr_light_segments[0]);
            definites.insert(ur_lr_light_segments[1], Segment::LowerRight);
            definites_reverse.insert(Segment::LowerRight, ur_lr_light_segments[1]);
            // We now know which light segment is connected to each of the wires except `LowerLeft`
            // and `Bottom`.
            let remaining_lights = {
                let mut remaining = "abcdefg".chars().filter(|c| !definites.contains_key(c));
                [remaining.next().unwrap(), remaining.next().unwrap()]
            };
            let ll_b_light_segments =
                match digits[6]
                    .chars()
                    .chain(digits[7].chars())
                    .chain(digits[8].chars())
                    .filter(|&c| remaining_lights[0] == c)
                    .count()
                {
                    2 => remaining_lights,
                    3 => [remaining_lights[1], remaining_lights[0]],
                    _ => unreachable!(
                        "The sixth, seventh, and eighth dimmest digits have an incorrect number of occurrences of {}, which must be one of LowerLeft or Bottom",
                        remaining_lights[0],
                    ),
                };
            definites.insert(ll_b_light_segments[0], Segment::LowerLeft);
            definites_reverse.insert(Segment::LowerLeft, ll_b_light_segments[0]);
            definites.insert(ll_b_light_segments[1], Segment::Bottom);
            definites_reverse.insert(Segment::Bottom, ll_b_light_segments[1]);
            assert_eq!(definites.len(), 7);
            assert_eq!(definites_reverse.len(), 7);

            Ok(output
                .split_whitespace()
                .map(|s| unscramble(s, &definites))
                .fold(0, |acc, digit| acc * 10 + digit))
        })
        .sum()
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 8 Part 1");
        println!(
            "There are {} digits that are trivial to distinguish",
            part1(&mut BufReader::new(File::open("2021_08.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 8 Part 2");
        println!(
            "The total of all the outputs is {}",
            part2(&mut BufReader::new(File::open("2021_08.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let s = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let expected = 26;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let expected = 61_229;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
