fn main() {
    let input = include_str!("input_1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut first_number = String::new();
        let mut last_number = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                if first_number.is_empty() {
                    first_number = c.to_string();
                }
                last_number = c.to_string();
            }
        }
        let combined_numbers = format!("{first_number}{last_number}");
        let combined_numbers: u32 = combined_numbers.parse().unwrap();
        total += combined_numbers;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: u32 = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
