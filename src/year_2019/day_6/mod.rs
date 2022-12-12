use std::io;

struct Body {
    name: String,
    orbiters: Vec<Body>,
}

impl Body {
    fn new(name: String) -> Body {
        Body {
            name,
            orbiters: vec![],
        }
    }

    fn count_orbits(&self, num_parents: u32) -> u32 {
        let mut ret = num_parents;
        for orbiter in self.orbiters.iter() {
            ret += orbiter.count_orbits(num_parents + 1);
        }
        ret
    }

    fn num_orbits(&self) -> u32 {
        self.count_orbits(0)
    }

    fn distances(
        &self,
        mut name1: Option<&str>,
        mut name2: Option<&str>,
    ) -> (Option<u32>, Option<u32>) {
        let mut ret = (None, None);
        if name1.is_none() && name2.is_none() {
            return ret;
        }
        for orbiter in self.orbiters.iter() {
            if name1.filter(|&n| n == orbiter.name).is_some() {
                return (Some(0), None);
            } else if name2.filter(|&n| n == orbiter.name).is_some() {
                return (None, Some(0));
            } else {
                let (dist1, dist2) = orbiter.distances(name1, name2);
                if dist1.is_some() && dist2.is_some() {
                    return (dist1, dist2);
                }
                if let Some(dist1) = dist1 {
                    ret.0 = Some(dist1 + 1);
                    name1 = None;
                }
                if let Some(dist2) = dist2 {
                    ret.1 = Some(dist2 + 1);
                    name2 = None;
                }
            }
            if ret.0.is_some() && ret.1.is_some() {
                break;
            }
        }
        ret
    }

    fn distance_from(&self, other: &str) -> Option<u32> {
        match self.distances(Some("YOU"), Some(other)) {
            (Some(you), Some(other)) => Some(you + other),
            _ => None,
        }
    }

    fn add(&mut self, parent: String, child: String) -> bool {
        if self.name == parent {
            self.orbiters.push(Self::new(child));
            true
        } else {
            for orbiter in self.orbiters.iter_mut() {
                if orbiter.add(parent.clone(), child.clone()) {
                    return true;
                }
            }
            false
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new("COM".to_string())
    }
}

fn get_orbits() -> io::Result<Body> {
    let mut orbits: Box<dyn Iterator<Item = (String, String)>> =
        box crate::get_lines("2019_6.txt")?.map(|s| {
            let mut strs = s.split(')').map(|s| s.to_owned());
            let parent = strs.next().unwrap();
            let child = strs.next().unwrap();
            (parent, child)
        });
    let mut com = Body::default();
    loop {
        let mut missed = vec![];
        for (parent, child) in orbits {
            if !com.add(parent.clone(), child.clone()) {
                missed.push((parent, child));
            }
        }
        if missed.is_empty() {
            break;
        } else {
            orbits = box missed.into_iter();
        }
    }
    Ok(com)
}

pub(super) fn run() -> io::Result<()> {
    println!("Building map...");
    let com = get_orbits()?;
    {
        println!("Year 2019 Day 6 Part 1");
        println!("There are {} orbits", com.num_orbits());
    }
    {
        println!("Year 2019 Day 6 Part 2");
        println!(
            "You are {} transfers away from Santa",
            com.distance_from("SAN").unwrap(),
        );
    }
    Ok(())
}
