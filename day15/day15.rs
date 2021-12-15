use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use array2d::Array2D;
use clap::{App, Arg};
use simple_error::SimpleError;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let matches = App::new("day15")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();

    let grid = parse(matches.value_of("file").unwrap())?;
    let large_grid = expand(&grid);

    let graph = Graph::new(grid);
    println!("Shortest path has risk {}", graph.shortest_path_cost()?);
    let large_graph = Graph::new(large_grid);
    println!(
        "Shortest path in full cave has risk {}",
        large_graph.shortest_path_cost()?
    );

    Ok(())
}

fn parse(name: &str) -> Result<Array2D<u8>, Error> {
    let lines = BufReader::new(File::open(name)?)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    let rows = lines.len();
    if rows == 0 {
        return Err(Box::new(SimpleError::new("empty file")));
    }
    let cols = lines[0].len();
    if cols == 0 {
        return Err(Box::new(SimpleError::new("empty lines")));
    }
    let mut grid = Array2D::filled_with(0u8, rows, cols);
    for r in 0..rows {
        for c in 0..cols {
            let v = lines[r][c..c + 1].parse::<u8>()?;
            grid[(r, c)] = v;
        }
    }
    Ok(grid)
}

fn expand(grid: &Array2D<u8>) -> Array2D<u8> {
    let rows = grid.num_rows();
    let cols = grid.num_columns();
    let mut out = Array2D::filled_with(0u8, rows * 5, cols * 5);
    let add = |a, b, c| {
        let mut v = a + b + c;
        while v > 9 {
            v -= 9;
        }
        v
    };

    for r in 0..rows {
        for c in 0..cols {
            let v = grid[(r, c)];
            for i in 0..5 {
                for j in 0..5 {
                    out[(r + i * rows, c + j * rows)] = add(v, i as u8, j as u8);
                }
            }
        }
    }
    out
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point(usize, usize);

struct Graph {
    grid: Array2D<u8>,
}

impl Graph {
    fn new(grid: Array2D<u8>) -> Graph {
        Graph { grid: grid }
    }

    fn for_neighbor<F>(&self, p: Point, mut f: F)
    where
        F: FnMut(Point, u8),
    {
        let mut cb = |r, c| {
            f(Point(r, c), self.grid[(r, c)]);
        };
        if p.0 > 0 {
            cb(p.0 - 1, p.1);
        }
        if p.0 < self.grid.num_rows() - 1 {
            cb(p.0 + 1, p.1);
        }
        if p.1 > 0 {
            cb(p.0, p.1 - 1);
        }
        if p.1 < self.grid.num_columns() - 1 {
            cb(p.0, p.1 + 1);
        }
    }

    fn shortest_path_cost(&self) -> Result<u64, Error> {
        let mut q = BinaryHeap::new();
        let mut visited = HashSet::new();
        let start = Point(0, 0);
        let end = Point(self.grid.num_rows() - 1, self.grid.num_columns() - 1);

        q.push(QEntry::new(0, start, start));
        while !q.is_empty() {
            let qe = q.pop().unwrap();
            if visited.contains(&qe.dst) {
                continue;
            }
            visited.insert(qe.dst);
            if qe.dst == end {
                return Ok(qe.cost);
            }
            self.for_neighbor(qe.dst, |p, w| {
                q.push(QEntry::new(qe.cost + w as u64, qe.dst, p));
            })
        }
        Err(Box::new(SimpleError::new("no path found")))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct QEntry {
    cost: u64,
    src: Point,
    dst: Point,
}

impl QEntry {
    fn new(cost: u64, src: Point, dst: Point) -> QEntry {
        QEntry {
            cost: cost,
            src: src,
            dst: dst,
        }
    }
}

impl std::cmp::PartialOrd for QEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for QEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        }
    }
}
