use std::fs;

static MAX_RED: i32 = 12;
static MAX_GREEN: i32 = 13;
static MAX_BLUE: i32 = 14;

fn part_one() {
    let games: Vec<String> = fs::read_to_string("src/input.txt")
        .expect("Could not find file input.txt")
        .lines()
        .map(|raw| String::from(raw))
        .collect();

    let valid_count: i32 = games
        .iter()
        .map(|game| {
            let parsed_game: Vec<&str> = game.split(": ").collect();
            let game_no = parsed_game
                .first()
                .unwrap()
                .replace("Game ", "")
                .parse::<i32>()
                .expect("Invalid game number");

            let moves = parsed_game
                .last()
                .expect(format!("Unable to find a set of moves for game {:?}", game).as_str());
            let sets: Vec<&str> = moves.split("; ").collect();

            let is_invalid = sets.iter().any(|set| {
                set.split(", ").any(|raw| {
                    let val = raw.to_string();
                    let dat: Vec<&str> = val.split(" ").collect();
                    let num = dat
                        .first()
                        .expect("Invalid number")
                        .parse::<i32>()
                        .expect("Invalid number");
                    let color = dat.last().unwrap().to_owned();
                    match color {
                        "red" => num > MAX_RED,
                        "green" => num > MAX_GREEN,
                        "blue" => num > MAX_BLUE,
                        _ => panic!("Invalid color"),
                    }
                })
            });

            if is_invalid {
                return 0;
            }
            return game_no;
        })
        .sum();
    println!("Part one result {}", valid_count);
}
fn part_two() {
    let games: Vec<String> = fs::read_to_string("src/input.txt")
        .expect("Could not find file input.txt")
        .lines()
        .map(|raw| String::from(raw))
        .collect();

    let valid_count: i32 = games
        .iter()
        .map(|game| {
            let parsed_game: Vec<&str> = game.split(": ").collect();

            let moves = parsed_game
                .last()
                .expect(format!("Unable to find a set of moves for game {:?}", game).as_str());
            let sets: Vec<&str> = moves.split("; ").collect();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for set in sets {
                set.split(", ").for_each(|raw| {
                    let val = raw.to_string();
                    let dat = val.split(" ").collect::<Vec<&str>>();
                    let num = dat
                        .first()
                        .expect("Invalid number")
                        .parse::<i32>()
                        .expect("Invalid number");
                    let color = dat.last().unwrap().to_owned();
                    if color == "red" && num > max_red {
                        max_red = num;
                    }
                    if color == "green" && num > max_green {
                        max_green = num;
                    }
                    if color == "blue" && num > max_blue {
                        max_blue = num;
                    }
                });
            }
            max_red * max_green * max_blue
        })
        .sum();
    println!("Part two result {}", valid_count);
}

fn main() {
    part_one();
    part_two();
}
