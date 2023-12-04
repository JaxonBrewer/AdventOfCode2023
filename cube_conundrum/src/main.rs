use std::env;
use std::fs;
use std::cmp;
use regex::Regex;

fn read_file(path: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

// Game 1: 1 green, 1 blue, 1 red; 1 green, 8 red, 7 blue; 6 blue, 10 red; 4 red, 9 blue, 2 green; 1 green, 3 blue; 4 red, 1 green, 10 blue
fn parse_line(line: &String) -> [i32; 4] {
    const GAME_REGEX: &str = r"Game (\d+)";
    const CUBE_REGEX: &str = r"(\d+) (\w+)";
    let mut result: [i32; 4] = [0, 0, 0, 0];

    let data: Vec<&str> = line.split(':').collect();
    let game = Regex::new(GAME_REGEX).unwrap().captures(&data[0]).unwrap();
    result[0] = game[1].parse::<i32>().unwrap();

    for cube in Regex::new(CUBE_REGEX).unwrap().captures_iter(&data[1]) {
        let num = cube[1].parse::<i32>().unwrap();
        let color = cube[2].to_string();
        if color == "red" {
            result[1] = cmp::max(num, result[1]);
        } else if color == "green" {
            result[2] = cmp::max(num, result[2]);
        } else if color == "blue" {
            result[3] = cmp::max(num, result[3]);
        } else {
            panic!("Invalid color: {}", color);
        }
    }
    result
}

fn is_game_valid(data: [i32; 4]) -> bool {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    let red = data[1];
    let green = data[2];
    let blue = data[3];
    return red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE
}

fn main() -> std::io::Result<()> {
    let mut sum = 0;
    let mut power = 0;
    // Get first argument
    let args: Vec<String> = env::args().collect();
    let lines = read_file(&args[1]);
    for line in lines {
        let data = parse_line(&line);
        power += data[1] * data[2] * data[3];
        if is_game_valid(data) {
            println!("Game {} is valid", data[0]);
            sum += data[0];
        } else {
            println!("Game {} is invalid", data[0]);
        }
    }

    println!("Sum of valid games: {}", sum);
    println!("Power of valid games: {}", power);
    Ok(())
}
