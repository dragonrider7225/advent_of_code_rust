use std::{
    collections::{HashMap, VecDeque},
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{Deref, DerefMut},
};

use aoc_util::nom_extended::NomParse;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi,
    sequence, IResult,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct ModuleId {
    inner: &'static str,
}

impl Debug for ModuleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Display for ModuleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<'s> NomParse<&'s str> for ModuleId {
    fn nom_parse(input: &'s str) -> IResult<&'s str, Self> {
        combinator::map(character::alpha1, |s: &'s str| Self {
            inner: Box::leak(s.to_string().into_boxed_str()),
        })(input)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

trait Module {
    /// The name of this module.
    fn name(&self) -> &ModuleId;

    /// The names of the modules that this module outputs signals to.
    fn outputs(&self) -> &[ModuleId];

    /// Tell this module that it receives signals from the module named `input`.
    fn connect_input(&mut self, input: &ModuleId);

    /// Sends a signal to this module along the wire from the module named `from`. Returns the
    /// signal (if any) that this module sends to [`self.outputs()`] in response.
    ///
    /// [`self.outputs()`]: #tymethod.outputs
    fn receive_signal(&mut self, signal: Pulse, from: &ModuleId) -> Option<Pulse>;
}

#[derive(Clone, Debug)]
struct FlipFlopModule {
    name: ModuleId,
    outputs: Vec<ModuleId>,
    on: bool,
}

impl FlipFlopModule {
    const fn new(name: ModuleId, outputs: Vec<ModuleId>) -> Self {
        Self {
            on: false,
            name,
            outputs,
        }
    }

    fn nom_parse(s: &str) -> IResult<&str, Self> {
        sequence::preceded(
            bytes::tag("%"),
            combinator::map(
                sequence::separated_pair(
                    ModuleId::nom_parse,
                    bytes::tag(" -> "),
                    multi::separated_list1(bytes::tag(", "), ModuleId::nom_parse),
                ),
                |(name, outputs)| Self::new(name, outputs),
            ),
        )(s)
    }
}

impl Module for FlipFlopModule {
    fn name(&self) -> &ModuleId {
        &self.name
    }

    fn outputs(&self) -> &[ModuleId] {
        &self.outputs
    }

    fn connect_input(&mut self, _: &ModuleId) {}

    fn receive_signal(&mut self, signal: Pulse, _: &ModuleId) -> Option<Pulse> {
        match signal {
            Pulse::Low => {
                self.on = !self.on;
                if self.on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
            Pulse::High => None,
        }
    }
}

#[derive(Clone, Debug)]
struct ConjunctionModule {
    name: ModuleId,
    outputs: Vec<ModuleId>,
    memory: HashMap<ModuleId, Pulse>,
}

impl ConjunctionModule {
    fn new(name: ModuleId, outputs: Vec<ModuleId>) -> Self {
        Self {
            name,
            outputs,
            memory: HashMap::new(),
        }
    }

    fn nom_parse(s: &str) -> IResult<&str, Self> {
        sequence::preceded(
            bytes::tag("&"),
            combinator::map(
                sequence::separated_pair(
                    ModuleId::nom_parse,
                    bytes::tag(" -> "),
                    multi::separated_list1(bytes::tag(", "), ModuleId::nom_parse),
                ),
                |(name, outputs)| Self::new(name, outputs),
            ),
        )(s)
    }
}

impl Module for ConjunctionModule {
    fn name(&self) -> &ModuleId {
        &self.name
    }

    fn outputs(&self) -> &[ModuleId] {
        &self.outputs
    }

    fn connect_input(&mut self, input: &ModuleId) {
        self.memory.insert(*input, Pulse::Low);
    }

    fn receive_signal(&mut self, signal: Pulse, from: &ModuleId) -> Option<Pulse> {
        if self.memory.insert(*from, signal).is_some() {
            if self.memory.values().all(|&pulse| pulse == Pulse::High) {
                Some(Pulse::Low)
            } else {
                Some(Pulse::High)
            }
        } else {
            panic!("Unknown input {from:?}")
        }
    }
}

#[derive(Clone, Debug)]
struct BroadcastModule {
    outputs: Vec<ModuleId>,
}

impl BroadcastModule {
    const NAME: ModuleId = ModuleId {
        inner: "broadcaster",
    };

    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            sequence::preceded(
                bytes::tag("broadcaster -> "),
                multi::separated_list1(bytes::tag(", "), ModuleId::nom_parse),
            ),
            |outputs| Self { outputs },
        )(s)
    }
}

impl Module for BroadcastModule {
    fn name(&self) -> &ModuleId {
        &Self::NAME
    }

    fn outputs(&self) -> &[ModuleId] {
        &self.outputs
    }

    fn connect_input(&mut self, _: &ModuleId) {}

    fn receive_signal(&mut self, signal: Pulse, _: &ModuleId) -> Option<Pulse> {
        Some(signal)
    }
}

#[derive(Clone, Debug)]
enum WrappedModule {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcast(BroadcastModule),
}

impl Deref for WrappedModule {
    type Target = dyn Module;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::FlipFlop(m) => m as &Self::Target,
            Self::Conjunction(m) => m as &Self::Target,
            Self::Broadcast(m) => m as &Self::Target,
        }
    }
}

