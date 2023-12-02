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
                        current_number_name.clear();
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
                current_number_name = String::from("");

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
";
        assert_eq!(890.to_string(), process(input)?);

        Ok(())
    }

    #[test]
    fn test_process3() -> miette::Result<()> {
        let input = "1
two
1xx
11
1two
twoxx
two1
twotwo
1xxxx
1xx1
1xxtwo
11xx
111
11two
1twoxx
1two1
1twotwo
twoxxxx
twoxx1
twoxxtwo
two1xx
two11
two1two
twotwoxx
twotwo1
twotwotwo
1xxxxxx
1xxxx1
1xxxxtwo
1xx1xx
1xx11
1xx1two
1xxtwoxx
1xxtwo1
1xxtwotwo
11xxxx
11xx1
11xxtwo
111xx
1111
111two
11twoxx
11two1
11twotwo
1twoxxxx
1twoxx1
1twoxxtwo
1two1xx
1two11
1two1two
1twotwoxx
1twotwo1
1twotwotwo
twoxxxxxx
twoxxxx1
twoxxxxtwo
twoxx1xx
twoxx11
twoxx1two
twoxxtwoxx
twoxxtwo1
twoxxtwotwo
two1xxxx
two1xx1
two1xxtwo
two11xx
two111
two11two
two1twoxx
two1two1
two1twotwo
twotwoxxxx
twotwoxx1
twotwoxxtwo
twotwo1xx
twotwo11
twotwo1two
twotwotwoxx
twotwotwo1
twotwotwotwo
1xxxxxxxx
1xxxxxx1
1xxxxxxtwo
1xxxx1xx
1xxxx11
1xxxx1two
1xxxxtwoxx
1xxxxtwo1
1xxxxtwotwo
1xx1xxxx
1xx1xx1
1xx1xxtwo
1xx11xx
1xx111
1xx11two
1xx1twoxx
1xx1two1
1xx1twotwo
1xxtwoxxxx
1xxtwoxx1";
        assert_eq!(1549.to_string(), process(input)?);

        Ok(())
    }
}