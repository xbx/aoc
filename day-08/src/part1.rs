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
    let mut total_steps = 0;
    
    let sections: Vec<_> = input.split("\n\n").collect();

    let directions: Vec<_> = sections[0].chars().collect();

    let steps = parse_steps(sections[1].split("\n").collect());

    let mut current_direction_ix = 0;
    let mut current_step = steps.get("AAA").unwrap();

    loop {
        //println!("->{}, {current_step}", directions[current_direction_ix]);

        if current_step.name == "ZZZ" {
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
    
    Ok(total_steps.to_string())
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
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        let output = process(input.trim());
        assert_eq!(6.to_string(), output?);

        Ok(())
    }

}


