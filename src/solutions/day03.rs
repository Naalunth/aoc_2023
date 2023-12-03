use std::cmp::min;

use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> u64 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map = input
        .lines()
        .flat_map(|line| line.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    let mut part_sum = 0u64;

    for y in 0..height {
        let mut is_in_number = false;
        let mut number_start = 0;
        let get_part_number = |start_x: usize, end_x: usize| {
            (start_x..=end_x).fold(0u64, |acc, e| {
                acc * 10 + (map[y * height + e] - b'0') as u64
            })
        };
        let mut check_part = |start_x: usize, end_x: usize| {
            'outer: for check_y in y.saturating_sub(1)..=min(y + 1, height - 1) {
                for check_x in start_x.saturating_sub(1)..=min(end_x + 1, width - 1) {
                    let c = map[check_y * height + check_x];
                    if c != b'.' && !c.is_ascii_digit() {
                        part_sum += get_part_number(start_x, end_x);
                        break 'outer;
                    }
                }
            }
        };
        for x in 0..width {
            if map[y * height + x].is_ascii_digit() {
                if !is_in_number {
                    is_in_number = true;
                    number_start = x;
                }
            } else if is_in_number {
                check_part(number_start, x - 1);
                is_in_number = false;
            }
        }
        if is_in_number {
            check_part(number_start, width - 1);
        }
    }

    part_sum
}

#[derive(Clone, Copy)]
struct Part {
    y: usize,
    start_x: usize,
    end_x: usize,
}

#[derive(Clone, Copy)]
struct Gear {
    y: usize,
    x: usize,
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> u64 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map = input
        .lines()
        .flat_map(|line| line.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    let mut parts = vec![];
    let mut gears = vec![];

    let get_part_number = |part: Part| {
        (part.start_x..=part.end_x).fold(0u64, |acc, e| {
            acc * 10 + (map[part.y * height + e] - b'0') as u64
        })
    };

    for y in 0..height {
        let mut is_in_number = false;
        let mut number_start = 0;
        for x in 0..width {
            if map[y * height + x].is_ascii_digit() {
                if !is_in_number {
                    is_in_number = true;
                    number_start = x;
                }
            } else if is_in_number {
                parts.push(Part {
                    y,
                    start_x: number_start,
                    end_x: x - 1,
                });
                is_in_number = false;
            }
            if map[y * height + x] == b'*' {
                gears.push(Gear { y, x });
            }
        }
        if is_in_number {
            parts.push(Part {
                y,
                start_x: number_start,
                end_x: width - 1,
            });
        }
    }

    gears
        .into_iter()
        .flat_map(|gear| {
            let (count, product) = parts
                .iter()
                .filter(|&part| {
                    part.y.abs_diff(gear.y) <= 1
                        && gear.x >= part.start_x.saturating_sub(1)
                        && gear.x <= part.end_x + 1
                })
                .fold((0u64, 1u64), |(count, product), part| {
                    (count + 1, product * get_part_number(*part))
                });
            (count == 2).then_some(product)
        })
        .sum()
}
