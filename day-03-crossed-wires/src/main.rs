use std::collections::{HashMap, HashSet};
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

    let ans: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split(",").collect())
        .collect();

    let points_1 = get_path(&ans[0]);
    let points_2 = get_path(&ans[1]);

    let path_1 = get_coords(&points_1);
    let path_2 = get_coords(&points_2);

    let common: HashSet<_> = path_1.intersection(&path_2).collect();

    let ans_1 = common.iter().map(|(x, y)| x.abs() + y.abs()).min();

    println!("Part 1: Ans -> {:?}", ans_1);

    let ans_2 = common
        .iter()
        .map(|k| points_1.get(&k).unwrap() + points_2.get(&k).unwrap())
        .min();

    println!("Part 2: Ans -> {:?}", ans_2);
}

fn get_coords(path: &HashMap<(i32, i32), usize>) -> HashSet<&(i32, i32)> {
    path.keys().collect()
}

fn get_path(path: &Vec<&str>) -> HashMap<(i32, i32), usize> {
    let mut x = 0;
    let mut y = 0;
    let mut length = 0;

    let mut path_map = HashMap::new();

    for i in path {
        let (s_x, s_y, dist) = get_dir(i);
        for _ in 0..dist {
            x += s_x;
            y += s_y;
            length += 1;
            let k = (x, y);

            if !path_map.contains_key(&k) {
                path_map.insert((x, y), length);
            }
        }
    }

    path_map
}

fn get_dir(dir: &str) -> (i32, i32, i32) {
    let mut step_x = 0;
    let mut step_y = 0;
    match &dir[0..1] {
        "R" => step_x = 1,
        "L" => step_x = -1,
        "U" => step_y = 1,
        "D" => step_y = -1,
        _ => unimplemented!(),
    }

    (step_x, step_y, dir[1..].parse().unwrap())
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
