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

    let path_1 = get_path(&ans[0]);
    let path_2 = get_path(&ans[1]);

    let line_1 = get_coords(&path_1);
    let line_2 = get_coords(&path_2);

    let common: HashSet<_> = line_1.intersection(&line_2).collect();

    let ans_1 = common.iter().map(|(x, y)| x.abs() + y.abs()).min();

    println!("Part 1: Ans -> {:?}", ans_1);

    let dist_1 = dist_map(&path_1);
    let dist_2 = dist_map(&path_2);

    let ans_2 = common.iter().map(|k| dist_1.get(&k).unwrap() + dist_2.get(&k).unwrap()).min();

    println!("Part 2: Ans -> {:?}", ans_2);
}

fn get_coords(path: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    path.iter().cloned().collect()
}

fn get_path(path: &Vec<&str>) -> Vec<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;

    let mut path_list = Vec::new();

    for i in path {
        let (s_x, s_y, dist) = get_dir(i);
        for _ in 0..dist {
            x += s_x;
            y += s_y;

            path_list.push((x, y));
        }
    }

    path_list
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

fn dist_map(path: &Vec<(i32, i32)>) -> HashMap<(i32, i32), usize> {
    let mut dist = HashMap::new();

    for (ind, &x) in path.iter().enumerate() {
        if !dist.contains_key(&x) {
            dist.insert(x, ind + 1);
        }
    }

    dist
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
