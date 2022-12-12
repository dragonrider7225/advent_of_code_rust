use std::{
    collections::{HashMap, HashSet},
    io,
};

fn invert_case(c: char) -> char {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c
    }
}

#[derive(Clone)]
struct Braid {
    crossings: Vec<char>,
}

impl Braid {
    fn new() -> Braid {
        Braid {
            crossings: Vec::new(),
        }
    }

    fn add(&mut self, crossing: char) {
        match self.crossings.pop() {
            None => self.crossings.push(crossing),
            Some(last_crossing) if invert_case(last_crossing) == crossing => {}
            Some(last_crossing) => {
                self.crossings.push(last_crossing);
                self.crossings.push(crossing);
            }
        }
    }

    fn len(&self) -> usize {
        self.crossings.len()
    }

    fn strip(self, c: char) -> Braid {
        let mut ret = Braid::new();
        let c = c.to_ascii_lowercase();
        for crossing in self.crossings {
            if crossing.to_ascii_lowercase() != c {
                ret.add(crossing);
            }
        }
        ret
    }
}

fn get_polymer() -> io::Result<String> {
    Ok(super::super::get_lines("5.txt")?.next().unwrap())
}

pub fn run() -> io::Result<()> {
    {
        // Part 1
        let mut polymer = Braid::new();
        for c in get_polymer()?.chars() {
            polymer.add(c);
        }
        println!("The polymer's length is {}", polymer.len());
    }
    {
        // Part 2
        let mut polymer = Braid::new();
        let mut components = HashSet::new();
        for c in get_polymer()?.chars() {
            polymer.add(c);
            components.insert(c.to_ascii_lowercase());
        }
        let mut stripped_lengths = HashMap::new();
        for c in components {
            stripped_lengths.insert(c, polymer.clone().strip(c).len());
        }
        let (component, length) = stripped_lengths
            .into_iter()
            .min_by_key(|&(_, length)| length)
            .unwrap();
        println!(
            "The component which most significantly expands the suit is {component} which, when removed, allows it to shrink down to {length} elements",
        );
    }
    Ok(())
}
