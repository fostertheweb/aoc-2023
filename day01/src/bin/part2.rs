fn main() {
    let input = include_str!("./part1.txt");

    part2(input);
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let lines = input.lines();

    for line in lines {
        let num = parse_int(line).unwrap();
        sum += num;
    }

    println!("{}", sum);
    sum
}

fn parse_int(s: &str) -> Option<u32> {
    let num_mapping = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut num_str: Vec<(usize, char)> = Vec::new();

    for (word, value) in num_mapping {
        let _ = s
            .match_indices(word)
            .map(|(start, _)| {
                let found_num = value.chars().next().unwrap();
                num_str.push((start, found_num));
                (start, found_num)
            })
            .collect::<Vec<(usize, char)>>();
    }

    for (index, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            num_str.push((index, c));
        }
    }

    num_str.sort_by_key(|pair| pair.0);

    println!("{:?}", num_str);

    let first = num_str.first().unwrap().1;
    let last = num_str.pop().unwrap().1;
    let two_digits = format!("{}{}", first, last);

    two_digits.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int_from_str() {
        let result = parse_int("pqr3st4u8vw").unwrap();
        assert_eq!(result, 38);
    }

    #[test]
    fn test_sum_integers() {
        let input = "shrzvdcghblt21
sixdddkcqjdnzzrgfourxjtwosevenhg9
threevt1onegxgvc9flk
7dmqzksnlcpbsqkzqlfour1four";

        let result = part2(input);
        assert_eq!(result, 203)
    }
}
