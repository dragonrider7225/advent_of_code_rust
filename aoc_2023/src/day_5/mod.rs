use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{
    bytes::complete as bytes, character::complete as character, combinator, multi, sequence,
    IResult,
};

type Id = u64;

fn id_nom_parse(s: &str) -> IResult<&str, Id> {
    character::u64(s)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SeedList(Vec<Id>);

impl SeedList {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            sequence::delimited(
                bytes::tag("seeds: "),
                multi::separated_list1(bytes::tag(" "), id_nom_parse),
                aoc_util::newline,
            ),
            Self,
        )(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MapRange {
    from: Id,
    to: Id,
    len: Id,
}

impl MapRange {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            sequence::tuple((
                sequence::terminated(id_nom_parse, bytes::tag(" ")),
                sequence::terminated(id_nom_parse, bytes::tag(" ")),
                sequence::terminated(id_nom_parse, aoc_util::newline),
            )),
            |(to, from, len)| Self { to, from, len },
        )(s)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Map(Vec<MapRange>);

impl Map {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(multi::many0(MapRange::nom_parse), |mut ranges| {
            ranges.sort_by_key(|range| range.from);
            Self(ranges)
        })(s)
    }

    fn get(&self, from: Id) -> Id {
        self.0
            .iter()
            .find_map(|range| {
                if (range.from..(range.from + range.len)).contains(&from) {
                    Some(range.to + (from - range.from))
                } else {
                    None
                }
            })
            .unwrap_or(from)
    }

    fn get_range(&self, (mut least_from, mut range_len): (Id, u64)) -> Vec<(Id, u64)> {
        let mut out = Vec::with_capacity(self.0.len());
        for &range in &self.0 {
            if least_from < range.from {
                let flat_len = range_len.min(range.from - least_from);
                out.push((least_from, flat_len));
                least_from += flat_len;
                range_len -= flat_len;
                if range_len == 0 {
                    break;
                }
            }
            if least_from < range.from + range.len {
                let num_skipped = least_from - range.from;
                let num_taken = range_len.min(range.len - num_skipped);
                out.push((range.to + num_skipped, num_taken));
                least_from += num_taken;
                range_len -= num_taken;
                if range_len == 0 {
                    break;
                }
            }
        }
        if range_len != 0 {
            out.push((least_from, range_len));
        }
        out
    }
}

macro_rules! mk_maps {
    ($($struct_name:ident $header:literal;)*) => {
        $(
            #[derive(Clone, Debug, Eq, PartialEq)]
            struct $struct_name(Map);

            impl $struct_name {
                fn nom_parse(s: &str) -> IResult<&str, Self> {
                    combinator::map(
                        sequence::preceded(
                            sequence::pair(
                                bytes::tag(concat!($header, " map:")),
                                aoc_util::newline,
                            ),
                            Map::nom_parse,
                        ),
                        Self,
                    )(s)
                }
            }
        )*
    };
}

mk_maps!(
    SeedToSoilMap "seed-to-soil";
    SoilToFertilizerMap "soil-to-fertilizer";
    FertilizerToWaterMap "fertilizer-to-water";
    WaterToLightMap "water-to-light";
    LightToTemperatureMap "light-to-temperature";
    TemperatureToHumidityMap "temperature-to-humidity";
    HumidityToLocationMap "humidity-to-location";
);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Almanac {
    seeds: Vec<Id>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn read_from(input: &mut dyn BufRead) -> io::Result<Self> {
        let input = {
            let mut line = String::new();
            input.read_to_string(&mut line)?;
            io::Result::Ok(line)
        }?;
        Self::nom_parse(&input)
            .map(|(_, almanac)| almanac)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            sequence::tuple((
                sequence::terminated(SeedList::nom_parse, aoc_util::newline),
                sequence::terminated(SeedToSoilMap::nom_parse, aoc_util::newline),
                sequence::terminated(SoilToFertilizerMap::nom_parse, aoc_util::newline),
                sequence::terminated(FertilizerToWaterMap::nom_parse, aoc_util::newline),
                sequence::terminated(WaterToLightMap::nom_parse, aoc_util::newline),
                sequence::terminated(LightToTemperatureMap::nom_parse, aoc_util::newline),
                sequence::terminated(TemperatureToHumidityMap::nom_parse, aoc_util::newline),
                HumidityToLocationMap::nom_parse,
            )),
            |almanac| Self {
                seeds: almanac.0 .0,
                seed_to_soil: almanac.1 .0,
                soil_to_fertilizer: almanac.2 .0,
                fertilizer_to_water: almanac.3 .0,
                water_to_light: almanac.4 .0,
                light_to_temperature: almanac.5 .0,
                temperature_to_humidity: almanac.6 .0,
                humidity_to_location: almanac.7 .0,
            },
        )(s)
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<Id> {
    let almanac = Almanac::read_from(input)?;
    Ok(almanac
        .seeds
        .iter()
        .copied()
        .map(|seed| almanac.seed_to_soil.get(seed))
        .map(|soil| almanac.soil_to_fertilizer.get(soil))
        .map(|fertilizer| almanac.fertilizer_to_water.get(fertilizer))
        .map(|water| almanac.water_to_light.get(water))
        .map(|light| almanac.light_to_temperature.get(light))
        .map(|temperature| almanac.temperature_to_humidity.get(temperature))
        .map(|humidity| almanac.humidity_to_location.get(humidity))
        .min()
        .expect("Almanac contains no seeds"))
}

fn part2(input: &mut dyn BufRead) -> io::Result<Id> {
    let almanac = Almanac::read_from(input)?;
    let seed_ranges = almanac.seeds.chunks(2).fold(
        Vec::with_capacity(almanac.seeds.len() / 2),
        |mut acc, chunk| {
            acc.push((chunk[0], chunk[1]));
            acc
        },
    );
    Ok(seed_ranges
        .into_iter()
        .flat_map(|range| almanac.seed_to_soil.get_range(range))
        .flat_map(|range| almanac.soil_to_fertilizer.get_range(range))
        .flat_map(|range| almanac.fertilizer_to_water.get_range(range))
        .flat_map(|range| almanac.water_to_light.get_range(range))
        .flat_map(|range| almanac.light_to_temperature.get_range(range))
        .flat_map(|range| almanac.temperature_to_humidity.get_range(range))
        .flat_map(|range| almanac.humidity_to_location.get_range(range))
        .min_by_key(|range| range.0)
        .expect("Almanac contained no seeds")
        .0)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 5 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_05.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 5 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_05.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "seeds: 79 14 55 13\n",
        "\n",
        "seed-to-soil map:\n",
        "50 98 2\n",
        "52 50 48\n",
        "\n",
        "soil-to-fertilizer map:\n",
        "0 15 37\n",
        "37 52 2\n",
        "39 0 15\n",
        "\n",
        "fertilizer-to-water map:\n",
        "49 53 8\n",
        "0 11 42\n",
        "42 0 7\n",
        "57 7 4\n",
        "\n",
        "water-to-light map:\n",
        "88 18 7\n",
        "18 25 70\n",
        "\n",
        "light-to-temperature map:\n",
        "45 77 23\n",
        "81 45 19\n",
        "68 64 13\n",
        "\n",
        "temperature-to-humidity map:\n",
        "0 69 1\n",
        "1 0 69\n",
        "\n",
        "humidity-to-location map:\n",
        "60 56 37\n",
        "56 93 4\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 35;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 46;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
