use advent_of_code_2016::*;
use anyhow::anyhow;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    match opts.part.as_str() {
        "2-1" => println!("{}", p2_1()),
        "2-2" => println!("{}", p2_2()),
        "3-1" => println!("{}", p3_1()),
        "3-2" => println!("{}", p3_2()),
        _ => return Err(anyhow!("Unknown puzzle")),
    }
    Ok(())
}