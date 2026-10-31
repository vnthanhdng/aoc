use std::fs;

const SIZE: i32 = 100;
const START: i32 = 50;

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(str::trim).filter(|l| !l.is_empty()).map(|line| {
        let (dir, num) = line.split_at(1);
        let n: i32 = num.parse().expect("bad_number");
        match dir {
            "R" => n,
            "L" => -n,
            _ => panic!("bad direction in {line:?}"),
        }
    })
    .collect()
}

fn puzzle1(rotations: &[i32]) -> usize {
    rotations.iter().scan(START, |pos, &m| {
        *pos = (*pos + m).rem_euclid(SIZE);
        Some(*pos)
    })
    .filter(|&pos| pos == 0).count()
}

fn puzzle2(rotations: &[i32]) -> i32 {
    let mut pos = START;
    let mut count = 0;
    for &rot in rotations {
        let ahead = if rot > 0 {pos} else {(SIZE - pos) % SIZE};
        count += (ahead + rot.abs()) / SIZE;
        pos = (pos + rot).rem_euclid(SIZE);
    }
    count
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(&path).expect("could not read input file");
    let rotations = parse(&input);
    println!("part1: {}", puzzle1(&rotations));
    println!("part2: {}", puzzle2(&rotations));
}