use std::collections::HashSet;

use regex::Regex;

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
        let groups: Vec<_> = parts[1].split(',')
            .map(|c| _parse_number(c)).collect();

        let re_vec: Vec<String> = groups
        .iter().map(|group| format!("[\\?#]{{{}}}", group.to_string())).into_iter().collect();
        let re_string = format!("^[\\.\\?]*{}[\\.\\?]*$", re_vec.join("[\\.\\?]{1,}"));
        let re_groups = Regex::new(&re_string).unwrap();

        let mut visited: HashSet<String> = HashSet::new();
        
        let checked = check(&sequence, 0, &groups, &mut visited, &re_groups);
        println!("[total] {} => {}", &sequence.into_iter().collect::<String>(), checked);
        result += checked

    }

    Ok(result.to_string())
}

fn check(sequence: &Vec<char>, current_ix: usize, groups: &Vec<usize>, mut visited: &mut HashSet<String>, re_groups: &regex::Regex) -> usize {
    let sequence_str: String = sequence.into_iter().collect();
    if visited.contains(&sequence_str) {
        println!("visited! {}", sequence_str);
        //return 0
    } else {
        visited.insert(sequence_str.clone());
    }

    let mut result = 0;
    let is_valid_result = is_valid(&sequence, &re_groups);

    //println!("[*] {} => {}", sequence_str, is_valid_result);
    if !is_valid_result {
        return 0;
    }

    if sequence.iter().all(|char| *char != '?' ) {
        return 1
    }

    let char = sequence[current_ix];
    let new_current_ix = current_ix + 1;
    if char == '?' {
        let new_sequence_hash = generate(&sequence, current_ix, '#');
        result += check(&new_sequence_hash, new_current_ix, &groups, &mut visited, re_groups);

        let new_sequence_dot = generate(&sequence, current_ix, '.');
        result += check(&new_sequence_dot, new_current_ix, &groups, &mut visited, re_groups)
    } else {
        result += check(&sequence, new_current_ix, &groups, &mut visited, re_groups)
    }

    result
}

fn is_valid(sequence: &Vec<char>, re_groups: &regex::Regex) -> bool {
    let sequence_str: String = sequence.into_iter().collect();
    re_groups.is_match(&sequence_str)
}

fn generate(sequence: &Vec<char>, ix_to_replace: usize, replacement: char) -> Vec<char> {
    let mut result: Vec<char> = sequence.iter().map(|c| c.clone()).collect();
    result[ix_to_replace] = replacement;

    result
}


fn _parse_digit(number_char: char) -> usize {
    number_char.to_digit(10).unwrap() as usize
}

fn _parse_number(number_str: &str) -> usize {
    let number_str_trim = number_str.trim();
    if number_str_trim.len() > 0 {
        number_str_trim.parse::<usize>().unwrap()
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process2() -> miette::Result<()> {
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

    #[test]
    fn test_process3() -> miette::Result<()> {
        let input = "
.??..??...?##. 1,1,3
";
        let output = process(input.trim());
        assert_eq!(4.to_string(), output?);

        Ok(())
    }

}


