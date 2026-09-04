use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
    ops::BitOr,
    str::FromStr,
    time::Instant,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct IndicatorLights {
    lights: usize,
    num_lights: usize,
}

impl IndicatorLights {
    pub fn num_lights(&self) -> usize {
        self.num_lights
    }

    pub fn push(&mut self, button: &Button) {
        debug_assert_eq!(self.num_lights, button.num_lights);
        self.lights ^= button.lights;
    }

    fn minimum_buttons(&self, buttons: &[Button]) -> Option<usize> {
        fn go(
            target: &mut IndicatorLights,
            buttons: &[Button],
            pushed: &mut HashSet<usize>,
            cache: &mut HashMap<Vec<usize>, Option<usize>>,
        ) -> Option<usize> {
            let _indent = pushed.len();
            let log = |_msg: fmt::Arguments| {
                #[cfg(test)]
                eprintln!("{: >_indent$}{_msg}", "");
            };
            macro_rules! log {
                ($args:tt) => {
                    log(format_args!($args))
                };
            }
            log!("Calling go for {target}");
            if target.lights == 0 {
                log!("Already at target");
                return Some(0);
            }
            let mut key = pushed.iter().copied().collect::<Vec<_>>();
            key.sort_unstable();
            if let Some(&ret) = cache.get(&key) {
                return ret;
            }
            let possible_lights = reduce_bit_or(
                buttons
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !pushed.contains(i))
                    .map(|(_, button)| button.lights),
            )
            .unwrap_or(0);
            if target.lights & !possible_lights != 0 {
                log!("Can't reach {target} with buttons = {{");
                for (i, button) in buttons.iter().enumerate() {
                    if !pushed.contains(&i) {
                        log!("{button}");
                    }
                }
                log!("}}");
                cache.insert(key, None);
                return None;
            }
            let ret = buttons
                .iter()
                .enumerate()
                .filter_map(|(i, button)| {
                    if !pushed.contains(&i) {
                        log!(" Testing button {button}");
                        pushed.insert(i);
                        target.push(button);
                        let ret = go(target, buttons, pushed, cache).map(|n| n + 1);
                        target.push(button);
                        pushed.remove(&i);
                        ret.inspect(|n| log!("Found method for {target} in {n} buttons"))
                    } else {
                        None
                    }
                })
                .min();
            debug_assert_eq!(pushed.len(), key.len());
            for idx in &key {
                debug_assert!(pushed.contains(idx));
            }
            cache.insert(key, ret);
            ret
        }

        for button in buttons {
            debug_assert_eq!(button.num_lights, self.num_lights());
        }
        go(
            &mut self.clone(),
            buttons,
            &mut HashSet::new(),
            &mut HashMap::new(),
        )
    }
}

impl Display for IndicatorLights {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self { lights, num_lights } = self;
        write!(f, "[{lights:0num_lights$b}]")
    }
}

