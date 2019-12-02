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

    let mut ans: Vec<usize> = contents
        .split(",")
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    ans[1] = 12;
    ans[2] = 2;

    for i in (0..ans.len()).step_by(4) {
        if ans[i] == 99 {
            println!("Part 1: Ans -> {:?}", ans[0]);
        }

        if i + 3 < ans.len() {
            match (ans[i], ans[i + 1], ans[i + 2], ans[i + 3]) {
                (1, x, y, z) => {
                    ans[z] = ans[x] + ans[y];
                }
                (2, x, y, z) => {
                    ans[z] = ans[x] * ans[y];
                }
                _ => {}
            }
        }
    }

    for x in 0..100 {
        for y in 0..100 {
            let mut n : Vec<usize> = contents
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
            n[1] = x;
            n[2] = y;
            if calc(&mut n) {
                println!("Part 2: Ans -> {}", 100 * x + y);
                break;
            }
        }
    }
}

fn calc(ans: &mut Vec<usize>) -> bool {
    for i in (0..ans.len()).step_by(4) {
        if ans[i] == 99 {
            if ans[0] == 19690720 {
                return true;
            }
            return false;
        }

        match (ans[i], ans[i + 1], ans[i + 2], ans[i + 3]) {
            (1, x, y, z) => {
                ans[z] = ans[x] + ans[y];
            }
            (2, x, y, z) => {
                ans[z] = ans[x] * ans[y];
            }
            _ => {}
        }
    }

    false
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
