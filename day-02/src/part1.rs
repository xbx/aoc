use crate::custom_error::AocError;

struct Game {
    id: isize,
    rounds: Vec<Round>,
}

struct Round {
    red: isize,
    green: isize,
    blue: isize,
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut total_ids = 0;

    input.split("\n").for_each(|line| {        
        let game = parse_line(line.trim());

        let mut possible = true;
        for round in game.rounds {
            if round.red > max_red
                || round.green > max_green
                || round.blue > max_blue {
                    possible = false;
                    break;
            }
        }
        if(possible == true) {
            total_ids += game.id;
        }
    });

    Ok(String::from(total_ids.to_string()))
}


fn parse_line(line: &str) -> Game {
    let mut rounds: Vec<Round> = vec![];

    let game_line: Vec<_> = line.split(": ").collect();
    let id_part: Vec<_> = game_line[0].split(" ").collect();
    let id = id_part[1];

    let rounds_txt: Vec<_> = game_line[1].split("; ").collect();

    for round_txt in rounds_txt {
        let colors: Vec<_> = round_txt.split(", ").collect();
        
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;
        for color_entry in colors {
            let color_entry_parts: Vec<_> = color_entry.split(" ").collect();
            let count = color_entry_parts[0].parse::<i32>().unwrap();
            match color_entry_parts[1] {
                "red" => red_count = count,
                "green" => green_count = count,
                "blue" => blue_count = count,
                _ => panic!("unhandled color: {}", color_entry_parts[1])
            }
        }
        
        rounds.push(
            Round {
                red: red_count as isize,
                green: green_count as isize,
                blue: blue_count as isize,
            }
        )
    }
    

    Game {
        id: id.parse::<i32>().unwrap() as isize,
        rounds: rounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8.to_string(), process(input)?);

        Ok(())
    }
}