impl FromStr for IndicatorLights {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some('[') => {}
            Some(c) => return Err(format!("{s:?} is not a valid indicator lights string; must begin with '[', not {c:?}")),
            None => return Err(format!("{s:?} is not a valid indicator lights string; an empty list of indicator lights would be represented by \"[]\"")),
        }
        let mut num_lights = 0;
        let mut lights = 0;
        loop {
            match chars.next() {
                Some('#') => lights |= 1usize,
                Some('.') => {}
                Some(']') => {
                    lights >>= 1;
                    break;
                }
                Some(c) => {
                    return Err(format!(
                        "{c:?} is not a valid character at any point in an indicator lights string"
                    ))
                }
                None => {
                    return Err(format!(
                        "Ran out of characters while parsing {s:?} as indicator lights"
                    ))
                }
            }
            num_lights += 1;
            lights <<= 1;
        }
        if let Some(c) = chars.next() {
            return Err(format!("Trailing characters starting with {c:?} when parsing {s:?} as indicator lights string"));
        }
        Ok(Self {
            lights,
            num_lights: num_lights as _,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Button {
    lights: usize,
    num_lights: usize,
}

impl Button {
    fn parse_with(s: &str, indicator_panel_size: usize) -> Result<Self, <Self as FromStr>::Err> {
        let mut ret = s.parse::<Self>()?;
        let Some(diff) = indicator_panel_size.checked_sub(ret.num_lights) else {
            return Err(format!(
                "Button is connected to {} out of {indicator_panel_size} lights",
                ret.num_lights,
            ));
        };
        if diff != 0 {
            ret.lights <<= diff;
            ret.num_lights += diff;
        }
        Ok(ret)
    }

    /// Converts the connections from a form that easily interacts with [`IndicatorLights`] to a
    /// form that easily interacts with [`Joltage`]. Simply `.zip(..)` the return value with the
    /// joltage requirements.
    fn unpack(&self) -> impl Iterator<Item = bool> {
        iter::successors(Some(*self), |button| {
            Some(Button {
                lights: button.lights / 2,
                num_lights: button.num_lights.checked_sub(1).filter(|&n| n > 0)?,
            })
        })
        .map(|button| button.lights & 1 == 1)
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self { lights, num_lights } = self;
        write!(f, "({lights:0num_lights$b})")
    }
}

impl FromIterator<bool> for Button {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = bool>,
    {
        iter.into_iter().fold(
            Self {
                lights: 0,
                num_lights: 0,
            },
            |mut acc, effect| {
                acc.lights |= (effect as usize) << acc.num_lights;
                acc.num_lights += 1;
                acc
            },
        )
    }
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some('(') => {}
            Some(c) => {
                return Err(format!(
                    "Got unexpected initial character {c:?} while parsing button wiring"
                ))
            }
            None => {
                return Err(format!(
                    "The empty string is not a button wiring diagram. The minimal diagram is {:?}",
                    "()"
                ))
            }
        }
        let Some(_) = chars.position(|c| c == ')') else {
            return Err("Ran out of characters while parsing button wiring".to_string());
        };
        if let Some(c) = chars.next() {
            return Err(format!(
                "Wiring diagram {s:?} contains trailing characters starting at {c:?}"
            ));
        }
        let stripped = &s['('.len_utf8()..(s.len() - ')'.len_utf8())];
        let nums = stripped
            .split(',')
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|e| format!("{e:?}: {n:?} is not a number"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let num_lights = nums.iter().copied().max().map(|n| n + 1).unwrap_or(0);
        let mut lights = 0;
        for i in 0..num_lights {
            lights <<= 1;
            if nums.contains(&i) {
                lights |= 1;
            }
        }
        Ok(Self { lights, num_lights })
    }
}

fn reduce_bit_or<T>(iter: impl IntoIterator<Item = T>) -> Option<T>
where
    T: BitOr<Output = T> + Default,
{
    iter.into_iter().reduce(|acc, x| acc | x)
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                let mut parts = line.split_whitespace();
                let target = parts
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty line"))?
                    .parse::<IndicatorLights>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let buttons = parts
                    .by_ref()
                    .take_while(|part| part.starts_with('('))
                    .map(|part| {
                        Button::parse_with(part, target.num_lights())
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                    })
                    .collect::<io::Result<Vec<_>>>()?;
                target.minimum_buttons(&buttons).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("No way to solve {line:?}"),
                    )
                })
            })
        })
        .try_fold(0, |acc, line| line.map(|line| line + acc))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Joltage {
    joltages: Vec<usize>,
}

