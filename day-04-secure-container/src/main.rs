use fancy_regex::Regex;

fn main() {
    let re = Regex::new(r"^1*2*3*4*5*6*7*8*9*$").unwrap();
    let re2 = Regex::new(r"^\d*(\d)\1(?!\1)\d*$").unwrap();

    println!(
        "{:#?}",
        (136818..685980)
            .map(|x| x.to_string())
            .filter(|val| re.is_match(&val).unwrap())
            .filter(|val| re2.is_match(&val).unwrap())
            .map(|x| x.split().map())
            // .collect::<Vec<String>>()
            .count()
    )
}
