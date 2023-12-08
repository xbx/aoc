use regex::Regex;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let re = Regex::new(" +").unwrap();
    let input2 = re.replace_all(input, " ");
    
    let lines: Vec<_> = input2.split("\n").collect();
    
    let time_line = lines[0].split(":").collect::<Vec<_>>()[1].trim();
    let distance_line = lines[1].split(":").collect::<Vec<_>>()[1].trim();

    let times: Vec<_> = time_line.split(" ").map(|i| parse_number(i)).collect();
    let distances: Vec<_> = distance_line.split(" ").map(|i| parse_number(i)).collect();

    let mut total_ways = 1;
    for i in 0..times.len() {
        let race_time = times[i];
        let record_distance = distances[i];

        let mut ways = 0;
        for t in 1..(race_time + 1) {
            let speed = t;
            let distance = speed * (race_time - t);
            if distance > record_distance {
                ways += 1;
            }
        }
        total_ways *= ways;
    }

    for line in lines {
        println!("{line}");
    }

    Ok(total_ways.to_string())
}


fn parse_number(number_str: &str) -> isize {
    let number_str_trim = number_str.trim();
    if number_str_trim.len() > 0 {
        number_str_trim.parse::<isize>().unwrap()
    } else {
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
Time:      7  15   30
Distance:  9  40  200
";
        let output = process(input.trim());
        assert_eq!(288.to_string(), output?);

        Ok(())
    }

}


