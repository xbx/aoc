use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut total_values = 0;
    
    let lines: Vec<_> = input.split("\n").collect();

    for line in lines {
        let mut numbers: Vec<_> = line.split(" ").map(|n| _parse_number(n)).collect();
        let mut last_numbers: Vec<isize> = vec![*numbers.last().unwrap()];

        loop {
            let inner_sequence = get_inner_sequence(&numbers);
            last_numbers.push(*inner_sequence.last().unwrap());

            if inner_sequence.iter().all(|n| *n == 0) {
                println!("{}", inner_sequence.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
                break
            }
            numbers = inner_sequence
        }
        let next_value = last_numbers.iter().sum::<isize>();

        total_values += next_value;
    }
    
    Ok(total_values.to_string())
}

fn get_inner_sequence(numbers: &Vec<isize>) -> Vec<isize> {
    let mut inner_sequence: Vec<isize> = vec![];

    for i in 0..(numbers.len() - 1) {
        inner_sequence.push(numbers[i+1] - numbers[i]);
    }

    inner_sequence
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
    fn test_process2() -> miette::Result<()> {
        let input = "
15 12 9 6 3 0
";
        let output = process(input.trim());
        assert_eq!((-3).to_string(), output?);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let output = process(input.trim());
        assert_eq!(114.to_string(), output?);

        Ok(())
    }

}


