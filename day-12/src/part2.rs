use std::collections::HashSet;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<_> = input.split("\n").collect();
    let mut nonempty_cols: HashSet<usize> = HashSet::new();
    let mut nonempty_rows: HashSet<usize> = HashSet::new();

    let mut map = generate_map(lines, &mut nonempty_rows, &mut nonempty_cols);

    let expanded_map = expand_map(&mut map, &nonempty_rows, &nonempty_cols);

    expanded_map.iter().for_each(|row| {
        row.iter().for_each(|char| {
            print!("{}", char);
        });
        println!("");
    });

    let pairs = find_pairs(&expanded_map);

    let result = calculate_distances(&expanded_map, &pairs);
    
    Ok(result.to_string())
}

fn calculate_distances(
    map: &Vec<Vec<char>>, pairs: &Vec<((usize, usize), (usize, usize))>
) -> usize {
    let factor = 1000000 - 1;
    let mut total = 0;

    pairs.iter().for_each(|(a, b)| {
        let mut pair_distance = 0;
        let mut sorted: Vec<_> = vec![a, b];
        sorted.sort_by_key(|k| k.0);

        let (i, j) = (sorted[0], sorted[1]);

        for row_ix in (i.0 + 1)..(j.0 + 1) {
            if vec!['R', 'B'].contains(&map[row_ix][i.1]) {
                pair_distance += factor;
            } else {
                pair_distance += 1;
            }
        }

        sorted.sort_by_key(|k| k.1);
        let (i, j) = (sorted[0], sorted[1]);

        for col_ix in (i.1 + 1)..(j.1 + 1) {
            if vec!['C', 'B'].contains(&map[j.0][col_ix]) {
                pair_distance += factor;
            } else {
                pair_distance += 1;
            }
        }

        //println!("{:?}, {:?} => {}", a, b, pair_distance);

        total += pair_distance;
    });

    total
}

fn find_pairs(map: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize))> {
    let mut positions: Vec<(usize, usize)> = vec![];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#' {
                positions.push((i, j));
            }
        }
    }

    let mut pairs: Vec<((usize, usize), (usize, usize))> = vec![];
    for i in 0..positions.len() {
        for j in 0..positions.len() {
            if i == j {
                break
            }
            pairs.push((positions[i], positions[j]));
        }
    }

    pairs
}

fn generate_map(lines: Vec<&str>, nonempty_rows: &mut HashSet<usize>, nonempty_cols: &mut HashSet<usize>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];

    for (row, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        chars.iter().enumerate().for_each(|(col, char)| {
            if *char == '#' {
                nonempty_cols.insert(col);
            }
        });

        if chars.iter().any(|c| *c == '#') {
            nonempty_rows.insert(row);
        }

        map.push(chars);
    }
    
    map
}

fn expand_map<'a>(
    map: &'a mut Vec<Vec<char>>, nonempty_rows: &'a HashSet<usize>, nonempty_cols: &'a HashSet<usize>
) -> &'a mut Vec<Vec<char>> {   
    // R: expanded row
    // C: expanded col
    // B: expanded both 
    for row_ix in (0..map.len()).into_iter().rev() {
        if !nonempty_rows.contains(&row_ix) {
            //let newrow: Vec<char> = map[row_ix].iter().map(|c| c.clone()).collect();
            let newrow: Vec<char> = map[row_ix].iter().map(|_c| 'R').collect();
            map.insert(row_ix + 1, newrow);
        }
    }

    for row_ix in 0..map.len() {
        for col_ix in (0..map[row_ix].len()).into_iter().rev() {
            if !nonempty_cols.contains(&col_ix) {
                //let newchar = map[row_ix][col_ix].clone();
                let newchar = if map[row_ix][col_ix] == 'R' {'B'} else {'C'};
                map[row_ix].insert(col_ix + 1, newchar);
            }
        }
    }

    map
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
    fn test_process() -> miette::Result<()> {
        let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let output = process(input.trim());
        assert_eq!(82000210.to_string(), output?);

        Ok(())
    }

}
