use range_collections::{RangeSet2, RangeSet};

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
    let seeds_all: Vec<_> = sections[0].split(": ").collect::<Vec<_>>()[1].split(" ").collect();
    let mappings = parse_mappings(&sections[1..]);
    
    let mut min_number = std::isize::MAX;
    for seed_info in seeds_all.chunks(2) {
        let seed = parse_number(seed_info[0]);
        let seed_range = parse_number(seed_info[1]);
        // println!("new seed: {}", seed);

        let mut matching_mappings: Vec<(isize, isize)> = vec![(seed, seed_range)];
    
        for sub_mappings in &mappings {
            
            let mut new_matching_mappings: Vec<(isize, isize)> = vec![];
            while matching_mappings.len() > 0 {
                let maching_mapping = matching_mappings.pop().unwrap();
                new_matching_mappings.append(
                    &mut find_matching_mappings(
                        maching_mapping.0, maching_mapping.1, &sub_mappings
                    )
                );
            }
            matching_mappings.clear();
            matching_mappings.append(&mut new_matching_mappings);
        }

        for mapping in matching_mappings {
            if mapping.0 < min_number  {
                min_number = mapping.0
            }
        }
    }

    Ok(String::from(min_number.to_string()))
}

fn find_matching_mappings(seed_number: isize, range: isize,  mappings: &Vec<Mapping>) -> Vec<(isize, isize)> {
    //println!("searching for {seed_number}");
    let seed_range: RangeSet2<isize> = RangeSet::from(seed_number..(seed_number + range - 1));

    let mut matching_mappings: Vec<(isize, isize)> = vec![];

    for mapping in mappings {
        let mapping_range: RangeSet2<isize> = RangeSet::from(
            mapping.source_category..(mapping.source_category + mapping.range - 1)
        );
        
        if seed_range.intersects(&mapping_range) {
            let intersection: RangeSet2<isize> = seed_range.intersection(&mapping_range);
            let boundaries: &[isize] = intersection.boundaries();

            let new_range = boundaries[1] - boundaries[0] + 1;
            let new_number = boundaries[0] - mapping.source_category + mapping.destination_cagegory;

            matching_mappings.push((new_number, new_range));
        }
    };    

    if matching_mappings.len() > 0 {
        matching_mappings
    } else {
        vec![(seed_number, range)]
    }
}

fn parse_mappings(mappings_txt: &[&str]) -> Vec<Vec<Mapping>> {
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
//621354867
//621354867

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
        assert_eq!(46.to_string(), output?);

        Ok(())
    }

}


