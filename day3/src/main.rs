use std::cmp::min;
use std::fs;

fn read_lines() -> Vec<String> {
    fs::read_to_string("src/input.txt")
        .expect("Could not find file input.txt")
        .lines()
        .map(String::from)
        .collect()
}

fn has_symbol(line: Vec<char>) -> bool {
    let symbols = vec!['*', '#', '$', '/', '@', '&', '=', '%', '+', '-'];
    symbols.iter().any(|symbol| line.contains(symbol))
}

fn lookaround_for_symbol(
    lines: &Vec<String>,
    line_idx: usize,
    start_idx: usize,
    end_idx: usize,
) -> bool {
    let mut characters: Vec<char> = vec![];

    let mut start = line_idx;
    if line_idx > 0 {
        start = line_idx - 1;
    }
    let end = min(line_idx + 1, lines.len() - 1);

    // println!(
    //     "start: {}, end: {}, line_idx: {}, , start_idx: {}, end_idx: {}",
    //     start, end, line_idx, start_idx, end_idx
    // );
    for i in start..=end {
        // println!("line {}", i);
        let mut line_chars = lines[i]
            .chars()
            .skip(start_idx)
            .take(end_idx - start_idx)
            .collect::<Vec<_>>();
        characters.append(&mut line_chars);
    }
    // println!("{:?}", characters);
    has_symbol(characters)
}

fn part_one() {
    let lines = read_lines();

    let mut sum = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let mut current_digits: Vec<String> = vec![];
        let mut start_idx: Option<usize> = None;

        for (char_idx, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                current_digits.push(char.to_string());
                if start_idx.is_none() {
                    start_idx = Some(char_idx);
                }
            }
            let chars_amount = line.chars().count();
            let is_last = (char_idx + 1) == chars_amount;
            if start_idx.is_some() && (!char.is_digit(10) || is_last) {
                let mut start = start_idx.unwrap();
                if start != 0 {
                    start -= 1;
                }
                let end = min(char_idx + 1, chars_amount);
                let result = lookaround_for_symbol(&lines, line_idx, start, end);
                if result {
                    let num = current_digits
                        .join("")
                        .parse::<usize>()
                        .expect("Unexpected error while trying to parse");
                    // println!("{:?}", num);
                    sum += num;
                }
                start_idx = None;
                current_digits = vec![];
            }
        }
    }
    println!("Part 1 result: {}", sum);
}
fn part_two() {}

fn main() {
    part_one();
    part_two();
}
