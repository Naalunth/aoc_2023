use crate::util::parsers::*;
use anyhow::anyhow;
use aoc_runner_derive::*;
use nom::{branch::*, bytes::complete::*, combinator::*, multi::*, sequence::*, *};
use std::cmp::max;
use Color::*;

pub struct Game {
    pub id: u64,
    pub draws: Box<[Draw]>,
}

#[derive(Default, Copy, Clone)]
pub struct Draw {
    pub r: u64,
    pub g: u64,
    pub b: u64,
}

impl Draw {
    fn set_color(&mut self, value: u64, color: Color) {
        *match color {
            Red => &mut self.r,
            Green => &mut self.g,
            Blue => &mut self.b,
        } = value;
    }
}

impl FromIterator<(u64, Color)> for Draw {
    fn from_iter<T: IntoIterator<Item = (u64, Color)>>(iter: T) -> Self {
        let mut draw = Self::default();
        for (count, color) in iter {
            draw.set_color(count, color);
        }
        draw
    }
}

#[derive(Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[aoc_generator(day2)]
pub fn generator(input: &[u8]) -> anyhow::Result<Vec<Game>> {
    all_consuming(separated_list0(tag(b"\n"), parse_game))(input)
        .map_err(|e| anyhow!(e.to_owned()))
        .map(|(_input, value)| value)
}

fn parse_game(input: &[u8]) -> IResult<&[u8], Game> {
    let (input, _) = tag(b"Game ")(input)?;
    let (input, id) = unsigned_number::<u64>(input)?;
    let (input, _) = tag(b": ")(input)?;
    let (input, draws) = separated_list0(tag(b"; "), parse_draw)(input)?;
    Ok((
        input,
        Game {
            id,
            draws: draws.into_boxed_slice(),
        },
    ))
}

fn parse_draw(input: &[u8]) -> IResult<&[u8], Draw> {
    separated_list1(tag(b", "), parse_draw_entry)
        .map(|entries| entries.into_iter().collect())
        .parse(input)
}

fn parse_draw_entry(input: &[u8]) -> IResult<&[u8], (u64, Color)> {
    separated_pair(
        unsigned_number,
        tag(b" "),
        alt((
            value(Red, tag(b"red")),
            value(Green, tag(b"green")),
            value(Blue, tag(b"blue")),
        )),
    )(input)
}

#[aoc(day2, part1)]
pub fn part_1(games: &[Game]) -> u64 {
    games
        .iter()
        .filter(|&game| {
            game.draws
                .iter()
                .all(|draw| draw.r <= 12 && draw.g <= 13 && draw.b <= 14)
        })
        .map(|&Game { id, .. }| id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part_2(games: &[Game]) -> u64 {
    games
        .iter()
        .map(|game| {
            let Draw { r, g, b } = game
                .draws
                .iter()
                .copied()
                .reduce(|acc, draw| Draw {
                    r: max(acc.r, draw.r),
                    g: max(acc.g, draw.g),
                    b: max(acc.b, draw.b),
                })
                .unwrap_or(Draw::default());
            r * g * b
        })
        .sum()
}
