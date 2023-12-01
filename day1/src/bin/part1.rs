const INPUT: &str = include_str!("../../part1.txt");

fn main() {
    let result = INPUT
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
        .sum::<u32>();

    println!("{result}");
}
