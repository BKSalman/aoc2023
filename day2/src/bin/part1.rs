const INPUT: &str = include_str!("../../input1.txt");

fn solve(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|l| {
            let (game, cube_sets) = l.split_once(':')?;

            let (_game, id) = game.split_once(' ')?;

            let playable = cube_sets.split(';').all(|set| {
                let set = set.trim();
                let mut red = 12;
                let mut green = 13;
                let mut blue = 14;

                for cube in set.split(',') {
                    let cube = cube.trim();

                    let (count, r#type) = cube.split_once(' ').unwrap();

                    if r#type == "red" {
                        red -= count.parse::<i32>().unwrap();
                    } else if r#type == "blue" {
                        blue -= count.parse::<i32>().unwrap();
                    } else if r#type == "green" {
                        green -= count.parse::<i32>().unwrap();
                    }
                }

                red >= 0 && green >= 0 && blue >= 0
            });

            if playable {
                Some(id.parse::<i32>().unwrap())
            } else {
                None
            }
        })
        .sum::<i32>()
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

        assert_eq!(result, 8);
    }
}