impl DerefMut for WrappedModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::FlipFlop(m) => m as &mut Self::Target,
            Self::Conjunction(m) => m as &mut Self::Target,
            Self::Broadcast(m) => m as &mut Self::Target,
        }
    }
}

fn parse_module(s: &str) -> IResult<&str, WrappedModule> {
    branch::alt((
        combinator::map(FlipFlopModule::nom_parse, WrappedModule::FlipFlop),
        combinator::map(ConjunctionModule::nom_parse, WrappedModule::Conjunction),
        combinator::map(BroadcastModule::nom_parse, WrappedModule::Broadcast),
    ))(s)
}

fn parse_modules(input: &mut dyn BufRead) -> io::Result<Vec<WrappedModule>> {
    let mut modules = input
        .lines()
        .map(|line| {
            let line = line?;
            parse_module(&line)
                .map(|(_, x)| x)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    for i in 0..modules.len() {
        let current = *modules[i].name();
        let outputs = modules[i].outputs().to_owned();
        for output in outputs {
            if let Some(module) = modules.iter_mut().find(|module| module.name() == &output) {
                module.connect_input(&current);
            }
        }
    }
    Ok(modules)
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut modules = parse_modules(input)?;
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let mut pending_pulses = VecDeque::new();
        pending_pulses.push_back((
            ModuleId { inner: "button" },
            BroadcastModule::NAME,
            Pulse::Low,
        ));
        while let Some((from, to, pulse)) = pending_pulses.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }
            if let Some(module) = modules.iter_mut().find(|module| module.name() == &to) {
                if let Some(pulse) = module.receive_signal(pulse, &from) {
                    let from = to;
                    pending_pulses.extend(module.outputs().iter().map(|&to| (from, to, pulse)));
                }
            }
        }
    }
    Ok(low_pulses * high_pulses)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        (a, b) = (b, a);
    }
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    let doubled = gcd(a, b);
    a * (b / doubled)
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut modules = parse_modules(input)?;
    // Assumes that the modules are connected in the following way:
    //
    //                   [broadcaster]
    //                  /     ...     \
    //              (left)    ...   (right)
    //                  \     ...     /
    //                   \    ...    /
    //                    \   ...   /
    //                     \  ...  /
    //                      \ ... /
    //                       \.../
    //                       (join)
    //                         |
    //                         rx
    // where (left) and (right) represent disjoint sets of modules which are not interconnected,
    // the `...` represents an arbitrary number of additional sets like (left) and (right), and
    // (join) is a `ConjunctionModule`.
    let broadcaster_outputs = modules
        .iter()
        .find(|module| module.name() == &BroadcastModule::NAME)
        .expect("Missing broadcast module")
        .outputs();
    let join_module = *modules
        .iter()
        .find(|module| module.outputs().contains(&ModuleId { inner: "rx" }))
        .expect("Missing output connection")
        .name();
    let sections = broadcaster_outputs
        .iter()
        .map(|&broadcaster_output| {
            let target = modules
                .iter()
                .find(|module| *module.name() == broadcaster_output)
                .expect("Missing connection from broadcaster");
            let mut seen = vec![broadcaster_output];
            let mut pending = Vec::from(target.outputs());
            while let Some(module_id) = pending.pop() {
                if !seen.iter().any(|&module| module == module_id) {
                    let module = modules
                        .iter()
                        .find(|module| module.name() == &module_id)
                        .expect("Broken connection");
                    if module.outputs().contains(&join_module) {
                        return (broadcaster_output, module_id);
                    }
                    pending.extend(module.outputs());
                    seen.push(*module.name());
                }
            }
            panic!(
                "Section beginning with {broadcaster_output} does not reconnect to {join_module}"
            )
        })
        .collect::<Vec<_>>();
    let counts = sections
        .into_iter()
        .map(|(start, r#final)| {
            for i in 1usize.. {
                let mut pending_pulses = VecDeque::new();
                pending_pulses.push_back((ModuleId { inner: "button" }, start, Pulse::Low));
                while let Some((from, to, pulse)) = pending_pulses.pop_front() {
                    if r#final == from {
                        if pulse == Pulse::High {
                            println!("{final} emitted a high pulse on button press {i}");
                            return Ok(i);
                        }
                        continue;
                    }
                    if let Some(module) = modules.iter_mut().find(|module| module.name() == &to) {
                        if let Some(pulse) = module.receive_signal(pulse, &from) {
                            let from = to;
                            pending_pulses
                                .extend(module.outputs().iter().map(|&to| (from, to, pulse)));
                        }
                    }
                }
            }
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Ran out of numbers in usize",
            ))
        })
        .collect::<io::Result<Vec<_>>>()?;
    counts
        .into_iter()
        .reduce(lcm)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Broadcaster doesn't broadcast"))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 20 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_20.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 20 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_20.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = concat!(
        "broadcaster -> a, b, c\n",
        "%a -> b\n",
        "%b -> c\n",
        "%c -> inv\n",
        "&inv -> a\n",
    );

    const TEST_DATA_2: &str = concat!(
        "broadcaster -> a\n",
        "%a -> inv, con\n",
        "&inv -> b\n",
        "%b -> con\n",
        "&con -> output\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 32_000_000;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 11_687_500;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
