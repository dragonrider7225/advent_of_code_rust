use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
};

use nom::{
    bytes::complete as bytes, character::complete as character, combinator as comb, sequence,
    Finish,
};

trait Rng {
    fn next(&mut self) -> u32;
}

#[derive(Clone, Copy, Debug)]
struct DeterministicDie {
    next: u32,
    num_rolls: u32,
}

impl Default for DeterministicDie {
    fn default() -> Self {
        Self {
            next: 1,
            num_rolls: 0,
        }
    }
}

impl Rng for DeterministicDie {
    fn next(&mut self) -> u32 {
        let next = self.next;
        if next >= 100 {
            self.next = 0;
        }
        self.next += 1;
        self.num_rolls += 1;
        next
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Player {
    space: u32,
    score: u32,
}

impl Player {
    fn at(space: u32) -> Self {
        let mut this = Self {
            space,
            ..Self::default()
        };
        if this.space == 0 {
            this.space = 10;
        } else {
            this.space = 1 + ((this.space - 1) % 10)
        }
        this
    }
}

impl Player {
    fn take_turn(&mut self, distance: u32, goal: u32) -> bool {
        self.space += distance;
        if self.space > 10 {
            self.space = 1 + ((self.space - 1) % 10);
        }
        self.score += self.space;
        self.score >= goal
    }
}

impl Default for Player {
    fn default() -> Self {
        Self { space: 1, score: 0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PlayerId {
    Player1,
    Player2,
}

#[derive(Clone, Copy, Debug)]
struct DeterministicGameState {
    p1: Player,
    p2: Player,
    die: DeterministicDie,
}

impl DeterministicGameState {
    fn play_round(&mut self) -> Option<PlayerId> {
        let roll = self.roll();
        if self.p1.take_turn(roll, 1000) {
            Some(PlayerId::Player1)
        } else {
            let roll = self.roll();
            if self.p2.take_turn(roll, 1000) {
                Some(PlayerId::Player2)
            } else {
                None
            }
        }
    }

    fn roll(&mut self) -> u32 {
        self.die.next() + self.die.next() + self.die.next()
    }
}

#[derive(Clone, Debug)]
struct DiracGameState {
    states: HashMap<(Player, Player), u64>,
    /// A swap space for `play_round` to avoid repeatedly allocating then freeing memory.
    blank_states: HashMap<(Player, Player), u64>,
    completed_games: HashMap<PlayerId, u64>,
}

impl DiracGameState {
    fn starting_at(p1: Player, p2: Player) -> Self {
        Self {
            states: HashMap::from_iter([((p1, p2), 1)]),
            blank_states: HashMap::default(),
            completed_games: HashMap::default(),
        }
    }
}

impl DiracGameState {
    fn play_round(&mut self) -> bool {
        assert!(self.blank_states.is_empty());
        let potential_rolls = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let mut states = mem::take(&mut self.states);
        for ((p1, p2), count) in states.drain() {
            for (distance, freq) in potential_rolls.iter().copied() {
                let mut p1 = p1;
                if p1.take_turn(distance, 21) {
                    *self.completed_games.entry(PlayerId::Player1).or_default() += count * freq;
                    continue;
                }
                for (distance, freq) in potential_rolls
                    .iter()
                    .copied()
                    .map(|(dist, freq2)| (dist, freq * freq2))
                {
                    let mut p2 = p2;
                    if p2.take_turn(distance, 21) {
                        *self.completed_games.entry(PlayerId::Player2).or_default() += count * freq;
                    } else {
                        *self.blank_states.entry((p1, p2)).or_default() += count * freq;
                    }
                }
            }
        }
        mem::swap(&mut states, &mut self.blank_states);
        self.states = states;
        self.states.is_empty()
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    input.read_line(&mut buf)?;
    let (p1, p2) = sequence::terminated(
        sequence::separated_pair(
            comb::map(
                sequence::preceded(
                    bytes::tag("Player 1 starting position: "),
                    aoc_util::recognize_u32,
                ),
                Player::at,
            ),
            character::line_ending,
            comb::map(
                sequence::preceded(
                    bytes::tag("Player 2 starting position: "),
                    aoc_util::recognize_u32,
                ),
                Player::at,
            ),
        ),
        comb::opt(character::line_ending),
    )(&buf)
    .finish()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
    .1;
    let mut game = DeterministicGameState {
        p1,
        p2,
        die: DeterministicDie::default(),
    };
    let loser_score = loop {
        break match game.play_round() {
            None => continue,
            Some(PlayerId::Player1) => game.p2.score,
            Some(PlayerId::Player2) => game.p1.score,
        };
    };
    Ok(loser_score * game.die.num_rolls)
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    input.read_line(&mut buf)?;
    let (p1, p2) = sequence::terminated(
        sequence::separated_pair(
            comb::map(
                sequence::preceded(
                    bytes::tag("Player 1 starting position: "),
                    aoc_util::recognize_u32,
                ),
                Player::at,
            ),
            character::line_ending,
            comb::map(
                sequence::preceded(
                    bytes::tag("Player 2 starting position: "),
                    aoc_util::recognize_u32,
                ),
                Player::at,
            ),
        ),
        comb::opt(character::line_ending),
    )(&buf)
    .finish()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
    .1;
    let mut game = DiracGameState::starting_at(p1, p2);
    let mut scores = HashMap::new();
    for i in 0.. {
        scores.clear();
        for ((p1, p2), count) in &game.states {
            *scores.entry((p1.score, p2.score)).or_insert(0u64) += *count;
        }
        println!(
            "Round {}: {} states, {} distinct, {} distinct scores, {}-{}",
            i,
            game.states.values().sum::<u64>(),
            game.states.len(),
            scores.len(),
            game.completed_games
                .get(&PlayerId::Player1)
                .copied()
                .unwrap_or(0),
            game.completed_games
                .get(&PlayerId::Player2)
                .copied()
                .unwrap_or(0)
        );
        if game.play_round() {
            break;
        }
    }
    Ok(*game.completed_games.values().max().unwrap())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 21 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_21.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 21 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_21.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let expected = 739_785;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let expected = 444_356_092_776_315;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
