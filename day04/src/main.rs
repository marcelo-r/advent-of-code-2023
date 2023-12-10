use std::collections::HashSet;
#[allow(dead_code)]
#[allow(unused_variables)]
use std::{
    fs::{self},
    io::{self},
};

fn main() {
    let mut args = std::env::args();
    let schema = match args.nth(1).unwrap().as_str() {
        "1" => part_1,
        //"2" => part_2,
        _ => {
            println!("invalid schema");
            return;
        }
    };
    match parse_file(args.last().unwrap().as_str(), schema) {
        Ok(sum) => println!("result: {}", sum),
        Err(err) => panic!("{}", err),
    }
}

fn parse_file(filename: &str, parser: fn(Vec<&str>) -> i64) -> Result<i64, io::Error> {
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let lines: Vec<&str> = content.trim().lines().map(|l| l.trim()).collect();
    let res = parser(lines);

    Ok(res)
}

fn power_of_2(len: usize) -> i64 {
    let two: i64 = 2;
    let power = len as u32;
    two.pow(power - 1)
}

fn part_1(lines: Vec<&str>) -> i64 {
    let noprefix: Vec<&str> = lines
        .iter()
        .map(|line| line.split(":").into_iter().last().unwrap().trim())
        .collect();
    let games: Vec<_> = noprefix
        .iter()
        .map(|game| game.split("|").into_iter().collect::<Vec<_>>())
        .collect();
    let mut total = 0;
    for game in games.iter() {
        let winners = game[0].to_owned();
        let lottery: HashSet<_> = winners
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .collect();

        let bet = game[1].to_owned();
        let winning_numbers: Vec<_> = bet
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|x| lottery.contains(*x))
            .collect();

        let points = match winning_numbers.len() {
            0 => 0,
            _ => power_of_2(winning_numbers.len()),
        };
        println!("{:?}\t{} \t -> {}", lottery, bet, points);

        total += points;
    }

    total
}
