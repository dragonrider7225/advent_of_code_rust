use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    hint::unreachable_unchecked,
    io::{self, BufRead, BufReader},
    ops::Mul,
};

use nom::{
    bytes::complete as bytes, character::complete as character, combinator as comb, multi,
    sequence, IResult,
};

use aoc_util::nom_extended::NomParse;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Material(u64, String);

impl Material {
    fn amount(&self) -> u64 {
        self.0
    }

    fn chemical(&self) -> &String {
        &self.1
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount(), self.chemical())
    }
}

impl Mul<u64> for Material {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.amount() * rhs, self.chemical().clone())
    }
}

impl<'a> Mul<&'a u64> for Material {
    type Output = Self;

    fn mul(self, rhs: &'a u64) -> Self::Output {
        Self(self.amount() * rhs, self.chemical().clone())
    }
}

impl<'a> Mul<u64> for &'a Material {
    type Output = Material;

    fn mul(self, rhs: u64) -> Self::Output {
        Material(self.amount() * rhs, self.chemical().clone())
    }
}

impl<'a, 'b> Mul<&'b u64> for &'a Material {
    type Output = Material;

    fn mul(self, rhs: &'b u64) -> Self::Output {
        Material(self.amount() * rhs, self.chemical().clone())
    }
}

impl<'s> NomParse<&'s str> for Material {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::separated_pair(
                character::u64,
                bytes::tag(" "),
                multi::many1(character::one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")),
            ),
            |(n, name)| Material(n, name.into_iter().collect()),
        )(s)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Reaction(Vec<Material>, Material);

impl Reaction {
    fn ingredients(&self) -> &[Material] {
        &self.0[..]
    }

    fn result(&self) -> &Material {
        &self.1
    }
}

impl Display for Reaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ingredients = self
            .ingredients()
            .iter()
            .map(<Material as ToString>::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{} => {}", ingredients, self.result())
    }
}

impl Mul<u64> for Reaction {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(
            self.ingredients().iter().map(|mat| mat * rhs).collect(),
            self.result() * rhs,
        )
    }
}

impl<'a> Mul<&'a u64> for Reaction {
    type Output = Self;

    fn mul(self, rhs: &'a u64) -> Self::Output {
        Self(
            self.ingredients().iter().map(|mat| mat * rhs).collect(),
            self.result() * rhs,
        )
    }
}

impl<'a> Mul<u64> for &'a Reaction {
    type Output = Reaction;

    fn mul(self, rhs: u64) -> Self::Output {
        Reaction(
            self.ingredients().iter().map(|mat| mat * rhs).collect(),
            self.result() * rhs,
        )
    }
}

impl<'a, 'b> Mul<&'b u64> for &'a Reaction {
    type Output = Reaction;

    fn mul(self, rhs: &'b u64) -> Self::Output {
        Reaction(
            self.ingredients().iter().map(|mat| mat * rhs).collect(),
            self.result() * rhs,
        )
    }
}

impl<'s> NomParse<&'s str> for Reaction {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::separated_pair(
                multi::separated_list1(bytes::tag(", "), Material::nom_parse),
                bytes::tag(" => "),
                Material::nom_parse,
            ),
            |(ingredients, result)| Reaction(ingredients, result),
        )(s)
    }
}

aoc_util::impl_from_str_for_nom_parse!(Reaction);

type Reactions = HashMap<String, Reaction>;

