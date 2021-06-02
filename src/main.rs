use clap::Clap;
pub mod flagger;
use crate::flagger::commands::{fill, invert, set};
use crate::flagger::{format_output, Representation};

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Dermot H. <dermot.thomas.haughey@gmail.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Clap, Clone)]
enum SubCommand {
    #[clap(about = "Fills a binary/hexadecimal with all 1s, according to the count")]
    Fill(Fill),
    #[clap(about = "sets a single bit on binary/hexadecimal according to the count")]
    Set(Set),
    #[clap(
        about = "inverse of setting a single bit on binary/hexadecimal according based on the count, up to a given width"
    )]
    Invert(Invert),
}
impl Commentary for SubCommand {
    fn commentary_string(&self) -> Option<String> {
        match self {
            SubCommand::Fill(f) => f.commentary_string(),
            SubCommand::Set(s) => s.commentary_string(),
            SubCommand::Invert(i) => i.commentary_string(),
        }
    }
}
trait Commentary {
    fn commentary_string(&self) -> Option<String>;
}
#[derive(Clap, Clone)]
struct Fill {
    #[clap(short)]
    count: usize,
    #[clap(short, default_value = "x")]
    rep: Representation,
    #[clap(long)]
    comment: bool,
}
impl Commentary for Fill {
    fn commentary_string(&self) -> Option<String> {
        if self.comment {
            Some(format!("full bitmap of {} bits", self.count))
        } else {
            None
        }
    }
}

#[derive(Clap, Clone)]
struct Set {
    #[clap(short)]
    count: usize,
    #[clap(short, default_value = "x")]
    rep: Representation,
    #[clap(long)]
    comment: bool,
}
impl Commentary for Set {
    fn commentary_string(&self) -> Option<String> {
        if self.comment {
            Some(format!("single bit set at index {}", self.count))
        } else {
            None
        }
    }
}

#[derive(Clap, Clone)]
struct Invert {
    #[clap(short)]
    count: usize,
    #[clap(short)]
    width: usize,
    #[clap(short, default_value = "x")]
    rep: Representation,
    #[clap(
        long,
        about = "Adds a comment to the end of the line describing what the flag is"
    )]
    comment: bool,
}
impl Commentary for Invert {
    fn commentary_string(&self) -> Option<String> {
        if self.comment {
            Some(format!(
                "inverted bitmap at index {} of width {}",
                self.count - 1,
                self.width
            ))
        } else {
            None
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let number = match opts.subcmd.clone() {
        SubCommand::Fill(fill_opts) => fill(fill_opts.count),
        SubCommand::Set(set_opts) => set(set_opts.count),
        SubCommand::Invert(invert_opts) => invert(invert_opts.count, invert_opts.width),
    };

    let rep = match opts.subcmd.clone() {
        SubCommand::Fill(fill_opts) => fill_opts.rep,
        SubCommand::Set(set_opts) => set_opts.rep,
        SubCommand::Invert(invert_opts) => invert_opts.rep,
    };

    let comment: Option<String> = opts.subcmd.commentary_string();
    let output = format_output(number, rep);

    match comment {
        Some(s) => {
            println!("{} #{}", output, s)
        }
        None => {
            println!("{}", output)
        }
    }
}
