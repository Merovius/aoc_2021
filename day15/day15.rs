use std::collections::{BinaryHeap, HashSet};

use array2d::Array2D;
use simple_error::SimpleError;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let grid = parse();
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

fn parse() -> Array2D<u8> {
    let input = include_str!("input.txt").trim_end();
    let cols = input.find("\n").unwrap();
    let rows = input.len() / cols;
    Array2D::from_iter_row_major(
        input.bytes().filter_map(|b| {
            let b = b.wrapping_sub('0' as u8);
            if b < 10 {
                Some(b)
            } else {
                None
            }
        }),
        rows,
        cols,
    )
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

        q.push(QEntry::new(0, start));
        while !q.is_empty() {
            let qe = q.pop().unwrap();
            if qe.dst == end {
                return Ok(qe.cost);
            }
            self.for_neighbor(qe.dst, |p, w| {
                if visited.contains(&p) {
                    return;
                }
                visited.insert(p);
                q.push(QEntry::new(qe.cost + w as u64, p));
            })
        }
        Err(Box::new(SimpleError::new("no path found")))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct QEntry {
    cost: u64,
    dst: Point,
}

impl QEntry {
    fn new(cost: u64, dst: Point) -> QEntry {
        QEntry {
            cost: cost,
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
