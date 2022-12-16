use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::RangeInclusive,
    time::Instant,
};

fn manhattan_distance(
    left: (Coordinate, Coordinate),
    right: (Coordinate, Coordinate),
) -> Coordinate {
    (left.0 - right.0).abs() + (left.1 - right.1).abs()
}

type Coordinate = i32;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Sensor {
    pos: (Coordinate, Coordinate),
    beacon: (Coordinate, Coordinate),
}

impl Sensor {
    fn distance_to_beacon(&self) -> Coordinate {
        self.distance_to(self.beacon)
    }

    fn distance_to(&self, pos: (Coordinate, Coordinate)) -> Coordinate {
        manhattan_distance(self.pos, pos)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SensorData {
    sensor_responses: HashSet<Sensor>,
}

impl SensorData {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let sensor_responses = input
            .lines()
            .map(|line| {
                let line = line?;
                if let Some(tail) = line.strip_prefix("Sensor at x=") {
                    let (sensor_coords, beacon_coords) = tail
                        .split_once(": closest beacon is at x=")
                        .ok_or_else(|| {
                            io::Error::new(io::ErrorKind::InvalidData, "Missing separator")
                        })?;
                    let (sensor_x, sensor_y) =
                        sensor_coords.split_once(", y=").ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid sensor_coords: x={sensor_coords}"),
                            )
                        })?;
                    let sensor_x = sensor_x.parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid sensor x-coordinate {sensor_x:?}: {e:?}"),
                        )
                    })?;
                    let sensor_y = sensor_y.parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid sensor y-coordinate {sensor_y:?}: {e:?}"),
                        )
                    })?;
                    let (beacon_x, beacon_y) =
                        beacon_coords.split_once(", y=").ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid beacon_coords x={beacon_coords}"),
                            )
                        })?;
                    let beacon_x = beacon_x.parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid beacon x-coordinate {beacon_x:?}: {e:?}"),
                        )
                    })?;
                    let beacon_y = beacon_y.parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid beacon y-coordinate {beacon_y:?}: {e:?}"),
                        )
                    })?;
                    Ok(Sensor {
                        pos: (sensor_x, sensor_y),
                        beacon: (beacon_x, beacon_y),
                    })
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Missing line prefix",
                    ))
                }
            })
            .collect::<io::Result<_>>()?;
        Ok(Self { sensor_responses })
    }

    fn non_beacon_count(&self, y: Coordinate) -> usize {
        let beacon_xs = self
            .sensor_responses
            .iter()
            .filter_map(|&sensor| {
                if sensor.beacon.1 == y {
                    Some(sensor.beacon.0)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        let mut local_xs = self
            .sensor_responses
            .iter()
            .filter_map(|sensor| {
                let total_distance = sensor.distance_to_beacon();
                let target_vertical_distance = (y - sensor.pos.1).abs();
                if total_distance > target_vertical_distance {
                    let max_horizontal_distance = total_distance - target_vertical_distance;
                    Some(
                        (-max_horizontal_distance..=max_horizontal_distance)
                            .map(|dx| sensor.pos.0 + dx),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect::<HashSet<_>>();
        for beacon_x in beacon_xs {
            local_xs.remove(&beacon_x);
        }
        local_xs.len()
    }

    fn missing_beacon_tuning_frequency(
        &self,
        xs: RangeInclusive<Coordinate>,
        ys: RangeInclusive<Coordinate>,
    ) -> i64 {
        let start = Instant::now();
        let sensor_distances = self
            .sensor_responses
            .iter()
            .map(|&sensor| (sensor, sensor.distance_to_beacon()))
            .collect::<HashMap<_, _>>();
        let mut last = start;
        for x in xs {
            if x % 1_000_000 == 0 {
                let now = Instant::now();
                dbg!(x, now - start, now - last);
                last = now;
            }
            let mut y = *ys.start();
            while y <= *ys.end() {
                if sensor_distances
                    .iter()
                    .all(|(&sensor, &distance)| sensor.distance_to((x, y)) > distance)
                {
                    return 4_000_000 * x as i64 + y as i64;
                }
                let next_y = sensor_distances
                    .iter()
                    .filter(|(&sensor, &distance)| sensor.distance_to((x, y)) <= distance)
                    .map(|(&sensor, &distance)| sensor.pos.1 + distance - (x - sensor.pos.0).abs())
                    .max()
                    .unwrap();
                y = next_y + 1;
            }
        }
        panic!("No uncovered position found");
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let sensor_data = SensorData::read(input)?;
    Ok(sensor_data.non_beacon_count(2_000_000))
}

fn part2(input: &mut dyn BufRead) -> io::Result<i64> {
    let sensor_data = SensorData::read(input)?;
    dbg!("Sensor data parsed");
    Ok(sensor_data.missing_beacon_tuning_frequency(0..=4_000_000, 0..=4_000_000))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 15 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_15.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 15 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_15.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3\n",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16\n",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16\n",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16\n",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10\n",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10\n",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17\n",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22\n",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3\n",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 26;
        let actual = SensorData::read(&mut Cursor::new(TEST_DATA))?.non_beacon_count(10);
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 56_000_011;
        let actual = SensorData::read(&mut Cursor::new(TEST_DATA))?
            .missing_beacon_tuning_frequency(0..=20, 0..=20);
        assert_eq!(expected, actual);
        Ok(())
    }
}
