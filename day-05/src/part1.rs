use crate::custom_error::AocError;
 struct Mapping {
    source_category: isize,
    destination_cagegory: isize,
    range: isize,
 }

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let sections: Vec<_> = input.split("\n\n").collect();
    let seeds: Vec<_> = sections[0].split(": ").collect::<Vec<_>>()[1].split(" ").collect();
    let mappings = parse_mappings(&sections[1..]);
    
    let mut min_number = std::isize::MAX;
    for seed in seeds {
        let mut next_number = parse_number(&seed);
        for sub_mappings in &mappings {
            next_number = find_matching_mapping(next_number, &sub_mappings);
        }
        if next_number < min_number {
            min_number = next_number;
        }
    }

    Ok(String::from(min_number.to_string()))
}

fn find_matching_mapping(seed_number: isize, mappings: &Vec<Mapping>) -> isize {
    println!("searching for {seed_number}");
    for mapping in mappings {
        if (mapping.source_category..(mapping.source_category+mapping.range)).contains(&seed_number) {
            return seed_number - mapping.source_category + mapping.destination_cagegory
        }
    };    

    seed_number
}

fn parse_mappings<'a>(mappings_txt: &[&str]) -> Vec<Vec<Mapping>> {
    let mut mappings: Vec<Vec<Mapping>> = vec![];

    for mapping_txt in mappings_txt {
        let mut sub_mappings: Vec<Mapping> = vec![];
        let slice_1: Vec<_> = mapping_txt.split(":\n").collect();

        let number_lines: Vec<_> = slice_1[1].split("\n").collect();

        for number_line in number_lines {
            let numbers: Vec<_> = number_line.split(" ").collect();
            sub_mappings.push(
                Mapping {
                    source_category: parse_number(&numbers[1]),
                    destination_cagegory: parse_number(&numbers[0]),
                    range: parse_number(&numbers[2]),
                }
            );
        }
        mappings.push(sub_mappings);        
    }

    mappings
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let output = process(input.trim());
        assert_eq!(35.to_string(), output?);

        Ok(())
    }

}


