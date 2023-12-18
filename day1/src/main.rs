use std::fs;

fn replace_spelled_numbers(text: &str) -> String {
    text.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Could not find file input.txt");
    let input = input.trim();
    let lines: Vec<String> = input.lines().map(|x| String::from(x)).collect();

    let numbers: Vec<u32> = lines
        .iter()
        .filter_map(|line| {
            let digits: Vec<String> = replace_spelled_numbers(line)
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|v| v.to_string())
                .collect();
            let first = digits.first().expect("First digit is not valid").to_owned();
            let last = digits.last().expect("First digit is not valid").to_owned();
            let calc = vec![first, last].join("");
            Some(calc.parse::<u32>().unwrap_or(0))
        })
        .collect();

    println!("{:?}", numbers.iter().sum::<u32>());
}
