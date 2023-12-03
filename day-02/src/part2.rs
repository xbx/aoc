use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    // let mut total = 0;

    // Ok(String::from(total.to_string()))
    Ok(String::from("ok"))
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
        assert_eq!(0.to_string(), process(input)?);
        Ok(())
    }
}