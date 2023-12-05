fn main() {
    let input = include_str!("./part1.txt");

    part1(input);
}

fn part1(input: &str) -> u32 {
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
    let mut num_str = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            num_str.push(c);
        }
    }

    let first = num_str.chars().next().unwrap();
    let last = num_str.pop().unwrap();
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

        let result = part1(input);
        assert_eq!(result, 210)
    }
}
