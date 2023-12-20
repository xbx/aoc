use std::collections::HashSet;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<_> = input.split("\n").collect();
    
    let mut result = 0;

    for line in lines {
        println!("{}", line);

        let parts: Vec<_> = line.split(" ").collect();
        let sequence: Vec<_> = parts[0].chars().collect();
        let groups: Vec<_> = parts[1].as_bytes().iter().map(|c| _parse_number(&c.to_string())).collect();

        result += check(&sequence, &groups);

    }

    Ok(result.to_string())
}

fn check(sequence: &Vec<char>, groups: &Vec<isize>) -> usize {
    let mut result = 0;

    if !is_valid(&sequence, &groups) {
        return 0;
    }

    sequence.iter().enumerate().for_each(|(ix, char)| {
        if *char == '?' {
            let new_sequence = generate(&sequence, ix);

            result += check(&new_sequence, &groups)
        }
    });

    1
}

fn is_valid(sequence: &Vec<char>, groups: &Vec<isize>) -> bool {
    let mut found_groups = vec![];

    let previous_char = sequence[0];
    let mut current_length = 0;
    for i in 1..sequence.len() {
        if sequence[i] == '#' {
            current_length += 1;
        } else if previous_char == '#' {
            found_groups.push(current_length);
            current_length = 0;
        }
    }
    if current_length > 0 {
        found_groups.push(current_length);
    }

    true
}

fn generate(sequence: &Vec<char>, ix_to_replace: usize) -> Vec<char> {
    let mut result: Vec<char> = sequence.iter().map(|c| c.clone()).collect();
    result[ix_to_replace] = '#';

    result
}

fn _parse_number(number_str: &str) -> isize {
    let number_str_trim = number_str.trim();
    if number_str_trim.len() > 0 {
        number_str_trim.parse::<isize>().unwrap()
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let output = process(input.trim());
        assert_eq!(21.to_string(), output?);

        Ok(())
    }

}


