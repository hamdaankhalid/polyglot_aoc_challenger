use rand::Rng;

pub fn pick_lang() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..LANGUAGES.len());
    LANGUAGES[random_number]
}

pub fn pick_aoc_challenge_url() -> String {
    let mut rng = rand::thread_rng();
    let rand_year = rng.gen_range(2015..2022);
    let rand_day = rng.gen_range(1..25);
    format!("https://adventofcode.com/{}/day/{}", rand_year, rand_day)
}

const LANGUAGES: [&str; 14] = [
    "Golang",
    "Python",
    "Java",
    "C#",
    "Ruby",
    "Scala",
    "C++",
    "C",
    "F#",
    "OCaml",
    "Haskell",
    "Rust",
    "Javascript",
    "Typescript",
];
