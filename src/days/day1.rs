pub fn solve(input: Vec<String>) -> u32 {
    let mut password: u32 = 0;
    let mut counter: i32 = 50;

    for i in input {
        let split = i.split_at(1);
        let direction = split.0;

        let amount: i32 = split.1.parse().unwrap();

        if direction == "R" {
            counter = counter + amount;
        } else {
            counter = counter - amount;
        }

        if counter < 0 {
            counter = counter % 100;
        } else if counter > 99 {
            counter = counter % 100;
        }

        if counter == 0 {
            password = password + 1;
        }
    }

    password
}

#[cfg(test)]
mod tests {
    use crate::days::day1::solve;

    #[test]
    fn test_input() {
        let input: Vec<String> = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .lines()
        .map(|line| line.to_string())
        .collect();

        let actual = solve(input);

        assert_eq!(3, actual);
    }

    #[test]
    fn test_right_click() {
        let input: Vec<String> = "R49
R01"
        .lines()
        .map(|line| line.to_string())
        .collect();

        let actual = solve(input);

        assert_eq!(1, actual);
    }

    #[test]
    fn test_left_click() {
        let input: Vec<String> = "L49
L01"
        .lines()
        .map(|line| line.to_string())
        .collect();

        let actual = solve(input);

        assert_eq!(1, actual);
    }
}
