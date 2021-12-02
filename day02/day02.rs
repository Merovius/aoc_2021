use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let prog = parse(&mut reader)?;

    let p1 = run(&prog, apply1);
    println!("Final position, part 1: {}", p1);
    let p2 = run(&prog, apply2);
    println!("Final position: part 2: {}", p2);
    Ok(())
}

enum Op {
    Forward,
    Up,
    Down,
}

impl Display for Op {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match *self {
            Op::Forward => f.write_str("Forward"),
            Op::Up => f.write_str("Up"),
            Op::Down => f.write_str("Down"),
        }
    }
}

struct Command {
    op: Op,
    count: i32,
}

impl Display for Command {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        f.write_fmt(format_args!("{}[{}]", self.op, self.count))
    }
}

fn parse<R: BufRead>(r: &mut R) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut prog = Vec::new();
    for line in r.lines() {
        let l = line?;
        let i = l.find(' ').ok_or("no space in line")?;
        let count = l[i + 1..].parse::<i32>()?;
        let op = match &l[..i] {
            "forward" => Ok(Op::Forward),
            "up" => Ok(Op::Up),
            "down" => Ok(Op::Down),
            _ => Err("invalid command"),
        }?;
        prog.push(Command { op, count });
    }
    Ok(prog)
}

struct Pos {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Display for Pos {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        f.write_fmt(format_args!(
            "horizontal={}, depth={}, product={}",
            self.horizontal,
            self.depth,
            self.horizontal * self.depth
        ))
    }
}

fn run(prog: &Vec<Command>, apply: fn(&mut Pos, &Command)) -> Pos {
    let mut p = Pos {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for c in prog {
        apply(&mut p, &c);
    }
    return p;
}

fn apply1(p: &mut Pos, c: &Command) {
    match c.op {
        Op::Forward => p.horizontal += c.count,
        Op::Down => p.depth += c.count,
        Op::Up => p.depth -= c.count,
    };
}

fn apply2(p: &mut Pos, c: &Command) {
    match c.op {
        Op::Forward => {
            p.horizontal += c.count;
            p.depth += c.count * p.aim;
        }
        Op::Down => p.aim += c.count,
        Op::Up => p.aim -= c.count,
    }
}
