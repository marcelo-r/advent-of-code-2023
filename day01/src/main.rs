#[allow(dead_code)]
#[allow(unused_variables)]
use std::{
    collections::HashMap,
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
    let display = path.display();
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let reader = BufReader::new(file);
    let mut acc = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                println!("{} ", l);
                let (a, z) = parse_line(&l.to_owned());
                let digit = format!("{}{}", a, z);
                let number: i32 = match digit.parse() {
                    Ok(d) => d,
                    Err(_) => 0,
                };
                println!("\t-> '{}'++'{}' = {}\n-----", a, z, number);
                acc += number;
            }
            Err(e) => eprintln!("end of line: {}", e),
        }
    }
    println!("calibration: {}", acc);
    Ok(display.to_string())
}

const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";

const DIGIT_MAP: [(&str, &str); 9] = [
    (ONE, "1"),
    (TWO, "2"),
    (THREE, "3"),
    (FOUR, "4"),
    (FIVE, "5"),
    (SIX, "6"),
    (SEVEN, "7"),
    (EIGHT, "8"),
    (NINE, "9"),
];

fn create_hashmap() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for (word, digit) in DIGIT_MAP.iter() {
        map.insert(word.to_owned(), digit.to_owned());
    }
    map
}

fn parse_line(line: &str) -> (String, String) {
    let mut numbers: Vec<String> = Vec::new();
    let mut acc: Vec<String> = Vec::new();

    let map = create_hashmap();

    // FIXME for case 'eightwothree'
    for (i, char) in line.chars().enumerate() {
        println!("numbers: {:?}", numbers);
        if char.is_digit(10) {
            numbers.push(char.to_string());
            println!("numbers: {:?}", numbers);
            acc.clear();
            continue;
        }
        // i + 2
        // i + 3
        // i + 4
        let mut check_set = |word: &str| {
            match map.get(word) {
                Some(digit) => {
                    numbers.push(digit.to_string());
                    println!("  n:{:?}", digit);
                }
                None => (),
            };
        };
        println!("char: '{}' i: {} line.len(): {}", char, i, line.len());
        if i + 2 <= line.len() {
            let plus2 = &line.to_string()[i..i + 2];
            //println!(" plus2: {}", plus2);
            check_set(plus2);
        }
        if i + 3 <= line.len() {
            let plus3 = &line.to_string()[i..i + 3];
            //println!(" plus3: {}", plus3);
            check_set(plus3);
        }
        if i + 4 <= line.len() {
            let plus4 = &line.to_string()[i..i + 4];
            //println!(" plus4: {}", plus4);
            check_set(plus4);
        }
        if i + 5 <= line.len() {
            let plus5 = &line.to_string()[i..i + 5];
            //println!(" plus4: {}", plus4);
            check_set(plus5);
        }
    }

    if numbers.len() == 0 {
        return ("".to_string(), "".to_string());
    }
    if numbers.len() == 1 {
        return (
            numbers.first().unwrap().to_string(),
            numbers.first().unwrap().to_string(),
        );
    }
    return (
        numbers.first().unwrap().to_string(),
        numbers.last().unwrap().to_string(),
    );
}
