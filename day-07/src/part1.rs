use std::{fmt, cmp::Ordering, collections::HashMap};

use crate::custom_error::AocError;

#[derive(Eq)]
struct Hand {
    hand: Vec<char>,
    rank: usize,
    bid: usize,
    values: HashMap<char, usize>
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank != other.rank {
            self.rank.cmp(&other.rank)
        } else {
            for i in 0..self.hand.len() {
                if self.hand[i] == other.hand[i] {
                    continue
                } else {
                    return self.values[&self.hand[i]].cmp(&other.values[&other.hand[i]])
                }
            }
            return self.hand[0].cmp(&other.hand[0])
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Hand {
    pub fn create(hand: Vec<char>, bid: usize,) -> Self {
        let rank = Self::calculate_rank(&hand);
        let values = HashMap::from([
            ('A', 13),
            ('K', 12),
            ('Q', 11),
            ('J', 10),
            ('T', 9),
            ('9', 8),
            ('8', 7),
            ('7', 6),
            ('6', 5),
            ('5', 4),
            ('4', 3),
            ('3', 2),
            ('2', 1),
        ]);

        Self {
            hand, rank, bid, values
        }
    }

    fn calculate_rank(hand: &Vec<char>) -> usize {
        let mut sorted: Vec<char> = vec![];
        
        hand.iter().for_each(|c| {
            sorted.push(*c);
        });

        sorted.sort();

        let mut count: usize = 0;
        let mut curr_char = sorted[0];
        let mut summary: Vec<usize> = vec![];

        for char in &sorted {
            if *char == curr_char {
                count += 1
            } else {
                summary.push(count);
                count = 1;
            }
            curr_char = *char;
        }
        summary.push(count);

        summary.sort();

        return if summary == vec![5] {
            7
        } else if summary == vec![1, 4] {
            6
        } else if summary == vec![2, 3] {
            5
        } else if summary == vec![1, 1, 3] {
            4
        } else if summary == vec![1, 2, 2] {
            3
        } else if summary == vec![1, 1, 1, 2] {
            2
        } else if summary == vec![1, 1, 1, 1, 1] {
            1
        } else {
            panic!("unknown value");
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hand: String = self.hand.iter().collect();
        write!(f, "{} => ({}, {})", self.rank, hand, self.bid)
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut total_winnings = 0;
    
    let lines: Vec<_> = input.split("\n").collect();
    let mut hands: Vec<Hand> = vec![];

    for line in lines {
        hands.push(parse_line(line));
    }

    hands.sort_by(|a, b| a.cmp(b));

    let mut current_level = 1;
    let mut current_hand = &hands[0];

    for hand in &hands {
        if !hand.cmp(&current_hand).is_eq() {
            current_level += 1
        }
        total_winnings += current_level * hand.bid;
        current_hand = hand;
    }
    
    Ok(total_winnings.to_string())
}

fn parse_line(line: &str) -> Hand {
    let sides: Vec<_> = line.split(" ").collect();
    let hand = sides[0].chars().collect();
    Hand::create(
        hand,
        parse_number(&sides[1]),
    )
}

fn parse_number(number_str: &str) -> usize {
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
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let output = process(input.trim());
        assert_eq!(6440.to_string(), output?);

        Ok(())
    }

}


