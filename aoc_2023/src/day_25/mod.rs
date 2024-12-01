use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader, Write as _},
    ops::Range,
    path::Path,
    process::{Command, ExitStatus, Stdio},
    str::FromStr,
};

macro_rules! read_line {
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).map(|_| line)
    }};
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
    Orange,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red => write!(f, "red"),
            Self::Blue => write!(f, "blue"),
            Self::Green => write!(f, "green"),
            Self::Orange => write!(f, "orange"),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "orange" => Ok(Self::Orange),
            _ => Err(()),
        }
    }
}

struct Edge {
    left: String,
    right: String,
    color: Option<Color>,
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -- {}", self.left, self.right)?;
        if let Some(c) = self.color {
            write!(f, r#"[ color = "{c}" ]"#)?;
        }
        write!(f, ";")
    }
}

struct Graph {
    name: String,
    edges: Vec<Edge>,
    red_range: Option<Range<usize>>,
    blue_range: Option<Range<usize>>,
    green_range: Option<Range<usize>>,
}

impl Graph {
    fn new(name: String, edges: Vec<Edge>) -> Self {
        let mut ret = Self {
            name,
            edges,
            red_range: None,
            blue_range: None,
            green_range: None,
        };
        ret.color_edges(0..ret.edges.len());
        ret
    }

    fn uncolor_range(&mut self, range: Range<usize>) {
        for edge in &mut self.edges[range] {
            edge.color.take_if(|color| !matches!(color, Color::Orange));
        }
    }

    fn color_range(&mut self, range: Range<usize>, color: Color) {
        for edge in &mut self.edges[range.clone()] {
            match edge.color.as_mut() {
                Some(Color::Orange) => {}
                Some(c) => *c = color,
                None => edge.color = Some(color),
            }
        }
    }

    fn color_edges(&mut self, range: Range<usize>) {
        let start = range.start;
        let end = range.end;
        let num_edges = end - start;
        let red_end = start + num_edges / 3;
        let red_range = start..red_end;
        let blue_end = red_end + (num_edges - red_range.len()) / 2;
        let blue_range = red_end..blue_end;
        let green_range = blue_end..end;
        self.uncolor_range(0..start);
        self.color_range(red_range.clone(), Color::Red);
        self.red_range = Some(red_range);
        self.color_range(blue_range.clone(), Color::Blue);
        self.blue_range = Some(blue_range);
        self.color_range(green_range.clone(), Color::Green);
        self.green_range = Some(green_range);
        self.uncolor_range(end..self.edges.len());
    }

    fn write_to_file(&self, filename: impl AsRef<Path>) -> io::Result<ExitStatus> {
        let mut grapher = Command::new("dot")
            .arg("-Tsvg")
            .stdin(Stdio::piped())
            .stdout(File::create(filename)?)
            .spawn()?;
        write!(grapher.stdin.take().unwrap(), "{self}")?;
        grapher.wait()
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"graph "{}" {{"#, self.name)?;
        self.edges.iter().try_for_each(|edge| write!(f, "{edge}"))?;
        write!(f, "}}")
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut edges = input
        .lines()
        .map(|line| {
            let line = line?;
            let (a, b) = line.trim().split_once(": ").ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Line {line:?} didn't contain colon",
                )
            })?;
            let left = a.to_string();
            Ok(b.split_ascii_whitespace()
                .map(|right| (left.clone(), right.to_string()))
                .collect::<Vec<_>>())
        })
        .try_fold::<_, _, io::Result<_>>(vec![], |mut acc, pairs: io::Result<_>| {
            acc.extend(pairs?);
            Ok(acc)
        })?;
    edges.sort_unstable();
    let mut graph = Graph::new(
        "2023_25".to_string(),
        edges
            .into_iter()
            .map(|(left, right)| Edge {
                left,
                right,
                color: None,
            })
            .collect(),
    );
    print!("Enter filename for graph: ");
    io::stdout().flush()?;
    let graph_filename = read_line!()?.trim().to_string();
    if !graph.write_to_file(&graph_filename)?.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Couldn't write graph to {graph_filename:?}"),
        ));
    }
    let mut num_edges_found = 0;
    println!("Open {graph_filename:?} and look for a place where a cut through three edges will break it into two pieces");
    print!("Enter the color of one of those three edges: ");
    loop {
        io::stdout().flush()?;
        let color = read_line!()?;
        match color.trim().parse() {
            Ok(Color::Red) => {
                let range = graph.red_range.clone().unwrap();
                if range.len() == 1 {
                    graph.color_range(range, Color::Orange);
                } else {
                    graph.color_edges(range);
                }
            }
            Ok(Color::Blue) => {
                let range = graph.blue_range.clone().unwrap();
                if range.len() == 1 {
                    graph.color_range(range, Color::Orange);
                } else {
                    graph.color_edges(range);
                }
            }
            Ok(Color::Green) => {
                let range = graph.green_range.clone().unwrap();
                if range.len() == 1 {
                    graph.color_range(range, Color::Orange);
                } else {
                    graph.color_edges(range);
                }
            }
            Ok(Color::Orange) => {
                num_edges_found += 1;
                if num_edges_found == 3 {
                    break;
                }
                graph.color_edges(0..graph.edges.len());
                println!("Pick a new edge.");
            }
            Err(_) => {
                eprintln!("Unrecognized color {color:?}");
                continue;
            }
        }
        if !graph.write_to_file(&graph_filename)?.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Couldn't write graph to {graph_filename:?}"),
            ));
        }
        print!("Recolored graph. Enter the same edge's new color: ");
    }
    let edges = graph
        .edges
        .iter()
        .filter(|edge| matches!(edge.color, Some(Color::Orange)))
        .collect::<Vec<_>>();
    let boundary = edges
        .into_iter()
        .flat_map(|edge| [&*edge.left, &*edge.right])
        .collect::<Vec<_>>();
    let mut left = vec![boundary[0]];
    let mut unclassified = graph.edges.iter().collect::<Vec<_>>();
    let mut i = 0;
    while i < left.len() {
        let mut j = 0;
        while j < unclassified.len() {
            let edge = unclassified[j];
            let other = if edge.left == left[i] {
                &*edge.right
            } else if edge.right == left[i] {
                &*edge.left
            } else {
                j += 1;
                continue;
            };
            unclassified.swap_remove(j);
            if !boundary.contains(&left[i]) || !boundary.contains(&other) {
                left.push(other);
            }
        }
        i += 1;
    }
    let mut right = unclassified
        .into_iter()
        .flat_map(|edge| [&*edge.left, &*edge.right])
        .collect::<Vec<_>>();
    left.sort();
    left.dedup();
    right.sort();
    right.dedup();
    assert_eq!(
        (3, 3),
        boundary
            .into_iter()
            .fold((0, 0), |(left_count, right_count), node| {
                if left.contains(&node) {
                    assert!(!right.contains(&node));
                    (left_count + 1, right_count)
                } else {
                    assert!(right.contains(&node));
                    (left_count, right_count + 1)
                }
            })
    );
    Ok(left.len() * right.len())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 25 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_25.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 25 Part 2");
        println!("This one's free!");
    }
    Ok(())
}