fn parse_reactions() -> io::Result<Reactions> {
    BufReader::new(File::open("2019_14.txt")?)
        .lines()
        .map(|line| {
            line?
                .parse::<Reaction>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .try_fold(Reactions::new(), |mut acc, x| {
            let x = x?;
            assert!(acc.insert(x.result().chemical().clone(), x).is_none());
            Ok(acc)
        })
}

pub(super) fn run() -> io::Result<()> {
    let reactions = parse_reactions()?;
    {
        println!("Year 2019 Day 14 Part 1");
        let mut num_ore = 0;
        let mut materials = HashMap::<String, _>::new();
        let mut leftovers = HashMap::<String, _>::new();
        materials.insert("FUEL".to_string(), 1);
        while !materials.is_empty() {
            // println!("Still need {:?} and {} ORE", materials, num_ore);
            // println!("Have {:?} extra", leftovers);
            let producing = materials.keys().next().unwrap().clone();
            let producing = Material(materials.remove(&producing).unwrap(), producing);
            // print!("Producing {} ", producing);
            let reaction = reactions.get(producing.chemical()).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "1 FUEL requires unproducable material {}",
                        producing.chemical()
                    ),
                )
            })?;
            let repeats = (producing.amount() as f32 / reaction.result().amount() as f32).ceil();
            let reaction = reaction * repeats as u64;
            // println!("by reaction {}", reaction);
            let ingredients = reaction.ingredients();
            for ingredient in ingredients {
                let chemical = ingredient.chemical().clone();
                let material_count = leftovers.remove(&chemical).unwrap_or(0);
                if &chemical == "ORE" {
                    num_ore += ingredient.amount();
                    continue;
                }
                match material_count.cmp(&ingredient.amount()) {
                    // More of the chemical is required for the reaction than previous reactions
                    // have left over.
                    Ordering::Less => {
                        let entry = materials.entry(chemical).or_insert(0);
                        *entry += ingredient.amount() - material_count;
                    }
                    // Previous reactions have left exactly enough of the chemical for the
                    // reaction.
                    Ordering::Equal => {}
                    // Previous reactions have left more than enough of the chemical for the
                    // reaction.
                    Ordering::Greater => {
                        leftovers.insert(chemical, material_count - ingredient.amount());
                    }
                }
            }
            match producing.amount().cmp(&reaction.result().amount()) {
                // The required amount (`producing.amount()`) is less than the amount actually
                // produced (`reaction.result().amount()`).
                Ordering::Less => {
                    let amount = leftovers.entry(producing.chemical().clone()).or_insert(0);
                    *amount += reaction.result().amount() - producing.amount();
                }
                // The required amount is exactly equal to the amount actually produced.
                Ordering::Equal => {}
                // The required amount is greater than the amount actually produced. This case is
                // impossible because `reaction` is defined to be the smallest integer multiple of
                // the canonical reaction that produces at least the required amount of the
                // chemical.
                Ordering::Greater => unsafe { unreachable_unchecked() },
            }
        }
        println!("{num_ore} ORE is required to make 1 FUEL");
    }
    {
        println!("Year 2019 Day 14 Part 2");
        let mut num_ore = 0u64;
        let mut num_fuel = 0u64;
        let mut trying = 10_000u64;
        let mut materials = HashMap::<String, _>::new();
        let mut leftovers = HashMap::<String, _>::new();
        materials.insert("FUEL".to_string(), trying);
        loop {
            trying = if num_ore < 1_000_000_000_000 - 2_400_000_000 {
                10_000u64
            } else if num_ore < 1_000_000_000_000 - 400_000_000 {
                1000u64
            } else if num_ore < 1_000_000_000_000 - 40_000_000 {
                100u64
            } else if num_ore < 1_000_000_000_000 - 4_000_000 {
                10u64
            } else {
                1u64
            };
            materials.insert("FUEL".to_string(), trying);
            if num_fuel.is_power_of_two() {
                println!("Can make {num_fuel} FUEL from {num_ore} ORE");
            }
            while !materials.is_empty() {
                // println!("Still need {:?} and {} ORE", materials, num_ore);
                // println!("Have {:?} extra", leftovers);
                let producing = materials.keys().next().unwrap().clone();
                let producing = Material(materials.remove(&producing).unwrap(), producing);
                let reaction = reactions.get(producing.chemical()).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "1 FUEL requires unproducable material {}",
                            producing.chemical()
                        ),
                    )
                })?;
                let repeats = {
                    // If the required amount of the chemical can be produced by a whole number of
                    // occurrences of the reaction, perform the reaction that many times. Otherwise,
                    // perform the reaction as many times as possible without producing more of the
                    // chemical than required then perform the reaction one more time.
                    let pa = producing.amount();
                    let ra = reaction.result().amount();
                    pa / ra + if pa % ra == 0 { 0 } else { 1 }
                };
                let reaction = reaction * repeats;
                let ingredients = reaction.ingredients();
                for ingredient in ingredients {
                    let chemical = ingredient.chemical().clone();
                    let material_count = leftovers.remove(&chemical).unwrap_or(0);
                    if &chemical == "ORE" {
                        num_ore += ingredient.amount();
                        continue;
                    }
                    match material_count.cmp(&ingredient.amount()) {
                        // More of the chemical is required for the reaction than previous reactions
                        // have left over.
                        Ordering::Less => {
                            let entry = materials.entry(chemical).or_insert(0);
                            *entry += ingredient.amount() - material_count;
                        }
                        // Previous reactions have left exactly enough of the chemical for the
                        // reaction.
                        Ordering::Equal => {}
                        // Previous reactions have left more than enough of the chemical for the
                        // reaction.
                        Ordering::Greater => {
                            leftovers.insert(chemical, material_count - ingredient.amount());
                        }
                    }
                }
                match producing.amount().cmp(&reaction.result().amount()) {
                    // The required amount (`producing.amount()`) is less than the amount actually
                    // produced (`reaction.result().amount()`).
                    Ordering::Less => {
                        let amount = leftovers.entry(producing.chemical().clone()).or_insert(0);
                        *amount += reaction.result().amount() - producing.amount();
                    }
                    // The required amount is exactly equal to the amount actually produced.
                    Ordering::Equal => {}
                    // The required amount is greater than the amount actually produced. This case
                    // is impossible because `reaction` is defined to be the smallest integer
                    // multiple of the canonical reaction that produces at least the required amount
                    // of the chemical.
                    Ordering::Greater => {
                        panic!(
                            "Incorrect required/total production amounts: {}/{}",
                            producing.amount(),
                            reaction.result().amount(),
                        );
                        // unsafe { unreachable_unchecked() },
                    }
                }
            }
            if num_ore <= 1_000_000_000_000 {
                num_fuel += trying;
            } else {
                if trying != 1 {
                    panic!(
                        "{} resulted in {} ORE while trying to produce {} FUEL simultaneously",
                        "Overly permissive scaling back", num_ore, trying,
                    );
                }
                break;
            }
        }
        println!("1E12 ORE can produce up to {num_fuel} FUEL");
    }
    Ok(())
}
