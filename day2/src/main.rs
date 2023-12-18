use std::fs;

static MAX_RED: i32 = 12;
static MAX_GREEN: i32 = 13;
static MAX_BLUE: i32 = 14;

fn first_part() {
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
    print!("Result {}", valid_count);
}

fn main() {
    first_part();
}
