use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input");
    let mut safe_reports = 0;
    let mut safe_reports2 = 0;
    for report in lines {
        println!("line: {report}");
        let mut numbers: Vec<_> = report
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let is_safe = is_safe_report(&numbers);
        if is_safe {
            safe_reports += 1;
            safe_reports2 += 1;
            continue;
        }

        for i in 0..numbers.len() {
            let tmp_num = numbers[i];
            numbers.remove(i);
            let is_safe = is_safe_report(&numbers);
            if is_safe {
                safe_reports2 += 1;
                break;
            }
            numbers.insert(i, tmp_num);
        }
    }
    println!("total_safe: {}", safe_reports);
    println!("total_safe2: {}", safe_reports2);
}

fn is_safe_report(numbers: &[i32]) -> bool {
    let mut asc = true;
    let mut des = true;
    for i in 0..numbers.len() - 1 {
        println!("current {} next {}", numbers[i], numbers[i + 1]);
        if numbers[i] > numbers[i + 1] {
            asc = false;
        }
        if numbers[i] < numbers[i + 1] {
            des = false;
        }
        let diff = (numbers[i] - numbers[i + 1]).abs();
        if (diff < 1 || diff > 3) || (!asc && !des) {
            return false;
        }
    }
    true
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
