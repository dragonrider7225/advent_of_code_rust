use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

struct IntersperseIter<I, T> {
    iter: I,
    next: Option<T>,
    separator: T,
    done: bool,
}

impl<I, T> IntersperseIter<I, T>
where
    I: Iterator<Item = T>,
{
    fn new<II>(iter: II, separator: T) -> Self
    where
        II: IntoIterator<Item = T, IntoIter = I>,
    {
        let mut iter = iter.into_iter();
        let next = iter.next();
        Self {
            iter,
            next,
            separator,
            done: false,
        }
    }
}

impl<I, T> Iterator for IntersperseIter<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            match self.next.take() {
                None => match self.iter.next() {
                    None => {
                        self.done = true;
                        None
                    }
                    Some(next) => {
                        self.next = Some(next);
                        Some(self.separator.clone())
                    }
                },
                Some(next) => Some(next),
            }
        }
    }
}

trait Intersperse<T>: Iterator<Item = T>
where
    T: Clone,
    Self: Sized,
{
    fn intersperse_local(self, separator: T) -> IntersperseIter<Self, T>;
}

impl<T, I> Intersperse<T> for I
where
    T: Clone,
    I: Iterator<Item = T>,
{
    fn intersperse_local(self, separator: T) -> IntersperseIter<I, T> {
        IntersperseIter::new(self, separator)
    }
}

fn read_allergens(
    input: &mut dyn BufRead,
) -> io::Result<HashMap<BTreeSet<String>, HashSet<String>>> {
    input.lines().try_fold(HashMap::new(), |mut acc, line| {
        // `line?` is of the form `"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"` where
        // `mxmxvkd`, `kfcds`, `sqjhc`, and `nhms` are the ingredients and `dairy` and `fish` are
        // the marked allergens.
        //
        // Thus `ingredients` is of the form `"mxmxvkd kfcds sqjhc nhms" and `allergens` is of the
        // form `"dairy, fish)"`.
        let line = line?;
        let (ingredients, allergens) = line
            .split_once(" (contains ")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing allergen list"))?;
        // This can't be a HashSet because HashSet doesn't implement Hash for some reason.
        let ingredients = ingredients
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect::<BTreeSet<_>>();
        let allergens = allergens[..(allergens.len() - 1)]
            .split(", ")
            .map(ToOwned::to_owned)
            .collect::<HashSet<_>>();
        assert!(acc.insert(ingredients, allergens).is_none());
        Ok(acc)
    })
}

/// Returns a set of ingredients that don't contain any allergens and a map from allergen to the
/// ingredient that contains it.
fn separate_allergens(
    ingredients: &HashMap<BTreeSet<String>, HashSet<String>>,
) -> io::Result<(HashSet<String>, HashMap<String, String>)> {
    let known_allergens = ingredients;
    let mut potential_sources = known_allergens.iter().fold(
        HashMap::<_, HashSet<String>>::new(),
        |mut acc, (ingredients, allergens)| {
            for allergen in allergens {
                acc.entry(allergen.clone())
                    .or_insert_with(|| ingredients.iter().cloned().collect())
                    .retain(|ingredient| ingredients.contains(ingredient));
            }
            acc
        },
    );
    let mut ingredients: HashSet<_> = known_allergens
        .keys()
        .flat_map(IntoIterator::into_iter)
        .map(ToOwned::to_owned)
        .collect();
    let mut actual_sources = HashMap::new();
    let mut allergens_to_remove = HashSet::new();
    let mut ingredients_to_remove = HashSet::new();
    while !potential_sources.is_empty() {
        for (allergen, possible_ingredients) in &potential_sources {
            match possible_ingredients.len() {
                0 => {
                    println!("{allergen} has no more potential sources, removing it");
                    allergens_to_remove.insert(allergen.to_owned());
                }
                1 => {
                    let ingredient = possible_ingredients.iter().next().unwrap().to_owned();
                    println!("Found out that {ingredient} contains {allergen}");
                    actual_sources.insert(allergen.clone(), ingredient.clone());
                    ingredients_to_remove.insert(ingredient);
                }
                _ => {}
            }
        }
        let modified = allergens_to_remove
            .drain()
            // non-short-circuit `.any`.
            .fold(false, |acc, allergen| {
                potential_sources.remove(&allergen).is_some() || acc
            });
        let modified = ingredients_to_remove.drain().any(|ingredient| {
            ingredients.remove(&ingredient);
            potential_sources
                .iter_mut()
                // non-short-circuit `.any`.
                .fold(false, |acc, (_, possible_ingredients)| {
                    possible_ingredients.remove(&ingredient) || acc
                })
        }) || modified;
        if !modified {
            let remaining_potentials = potential_sources
                .into_iter()
                .flat_map(|(_, ingredients)| ingredients)
                .collect::<HashSet<_>>();
            ingredients.extract_if(|ingredient| remaining_potentials.contains(ingredient));
            break;
        }
    }
    Ok((ingredients, actual_sources))
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let ingredients = read_allergens(input)?;
    let (clean_ingredients, _) = separate_allergens(&ingredients)?;
    Ok(ingredients
        .keys()
        .flat_map(|ingredients| {
            ingredients
                .iter()
                .filter(|&ingredient| clean_ingredients.contains(ingredient))
        })
        .count())
}

fn part2(input: &mut dyn BufRead) -> io::Result<String> {
    let ingredients = read_allergens(input)?;
    let (_, allergens) = separate_allergens(&ingredients)?;
    let mut allergens = allergens.into_iter().collect::<Vec<_>>();
    allergens.sort_by_key(|(allergen, _)| allergen.clone());
    Ok(allergens
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .intersperse_local(",".to_owned())
        .collect())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2020 Day 21 Part 1");
        println!(
            "There are {} occurrences of ingredients that definitely do not contain any relevant allergens",
            part1(&mut BufReader::new(File::open("2020_21.txt")?))?,
        );
    }
    {
        println!("Year 2020 Day 21 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2020_21.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_part1() -> io::Result<()> {
        let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        let expected = 5;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        let expected = "mxmxvkd,sqjhc,fvjkl";
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn empty_intersperse_stays_empty() {
        let mut iter = std::iter::empty().intersperse_local(());
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn intersperse_works() {
        let mut iter = (1..=3).intersperse_local(5);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}
