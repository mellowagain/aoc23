fn main() {
    let input = include_str!("input.txt");
    //let input = include_str!("example.txt");

    let pulls =  input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(game, cubes)| {
            (
                game.replacen("Game ", "", 1).parse::<i32>().unwrap(),
                cubes.split("; ").collect::<Vec<&str>>(),
            )
        })
        .map(|(game, cubes)| {
            (
                game,
                cubes
                    .iter()
                    .map(|&pull| Pull::parse(pull))
                    .collect::<Vec<Pull>>(),
            )
        }).collect::<Vec<(i32, Vec<Pull>)>>();

    let a: i32 = pulls.iter()
        .filter(|(game, pulls)| pulls.iter().all(|pull| pull.valid()))
        .map(|(game, _)| game)
        .sum();

    let b: i32 = pulls.iter()
        .map(|(game, pulls)| (game, pulls.iter().fold(
            Pull {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, pull| Pull {
                red: if pull.red > acc.red { pull.red } else { acc.red },
                green: if pull.green > acc.green { pull.green } else { acc.green },
                blue: if pull.blue > acc.blue { pull.blue } else { acc.blue },
            },
        )))
        .map(|(_, pull)| pull.red * pull.green * pull.blue)
        .sum();


    println!("part a: {a}");
    println!("part b: {b}");
}

struct Pull {
    red: i32,
    green: i32,
    blue: i32,
}

impl Pull {
    fn parse(input: &str) -> Pull {
        let pulls: Vec<&str> = input.split(", ").collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pull in pulls {
            let (amount, color) = pull.split_once(" ").unwrap();

            let amount = amount.trim().parse::<i32>().unwrap();

            match color {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                _ => unreachable!(),
            }
        }

        Self { red, green, blue }
    }

    fn valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
