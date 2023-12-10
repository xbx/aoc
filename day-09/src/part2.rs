use std::{fmt, collections::HashMap};

use crate::custom_error::AocError;

struct Step {
    name: String,
    left: String,
    right: String
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] l: {} r: {}", self.name, self.left, self.right)
    }
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> { 
    let sections: Vec<_> = input.split("\n\n").collect();

    let directions: Vec<_> = sections[0].chars().collect();

    let steps = parse_steps(sections[1].split("\n").collect());

    let mut step_counts: Vec<usize> = vec![];

    steps.iter().for_each(|step| {
        if step.1.name.ends_with('A') {
            step_counts.push(calculate_total_steps(&directions, &steps, &step.1.name))
        }
    });

    let mut total_steps = 1;
    for step_count in step_counts {
        total_steps = lcm(total_steps, step_count)
    }
    
    Ok(total_steps.to_string())
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn calculate_total_steps(directions: &Vec<char>, steps: &HashMap<String, Step>, step_name: &str) -> usize {
    let mut total_steps = 0;
    let mut current_direction_ix = 0;
    let mut current_step = steps.get(step_name).unwrap();

    loop {
        //println!("->{}, {current_step}", directions[current_direction_ix]);

        if current_step.name.ends_with('Z') {
            break;
        }

        if directions[current_direction_ix] == 'L' {
            current_step = steps.get(&current_step.left).unwrap();
        } else {
            current_step = steps.get(&current_step.right).unwrap();
        }

        current_direction_ix += 1;
        if current_direction_ix == directions.len() {
            current_direction_ix = 0;
        }
        
        total_steps += 1;
    }

    total_steps
}

fn parse_steps(step_lines: Vec<&str>) -> HashMap<String, Step> {
    let mut steps: HashMap<String, Step> = HashMap::new();

    for step_line in step_lines {
        let parts: Vec<_> = step_line.split(" = ").collect();
        let name = parts[0].to_string();
        let name_2 = parts[0].to_string();
        let left_right: Vec<_> = parts[1].split(", ").collect();
        let left = left_right[0].replace("(", "");
        let right = left_right[1].replace(")", "");
        let step = Step {
            name, left, right
        };
        
        steps.insert(name_2, step);
    }

    steps
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
    fn test_process() -> miette::Result<()> {
        let input = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let output = process(input.trim());
        assert_eq!(6.to_string(), output?);

        Ok(())
    }

}


