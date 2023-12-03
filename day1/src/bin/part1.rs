const INPUT: &str = include_str!("../../input1.txt");

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut first = None;
            let mut last = None;

            for chara in l.chars() {
                if let Some(digit) = chara.to_digit(10) {
                    if first.is_none() {
                        first = Some(digit);
                    }
                    last = Some(digit);
                }
            }

            (first.unwrap() * 10) + last.unwrap()
        })
        .sum::<u32>()
}

fn main() {
    let result = solve(INPUT);

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::solve;

    const INPUT: &str = include_str!("../../example1.txt");

    #[test]
    fn it_works() {
        let result = solve(INPUT);

        assert_eq!(result, 142);
    }
}
