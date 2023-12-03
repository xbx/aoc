use std::collections::HashMap;

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

    let number_names: HashMap<&str, isize>  = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);


    let chars = line.chars();
    let mut current_number_name = String::from("");

    let mut first: isize = -1;
    let mut last: isize = -1;

    for char in chars {
        let result = char.to_digit(10);
        match result {
            None => {
                current_number_name.push(char);
                
                for (numberitem, numbervalue) in &number_names {
                    if current_number_name.contains(numberitem)  {
                        current_number_name = current_number_name.replace(
                            numberitem, &numberitem.chars().last().unwrap().to_string()
                        );
                        if first == -1 {
                            first = *numbervalue
                        } else {
                            last = *numbervalue
                        }
                        break;
                    }
                }
            },
            Some(number) =>  {
                current_number_name.clear();

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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281.to_string(), process(input)?);

        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "1qeighthfzzsvsvtph
tfivehpstfxnine8two
jpvprgf18vdxvhzksnngj
3fivebnzjjjnkqsix95four69
54hcnlonesblqvtnh3qhcnine
rzmjgxcxtfive9qspggmrsntwosix
ntkfivefive9ninevmcnrfznzfour2
5jeightlvd9kdzsscqchjbrfive
fbsthree3oneeight
zmmzsdhthreetwo7two1mzbcsnhmqllsftqzmnhjtbdrff
three
111111seven111111
one222222one
okokok
3kbklxmh
7874
oneone
6nine6
xzx93
1
4r
eightmvgrrqgqftjdk3mrfourthreefivef
qoneightfourgvz6sixone
qoneightfooo
";
        assert_eq!(919.to_string(), process(input)?);

        Ok(())
    }
}