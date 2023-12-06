use std::collections::HashMap;
#[allow(dead_code)]
#[allow(unused_variables)]
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let args = std::env::args();
    match parse_file(args.last().unwrap().as_str()) {
        Ok(sum) => println!("{}", sum),
        Err(err) => panic!("{}", err),
    }
}

fn parse_file(filename: &str) -> Result<String, io::Error> {
    let path = Path::new(filename);
    let _display = path.display();
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut res = 0;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    map.insert("red", 12);
    map.insert("green", 13);
    map.insert("blue", 14);
    for (_, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                println!("{} ", l);
                let n = parse_line(&l.to_owned(), &map);
                res += n;
            }
            Err(e) => eprintln!("end of line: {}", e),
        }
    }
    Ok(format!("result: {}", res))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_line(line: &str, max: &HashMap<&str, i32>) -> i32 {
    let bind = line.to_string();
    let set = bind.split(": ").nth(1).unwrap();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    let hand = set.to_string();
    let _: Vec<_> = hand
        .trim()
        .split(";")
        .into_iter()
        .map(|h| {
            h.trim().split(",").into_iter().for_each(|cube| {
                let split: Vec<&str> = cube.trim().split(" ").collect();
                let color = split[1].to_string();
                let value = split[0].trim().parse().unwrap();
                if color == "red" {
                    if value > max_red {
                        max_red = value;
                    }
                }
                if color == "green" {
                    if value > max_green {
                        max_green = value;
                    }
                }
                if color == "blue" {
                    if value > max_blue {
                        max_blue = value;
                    }
                }
            })
        })
        .collect();
    max_red * max_green * max_blue
}
