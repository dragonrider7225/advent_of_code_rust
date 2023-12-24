use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
    ops::Index,
    rc::Rc,
    sync::atomic::AtomicUsize,
};

use aoc_util::{geometry::Point2D, nom_extended::NomParse};
use nom::{branch, bytes::complete as bytes, combinator, multi};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    NorthSlope,
    EastSlope,
    SouthSlope,
    WestSlope,
}

impl<'s> NomParse<&'s str> for Tile {
    fn nom_parse(input: &'s str) -> nom::IResult<&'s str, Self> {
        branch::alt((
            combinator::value(Self::Path, bytes::tag(".")),
            combinator::value(Self::Forest, bytes::tag("#")),
            combinator::value(Self::NorthSlope, bytes::tag("^")),
            combinator::value(Self::EastSlope, bytes::tag(">")),
            combinator::value(Self::SouthSlope, bytes::tag("v")),
            combinator::value(Self::WestSlope, bytes::tag("<")),
        ))(input)
    }
}

type Position = Point2D<usize>;

#[derive(Clone, Debug)]
enum LinkedList<T> {
    Empty,
    Cell(T, Rc<Self>),
}

impl<T> LinkedList<T>
where
    for<'a> &'a T: PartialEq,
{
    fn contains(&self, position: &T) -> bool {
        let mut head = self;
        while let Self::Cell(value, tail) = head {
            if value == position {
                return true;
            }
            head = Rc::as_ref(tail);
        }
        false
    }
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    fn get(&self, idx: usize) -> Option<T> {
        let mut head = self;
        let mut i = 0;
        while let Self::Cell(p, tail) = head {
            if i == idx {
                return Some(p.clone());
            }
            head = Rc::as_ref(tail);
            i += 1;
        }
        None
    }
}

impl<T> LinkedList<T> {
    fn len(&self) -> usize {
        let mut head = self;
        let mut i = 0;
        while let Self::Cell(_, tail) = head {
            head = tail;
            i += 1;
        }
        i
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter()
            .fold(Self::Empty, |acc, pos| Self::Cell(pos, Rc::new(acc)))
    }
}

impl<T> Iterator for LinkedList<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match mem::replace(self, Self::Empty) {
            Self::Empty => None,
            Self::Cell(head, tail) => {
                *self = Rc::unwrap_or_clone(tail);
                Some(head)
            }
        }
    }
}

type History = LinkedList<Position>;

#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    /// Get those neighbors of `pos` which are both non-Forest and not an untraversable slope from
    /// `pos`.
    fn neighbors(&self, pos: Position, history: History) -> Vec<(Position, History)> {
        let history = Rc::new(history);
        let mut ret = vec![];
        if *pos.x() > 0 {
            let next_pos = pos - Point2D::at(1, 0);
            if matches!(
                self.tiles[*next_pos.y()][*next_pos.x()],
                Tile::WestSlope | Tile::Path
            ) && !history.contains(&next_pos)
            {
                ret.push((next_pos, History::Cell(next_pos, Rc::clone(&history))));
            }
        }
        if *pos.y() > 0 {
            let next_pos = pos - Point2D::at(0, 1);
            if matches!(
                self.tiles[*next_pos.y()][*next_pos.x()],
                Tile::NorthSlope | Tile::Path
            ) && !history.contains(&next_pos)
            {
                ret.push((next_pos, History::Cell(next_pos, Rc::clone(&history))));
            }
        }
        if *pos.x() + 1 < self.tiles[0].len() {
            let next_pos = pos + Point2D::at(1, 0);
            if matches!(
                self.tiles[*next_pos.y()][*next_pos.x()],
                Tile::EastSlope | Tile::Path
            ) && !history.contains(&next_pos)
            {
                ret.push((next_pos, History::Cell(next_pos, Rc::clone(&history))));
            }
        }
        if *pos.y() + 1 < self.tiles.len() {
            let next_pos = pos + Point2D::at(0, 1);
            if matches!(
                self.tiles[*next_pos.y()][*next_pos.x()],
                Tile::SouthSlope | Tile::Path
            ) && !history.contains(&next_pos)
            {
                ret.push((next_pos, History::Cell(next_pos, Rc::clone(&history))));
            }
        }
        ret
    }

    /// Get those non-forest neighbors of `pos` that have not already been visited.
    fn neighbors_shallow(&self, pos: Position, history: History) -> Vec<(Position, History)> {
        let history = Rc::new(history);
        let mut ret = vec![];
        let mut add_step = |next_pos: Position| {
            if !matches!(self.tiles[*next_pos.y()][*next_pos.x()], Tile::Forest)
                && !history.contains(&next_pos)
            {
                ret.push((next_pos, History::Cell(next_pos, Rc::clone(&history))));
            }
        };
        if *pos.x() > 0 {
            let next_pos = pos - Point2D::at(1, 0);
            add_step(next_pos);
        }
        if *pos.y() > 0 {
            let next_pos = pos - Point2D::at(0, 1);
            add_step(next_pos);
        }
        if *pos.x() + 1 < self.tiles[0].len() {
            let next_pos = pos + Point2D::at(1, 0);
            add_step(next_pos);
        }
        if *pos.y() + 1 < self.tiles.len() {
            let next_pos = pos + Point2D::at(0, 1);
            add_step(next_pos);
        }
        ret
    }

    /// Get the non-forest neighbors of `pos`.
    fn neighbors_agnostic(&self, pos: Position) -> Vec<Position> {
        let mut ret = vec![];
        if *pos.y() > 0 {
            let new_pos = Position::at(*pos.x(), *pos.y() - 1);
            if self[new_pos] != Tile::Forest {
                ret.push(new_pos);
            }
        }
        if *pos.x() > 0 {
            let new_pos = Position::at(*pos.x() - 1, *pos.y());
            if self[new_pos] != Tile::Forest {
                ret.push(new_pos);
            }
        }
        if *pos.y() + 1 < self.tiles.len() {
            let new_pos = Position::at(*pos.x(), *pos.y() + 1);
            if self[new_pos] != Tile::Forest {
                ret.push(new_pos);
            }
        }
        if *pos.x() + 1 < self.tiles[0].len() {
            let new_pos = Position::at(*pos.x() + 1, *pos.y());
            if self[new_pos] != Tile::Forest {
                ret.push(new_pos);
            }
        }
        ret
    }
}

