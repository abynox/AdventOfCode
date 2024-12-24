use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    println!("Test");
    let lines = read_lines("input");
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    for line in lines {
        let split: Vec<_> = line.split("   ").collect();
        first_list.push(split[0].parse::<i32>().unwrap());
        second_list.push(split[1].parse::<i32>().unwrap());
    }
    first_list.sort();
    second_list.sort();

    println!("{:?} {:?}", first_list, second_list);

    let mut total_distance = 0;

    for i in 0..first_list.len() {
        total_distance += (first_list[i].abs() - second_list[i].abs()).abs();
    }

    println!("total distance: {}", total_distance);

    total_distance = 0;

    for i in 0..first_list.len() {
        let mut number_counter = 0;
        for j in &second_list {
            if first_list[i] == *j {
                number_counter += 1;
            }
        }

        total_distance += first_list[i] * number_counter;
    }

    println!("total similarity: {}", total_distance);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
