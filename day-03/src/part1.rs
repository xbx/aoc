use crate::custom_error::AocError;
use regex::Regex;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {

    let mut lines_process: Vec<&str> = vec![];
    let mut lines: Vec<_> = input.split("\n").collect();

    
    let first_line = String::from(lines[0]);
    let re = Regex::new(".").unwrap();
    let stuff_line = re.replace_all(&first_line, ".");
    
    lines_process.push(&stuff_line);
    lines_process.append(&mut lines);
    
    if lines_process.len() % 3 > 0 {
        for _ in 0..(lines_process.len() % 3) {
            lines_process.push(&stuff_line);
        }
    }
    
    let triplets: Vec<_> = lines_process.chunks(3).collect();
    
    for triplet in triplets {        
        println!("{}/{}/{}", triplet[0], triplet[1], triplet[2]);
    };

    Ok(String::from(parse_line("ok").to_string()))
}


fn parse_line(_line: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(0.to_string(), process(input)?);

        Ok(())
    }
}