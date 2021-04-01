use std::str::FromStr;

use clap::Clap;
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Dermot H. <dermot.thomas.haughey@gmail.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Clap)]
enum SubCommand {
    #[clap()]
    Fill(Fill),
    Set(Set),
    Invert(Invert),
}
#[derive(Clap)]
struct Fill {
    #[clap(short)]
    count: usize,
    #[clap(short)]
    rep: Representation,
}

#[derive(Clap)]
struct Set {
    #[clap(short)]
    count: usize,
    #[clap(short)]
    rep: Representation,
}

#[derive(Clap)]
struct Invert {
    #[clap(short)]
    count: usize,
    #[clap(short)]
    width: usize,
    #[clap(short)]
    rep: Representation,
}

#[derive(Clap)]
enum Representation {
    Hex,
    Binary,
}
impl FromStr for Representation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Representation::Hex),
            "b" => Ok(Representation::Binary),
            _ => Err("no match"),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let output = match opts.subcmd {
        SubCommand::Fill(fill_opts) => {
            let num = fill(fill_opts.count);
            format_output(num, fill_opts.rep)
        }

        SubCommand::Set(set_opts) => {
            let num = 1 << (set_opts.count - 1);
            format_output(num, set_opts.rep)
        }
        SubCommand::Invert(invert_opts) => {
            let mut num: usize = fill(invert_opts.width);
            num ^= 1 << (invert_opts.count - 1);
            format_output(num, invert_opts.rep)
        }
    };

    println!("{}", output)
}

fn format_output(number: usize, representation: Representation) -> String {
    match representation {
        Representation::Hex => {
            format!("{:#x}", number)
        }
        Representation::Binary => {
            format!("{:#b}", number)
        }
    }
}
fn fill(count: usize) -> usize {
    let mut num: usize = 0;
    for i in 0..count {
        num |= 1 << i;
    }
    num
}
