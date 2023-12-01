const INPUT: &str = include_str!("../../part2.txt");

fn main() {
    let result = INPUT
        .lines()
        .map(|l| {
            let mut lmao: Vec<_> = l
                .match_indices("one")
                .chain(l.match_indices("two"))
                .chain(l.match_indices("three"))
                .chain(l.match_indices("four"))
                .chain(l.match_indices("five"))
                .chain(l.match_indices("six"))
                .chain(l.match_indices("seven"))
                .chain(l.match_indices("eight"))
                .chain(l.match_indices("nine"))
                .chain(l.match_indices("1"))
                .chain(l.match_indices("2"))
                .chain(l.match_indices("3"))
                .chain(l.match_indices("4"))
                .chain(l.match_indices("5"))
                .chain(l.match_indices("6"))
                .chain(l.match_indices("7"))
                .chain(l.match_indices("8"))
                .chain(l.match_indices("9"))
                .map(|(i, l)| match l {
                    "one" => (i, 1),
                    "two" => (i, 2),
                    "three" => (i, 3),
                    "four" => (i, 4),
                    "five" => (i, 5),
                    "six" => (i, 6),
                    "seven" => (i, 7),
                    "eight" => (i, 8),
                    "nine" => (i, 9),
                    "1" => (i, 1),
                    "2" => (i, 2),
                    "3" => (i, 3),
                    "4" => (i, 4),
                    "5" => (i, 5),
                    "6" => (i, 6),
                    "7" => (i, 7),
                    "8" => (i, 8),
                    "9" => (i, 9),
                    _ => unreachable!(),
                })
                .collect();

            lmao.sort_by(|a, b| a.cmp(&b));

            lmao.first().unwrap().1 * 10 + lmao.last().unwrap().1
        })
        .sum::<u32>();

    println!("{result}");
}
