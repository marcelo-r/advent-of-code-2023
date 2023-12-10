#[allow(dead_code)]
#[allow(unused_variables)]
use std::{
    fs::{self},
    io::{self},
};

fn main() {
    let mut args = std::env::args();
    let schema = match args.nth(1).unwrap().as_str() {
        "1" => schematic_1,
        "2" => schematic_2,
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

fn parse_file(filename: &str, schema: fn(Vec<&str>) -> i64) -> Result<i64, io::Error> {
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let lines: Vec<&str> = content.trim().lines().map(|l| l.trim()).collect();
    let res = schema(lines);

    Ok(res)
}

#[derive(Debug)]
struct FindError {
    number: String,
    new_x: usize,
}

fn schematic_1(lines: Vec<&str>) -> i64 {
    let matrix: Vec<Vec<&str>> = lines
        .iter()
        .map(|l| {
            println!("l: {:?}", l);
            l.trim().split("").into_iter().collect::<Vec<&str>>()
        })
        .collect();
    let _size = matrix[0].len();

    let min_x = 0;
    let max_x = matrix[0].len();
    let min_y = 0;
    let max_y = matrix.len();
    //println!("max_x: {}, max_y: {}", max_x, max_y);
    //println!("min_x: {}, min_y: {}", min_x, min_y);

    fn is_digit(s: &str) -> bool {
        if let Ok(_) = s.parse::<i64>() {
            return true;
        }
        false
    }
    fn is_symbol(s: &str) -> bool {
        let check = !is_digit(s);
        //println!("    {:?} is_symbol: {}", s, check);
        if s == "." || s == "" {
            return false;
        }
        if check {
            return true;
        }
        false
    }

    let find_and_check = |mut x: usize, y: usize| -> Result<(i64, usize), FindError> {
        let mut has_symbol = false;
        let mut acc: Vec<String> = Vec::new();

        while is_digit(matrix[y][x]) {
            acc.push(matrix[y][x].to_string());
            // checks
            if has_symbol {
                x += 1;
                continue;
            }
            // upper row
            // upper left
            if y > min_y && x >= min_x && is_symbol(matrix[y - 1][x - 1]) {
                has_symbol = true;
            }
            // above
            else if y > min_y && is_symbol(matrix[y - 1][x]) {
                has_symbol = true;
            }
            // upper right
            else if y > min_y && x <= max_x && is_symbol(matrix[y - 1][x + 1]) {
                has_symbol = true;
            }
            // behind
            else if x - 1 >= min_x && is_symbol(matrix[y][x - 1]) {
                has_symbol = true;
            }
            // ahead
            else if x + 1 < max_x && is_symbol(matrix[y][x + 1]) {
                has_symbol = true;
            }
            // bottom row
            // bottom left
            else if y + 1 < max_y && x >= min_x && is_symbol(matrix[y + 1][x - 1]) {
                println!("    bottom left: {} of ({},{})", matrix[y + 1][x - 1], x, y);
                has_symbol = true;
            }
            // bellow
            else if y + 1 < max_y && is_symbol(matrix[y + 1][x]) {
                has_symbol = true;
            }
            // bottom right
            else if y + 1 < max_y && x <= max_x && is_symbol(matrix[y + 1][x + 1]) {
                has_symbol = true;
            }
            x += 1;
        }
        if has_symbol {
            let number = acc.join("");
            if let Ok(v) = number.parse::<i64>() {
                return Ok((v, x));
            }
        }
        Err(FindError {
            number: acc.join(""),
            new_x: x,
        })
    };

    let mut with_symbols: Vec<i64> = Vec::new();
    let mut x = 1;
    let mut y = 0;

    loop {
        //println!("({},{}): {}", x, y, matrix[y][x]);
        if matrix[y][x] != "." {
            if let Ok(_) = matrix[y][x].parse::<i64>() {
                // is a number, look around and acumulate
                match find_and_check(x, y) {
                    Ok((number, new_x)) => {
                        with_symbols.push(number);
                        println!("   {} at ({},{}) skip_to: ({},{})", number, x, y, new_x, y);
                        x = new_x;
                        continue;
                    }
                    Err(e) => {
                        println!(
                            "     not found {} at ({},{}) skip_to: ({},{})",
                            e.number, x, y, e.new_x, y
                        );
                        x = e.new_x;
                    }
                };
            } else {
                //println!("{} is not a number", matrix[y][x]);
            }
        }
        x += 1;
        if x >= max_x {
            x = 1;
            y += 1;
        }
        if y >= max_y {
            break;
        }
    }

    with_symbols.iter().fold(0, |acc, elem| acc + elem)
}

#[derive(Debug)]
struct FindRatio {}

fn schematic_2(lines: Vec<&str>) -> i64 {
    let matrix: Vec<Vec<&str>> = lines
        .iter()
        .map(|l| {
            println!("l: {:?}", l);
            l.trim().split("").into_iter().collect::<Vec<&str>>()
        })
        .collect();
    let _size = matrix[0].len();

    let min_x = 0;
    let max_x = matrix[0].len();
    let min_y = 0;
    let max_y = matrix.len();
    //println!("max_x: {}, max_y: {}", max_x, max_y);
    //println!("min_x: {}, min_y: {}", min_x, min_y);

    fn is_digit(s: &str) -> bool {
        if let Ok(_) = s.parse::<i64>() {
            return true;
        }
        false
    }
    let parse_number = |x: usize, y: usize| -> i64 {
        let mut digits: Vec<String> = Vec::new();
        digits.push(matrix[y][x].to_string());

        let mut x_r = x;
        let mut x_l = x;
        while x_r + 1 < max_x && is_digit(matrix[y][x_r + 1]) {
            x_r += 1;
            digits.push(matrix[y][x_r].to_string());
        }
        while x_l - 1 >= min_x && is_digit(matrix[y][x_l - 1]) {
            x_l -= 1;
            digits.insert(0, matrix[y][x_l].to_string());
        }
        let number = digits.join("");
        match number.parse::<i64>() {
            Ok(value) => {
                println!("    digits at ({},{}): {:?}", x, y, digits);
                return value;
            }
            Err(e) => {
                println!("parse_number error: {:?}", e);
                return 0;
            }
        }
    };

    let find_and_check = |x: usize, y: usize| -> Result<i64, FindRatio> {
        let mut numbers: Vec<i64> = Vec::new();

        // upper row
        if y > min_y {
            // above
            if is_digit(matrix[y - 1][x]) {
                let parsed = parse_number(x, y - 1);
                numbers.push(parsed);
            } else {
                // upper left
                if x >= min_x && is_digit(matrix[y - 1][x - 1]) {
                    let parsed = parse_number(x - 1, y - 1);
                    numbers.push(parsed);
                }
                // upper right
                if x <= max_x && is_digit(matrix[y - 1][x + 1]) {
                    let parsed = parse_number(x + 1, y - 1);
                    numbers.push(parsed);
                }
            }
        }

        // behind
        if x - 1 >= min_x && is_digit(matrix[y][x - 1]) {
            let parsed = parse_number(x - 1, y);
            numbers.push(parsed);
        }
        // ahead
        if x + 1 < max_x && is_digit(matrix[y][x + 1]) {
            let parsed = parse_number(x + 1, y);
            numbers.push(parsed);
        }

        // bottom row
        if y + 1 < max_y {
            // bellow
            if is_digit(matrix[y + 1][x]) {
                let parsed = parse_number(x, y + 1);
                numbers.push(parsed);
            } else {
                // bottom left
                if x >= min_x && is_digit(matrix[y + 1][x - 1]) {
                    let parsed = parse_number(x - 1, y + 1);
                    numbers.push(parsed);
                }
                // bottom right
                if x <= max_x && is_digit(matrix[y + 1][x + 1]) {
                    let parsed = parse_number(x + 1, y + 1);
                    numbers.push(parsed);
                }
            }
        }
        println!("  acc: {:?}", numbers);
        if numbers.len() == 2 {
            let ratio = numbers[0] * numbers[1];
            return Ok(ratio);
        }
        Err(FindRatio {})
    };

    let mut with_symbols: Vec<i64> = Vec::new();
    let mut x = 1;
    let mut y = 0;

    loop {
        if matrix[y][x] == "*" {
            println!("({},{}): {}", x, y, matrix[y][x]);
            // is a number, look around and acumulate
            match find_and_check(x, y) {
                Ok(number) => {
                    with_symbols.push(number);
                    println!(" found {} at ({},{})", number, x, y);
                }
                Err(e) => {
                    println!(" no ratio at ({},{}):{:?}", x, y, e);
                }
            };
        }
        x += 1;
        if x >= max_x {
            x = 1;
            y += 1;
        }
        if y >= max_y {
            break;
        }
    }

    with_symbols.iter().fold(0, |acc, elem| acc + elem)
}