impl FromIterator<Vec<Tile>> for Map {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Vec<Tile>>,
    {
        Self {
            tiles: iter.into_iter().collect(),
        }
    }
}

impl Index<Position> for Map {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self.tiles[*index.y()][*index.x()]
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct NodeId(usize);

impl NodeId {
    fn new() -> Self {
        use std::sync::atomic::Ordering;
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        Self(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Node{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
struct Connection {
    node1: NodeId,
    node2: NodeId,
    length: usize,
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: Vec<NodeId>,
    connections: Vec<Connection>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: vec![],
            connections: vec![],
        }
    }

    fn neighbors(&self, n: NodeId) -> Vec<NodeId> {
        self.connections
            .iter()
            .filter_map(|&connection| match connection {
                Connection { node1, node2, .. } if n == node1 => Some(node2),
                Connection { node1, node2, .. } if n == node2 => Some(node1),
                _ => None,
            })
            .collect()
    }

    fn get_length(&self, mut node1: NodeId, mut node2: NodeId) -> Option<usize> {
        if node1 > node2 {
            mem::swap(&mut node1, &mut node2);
        }
        self.connections.iter().find_map(|connection| {
            if connection.node1 == node1 && connection.node2 == node2 {
                Some(connection.length)
            } else {
                None
            }
        })
    }

    fn add_nodes<I>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = NodeId>,
    {
        self.nodes.extend(nodes);
        self.nodes.sort_unstable();
        self.nodes.dedup();
    }

    fn connect(&mut self, mut node1: NodeId, mut node2: NodeId, length: usize) {
        if node1 > node2 {
            mem::swap(&mut node1, &mut node2);
        }
        if !self.nodes.contains(&node1) {
            self.nodes.push(node1);
        }
        if !self.nodes.contains(&node2) {
            self.nodes.push(node2);
        }
        if let Some(connection) = self
            .connections
            .iter_mut()
            .find(|connection| connection.node1 == node1 && connection.node2 == node2)
        {
            connection.length = connection.length.max(length);
        } else {
            self.connections.push(Connection {
                node1,
                node2,
                length,
            });
        }
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = input
        .lines()
        .map(|line| {
            let line = line?;
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Tile::nom_parse)(&line)
                .map(|(_, x)| x)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
            ret
        })
        .collect::<io::Result<Map>>()?;
    let starting_point = Position::at(
        map.tiles[0]
            .iter()
            .position(|&tile| tile == Tile::Path)
            .expect("Top row missing path tile"),
        0,
    );
    let mut states = vec![(starting_point, History::from_iter([starting_point]))];
    let mut current_length = 0;
    let mut max_hike_length = 0;
    let mut hike_length_count = 0usize;
    while !states.is_empty() {
        states = states
            .into_iter()
            .fold(vec![], |mut new_states, (current_pos, history)| {
                if current_pos.y() + 1 == map.tiles.len() {
                    if current_length != max_hike_length {
                        max_hike_length = current_length;
                        hike_length_count = 0;
                    }
                    hike_length_count += 1;
                } else {
                    new_states.extend(map.neighbors(current_pos, history));
                }
                new_states
            });
        current_length += 1;
    }
    Ok(max_hike_length)
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = input
        .lines()
        .map(|line| {
            let line = line?;
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Tile::nom_parse)(&line)
                .map(|(_, x)| x)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
            ret
        })
        .collect::<io::Result<Map>>()?;
    let map_ref = &map;
    let mut intersections = map
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter_map(move |(col_idx, tile)| {
                    let neighbors = map_ref.neighbors_agnostic(Position::at(col_idx, row_idx));
                    if tile != Tile::Forest
                        && (row_idx == 0
                            || row_idx + 1 == map_ref.tiles.len()
                            || neighbors.len() > 2)
                    {
                        Some((NodeId::new(), Position::at(col_idx, row_idx), neighbors))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();
    let mut graph = Graph::new();
    graph.add_nodes(intersections.iter().map(|&(node_id, ..)| node_id));
    for i in 0..intersections.len() {
        let (node_id, position, neighbors) = intersections[i].clone();
        let start_history = Rc::new(History::Cell(position, Rc::new(History::Empty)));
        for mut neighbor in neighbors {
            let mut history = History::Cell(neighbor, Rc::clone(&start_history));
            loop {
                let mut neighbors = map.neighbors_shallow(neighbor, history.clone());
                match neighbors.len() {
                    0 => break,
                    1 => (neighbor, history) = neighbors.pop().unwrap(),
                    _ => {
                        let (&mut node2, later_neighbors) = intersections
                            .iter_mut()
                            .find_map(|(node, position, neighbors)| {
                                if *position == neighbor {
                                    Some((node, neighbors))
                                } else {
                                    None
                                }
                            })
                            .expect("Didn't find all intersections");
                        graph.connect(node_id, node2, history.len() - 1);
                        let from = history.get(1).unwrap();
                        let idx = later_neighbors
                            .iter()
                            .position(|&p| p == from)
                            .expect("Reached intersection from non-existent neighbor");
                        later_neighbors.remove(idx);
                        break;
                    }
                }
            }
        }
    }
    let entrance = intersections[0].0;
    let exit = intersections.last().unwrap().0;
    let mut states = vec![(entrance, LinkedList::from_iter([entrance]))];
    let mut max_hike_length = 0;
    while !states.is_empty() {
        states = states
            .into_iter()
            .fold(vec![], |mut new_states, (current_pos, path)| {
                if current_pos == exit {
                    let total_length = path
                        .clone()
                        .map_windows::<_, _, 2>(|&[b, a]| graph.get_length(a, b))
                        .flatten()
                        .sum();
                    max_hike_length = max_hike_length.max(total_length);
                    return new_states;
                }
                let path = Rc::new(path);
                new_states.extend(
                    graph
                        .neighbors(current_pos)
                        .into_iter()
                        .filter(|neighbor| !path.contains(neighbor))
                        .map(|neighbor| (neighbor, LinkedList::Cell(neighbor, Rc::clone(&path)))),
                );
                new_states
            });
    }
    Ok(max_hike_length)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 23 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_23.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 23 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_23.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "#.#####################\n",
        "#.......#########...###\n",
        "#######.#########.#.###\n",
        "###.....#.>.>.###.#.###\n",
        "###v#####.#v#.###.#.###\n",
        "###.>...#.#.#.....#...#\n",
        "###v###.#.#.#########.#\n",
        "###...#.#.#.......#...#\n",
        "#####.#.#.#######.#.###\n",
        "#.....#.#.#.......#...#\n",
        "#.#####.#.#.#########v#\n",
        "#.#...#...#...###...>.#\n",
        "#.#.#v#######v###.###v#\n",
        "#...#.>.#...>.>.#.###.#\n",
        "#####v#.#.###v#.#.###.#\n",
        "#.....#...#...#.#.#...#\n",
        "#.#########.###.#.#.###\n",
        "#...###...#...#...#.###\n",
        "###.###.#.###v#####v###\n",
        "#...#...#.#.>.>.#.>.###\n",
        "#.###.###.#.###.#.#v###\n",
        "#.....###...###...#...#\n",
        "#####################.#\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 94;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 154;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
