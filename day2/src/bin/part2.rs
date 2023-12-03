const INPUT: &str = include_str!("../../input2.txt");

fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let (_game, cube_sets) = l.split_once(':').unwrap();

            let (red, green, blue) =
                cube_sets
                    .split(';')
                    .fold((0, 0, 0), |(mut red, mut green, mut blue), set| {
                        let set = set.trim();

                        for cube in set.split(',') {
                            let cube = cube.trim();

                            let (count, r#type) = cube.split_once(' ').unwrap();

                            let count = count.parse::<i32>().unwrap();

                            if r#type == "red" {
                                red = red.max(count);
                            } else if r#type == "blue" {
                                blue = blue.max(count);
                            } else if r#type == "green" {
                                green = green.max(count);
                            }
                        }

                        // set max
                        (red, green, blue)
                    });

            red * green * blue
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

    const INPUT: &str = include_str!("../../example2.txt");

    #[test]
    fn it_works() {
        let result = solve(INPUT);

        assert_eq!(result, 2286);
    }
}