impl Joltage {
    fn minimum_buttons(&self, buttons: &[Button]) -> Option<usize> {
        fn unpack_button_list(idx: usize, num_buttons: usize) -> impl Iterator<Item = bool> {
            (0..num_buttons).map(move |bit| (idx & (1 << bit)) != 0)
        }

        fn divide(mut this: Joltage, buttons: &[Button], patterns: &[Button]) -> Option<usize> {
            #[cfg(test)]
            eprintln!("Dividing {this}");
            debug_assert!(this.joltages.iter().all(|joltage| joltage % 2 == 0));
            this.joltages.iter_mut().for_each(|joltage| *joltage /= 2);
            conquer(&this, buttons, patterns).map(|subtotal| subtotal * 2)
        }

        fn conquer(this: &Joltage, buttons: &[Button], patterns: &[Button]) -> Option<usize> {
            if this.joltages.iter().all(|&joltage| joltage == 0) {
                return Some(0);
            }
            patterns
                .iter()
                .enumerate()
                .filter_map(|(idx, pattern)| {
                    if pattern
                        .unpack()
                        .zip(&this.joltages)
                        .all(|(odd, joltage)| (joltage % 2 != 0) == odd)
                    {
                        #[cfg(test)]
                        eprintln!("Conquering {this} with pattern ID {idx}: {pattern}");
                        let mut sub = this.clone();
                        unpack_button_list(idx, buttons.len())
                            .zip(buttons)
                            .filter_map(|(push, button)| push.then_some(button))
                            .try_for_each(|button| {
                                button.unpack().zip(&mut sub.joltages).try_for_each(
                                    |(effect, joltage)| {
                                        *joltage = joltage.checked_sub(effect as usize)?;
                                        Some(())
                                    },
                                )
                            })?;
                        divide(sub, buttons, patterns).map(|subtotal| {
                            subtotal
                                + unpack_button_list(idx, buttons.len())
                                    .filter(|&push| push)
                                    .count()
                        })
                    } else {
                        None
                    }
                })
                .min()
        }

        #[cfg(test)]
        {
            eprintln!("Solving {self}");
            eprintln!("There are {} lights", self.joltages.len());
        }
        let patterns = (0..2usize.pow(buttons.len() as _))
            .map(|idx| {
                unpack_button_list(idx, buttons.len())
                    .zip(buttons)
                    .filter_map(|(push, button)| push.then_some(button))
                    .fold(vec![false; self.joltages.len()], |mut acc, button| {
                        button
                            .unpack()
                            .zip(&mut acc)
                            .for_each(|(effect, joltage)| *joltage ^= effect);
                        acc
                    })
                    .into_iter()
                    .collect::<Button>()
            })
            .collect::<Vec<_>>();
        #[cfg(test)]
        {
            eprint!("[");
            if !patterns.is_empty() {
                eprint!("{}", patterns[0]);
            }
            for pattern in patterns.iter().skip(1) {
                eprint!(", {pattern}");
            }
            eprintln!("]");
        }
        conquer(self, buttons, &patterns)
    }
}

impl Display for Joltage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for joltage in self.joltages.iter().skip(1).rev() {
            write!(f, "{joltage},")?;
        }
        for joltage in self.joltages.iter().take(1) {
            write!(f, "{joltage}")?;
        }
        write!(f, "}}")
    }
}

impl FromStr for Joltage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some('{') => {}
            Some(c) => {
                return Err(format!(
                    "Got unexpected initial character {c:?} while parsing joltage requirement"
                ))
            }
            None => {
                return Err(format!(
                "The empty string is not a joltage requirement. The minimal description is {:?}",
                "{}"
            ))
            }
        }
        let Some(_) = chars.position(|c| c == '}') else {
            return Err("Ran out of characters while parsing button wiring".to_string());
        };
        if let Some(c) = chars.next() {
            return Err(format!(
                "Wiring diagram {s:?} contains trailing characters starting at {c:?}"
            ));
        }
        let stripped = &s['{'.len_utf8()..(s.len() - '}'.len_utf8())];
        let mut joltages = stripped
            .split(',')
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|e| format!("{e:?}: {n:?} is not a number"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        joltages.reverse();
        Ok(Self { joltages })
    }
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                let mut parts = line.split_whitespace();
                let indicator_lights = parts
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty line"))?;
                let indicator_lights = indicator_lights
                    .parse::<IndicatorLights>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let (buttons, joltage) = {
                    let mut buttons = vec![];
                    let mut joltage = None;
                    for part in parts.by_ref() {
                        if !part.starts_with('(') {
                            joltage = Some(part);
                            break;
                        }
                        buttons.push(
                            Button::parse_with(part, indicator_lights.num_lights())
                                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                        );
                    }
                    (buttons, joltage)
                };
                let joltage = joltage
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Line is missing joltage requirement",
                        )
                    })?
                    .parse::<Joltage>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                #[cfg(test)]
                eprintln!("{joltage}");
                if let Some(part) = parts.next() {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Line {line:?} contains trailing characters after joltage beginning with {part:?}")));
                }
                debug_assert_eq!(indicator_lights.num_lights, joltage.joltages.len());
                let ret = joltage.minimum_buttons(&buttons)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, format!("No way to satisfy joltage requirements for {line:?}"))
                    })?;
                #[cfg(test)]
                eprintln!("{ret} button presses required");
                Ok(ret)
            })
        })
        .try_fold(0, |acc, line| line.map(|line| acc + line))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 10 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2025_10.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 10 Part 2");
        #[cfg(debug_assertions)]
        eprintln!("28.5s on my input in `--dev`. 1s on my input in `--release`.");
        let start = Instant::now();
        let res = part2(&mut BufReader::new(File::open("2025_10.txt")?))?;
        eprintln!("Result computed in {:?}", Instant::now() - start);
        println!("{res}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 7;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 33;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
