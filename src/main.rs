extern crate simple;

use simple::{Hard, Middle, Simple};

#[derive(Simple)]
#[numbers(1, 2, 3, 4)]
struct JustStruct;

#[derive(Middle)]
#[values(tentacli=works, join=us, on=discord)]
struct BetterStruct;

#[derive(Hard)]
#[values(tentacli=works, join=us, on=discord)]
struct TopStruct {
    #[value("Tentacli")]
    name: String,
    #[value("https://github.com/idewave/tentacli")]
    github_link: String,
    #[value("https://crates.io/crates/tentacli")]
    crates_link: String,
}

fn main() {
    JustStruct::output();
    BetterStruct::output();
    TopStruct::output();
}
