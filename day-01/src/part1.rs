use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut total = 0;
    input.split("\n").for_each(|line| {        
        total += extract_number(line)
    });

    Ok(String::from(total.to_string()))
}

fn extract_number(line: &str) -> isize {
    let chars = line.chars();

    let mut first: isize = -1;
    let mut last: isize = -1;

    for char in chars {
        let result = char.to_digit(10);
        match result {
            None => {},
            Some(number) =>  {
                if first == -1 {
                    first = number as isize
                } else {
                    last = number as isize
                }
            }
        }
    };

    if first >= 0 && last >= 0 {
        return first * 10 + last
    } else if first >= 0 {
        return first * 10 + first
    } else {
        return 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "t2[22]es2t
te1[11]st1
te3[33]st3
1
abc1ddd2
--1234567890
0
000000
11+22abc
aaaa
bb0
0bb
1bb";
        assert_eq!(122.to_string(), process(input)?);


        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142.to_string(), process(input)?);

        Ok(())
    }
}