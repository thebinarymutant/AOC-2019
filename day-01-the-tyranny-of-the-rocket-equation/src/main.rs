use std::env;
use std::fs;

pub struct Config {
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        Config {
            filename: "src/input.txt".to_string(),
        }
    });

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    let ans: i32 = contents
        .lines()
        .map(|val| val.parse::<i32>().unwrap() / 3 - 2)
        .sum();

    println!("Part 1: Ans -> {}", ans);

    let ans: i32 = contents
        .lines()
        .map(|val| {
            let mut total = val.parse::<i32>().unwrap() / 3 - 2;
            let mut extra = total / 3 - 2;

            while extra > 0 {
                total += extra;
                extra = extra / 3 - 2;
            }
            total
        })
        .sum();

        println!("Part 2: Ans ->  {}", ans);
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("File path is missing");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}
