use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let line = read_lines("input").join("");

    let mut result_part1 = 0;
    let mut result_part2 = 0;
    let regex = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)").expect("Cannot create regex");
    let split_regex = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)|don't\(\)|do\(\)")
        .expect("Cannot create regex");

    let inst: Vec<_> = split_regex
        .find_iter(line.as_str())
        .map(|x| x.as_str())
        .collect();

    let mut mul = true;
    for x in inst {
        if x.starts_with("don") {
            mul = false;
        } else if x.starts_with("do") {
            mul = true;
        } else {
            
            let cap = regex.captures(x).unwrap();
            let split: Vec<_> = cap[1]
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if (mul) {
                result_part2 += split[0] * split[1];
            }
            result_part1 += split[0] * split[1];
        }
    }

    println!(
        "Result part1: {}\nResult part2: {}",
        result_part1, result_part2
    );
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
