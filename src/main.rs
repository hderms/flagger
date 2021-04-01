use clap::Clap;
pub mod flagger;
use flagger::commands::{fill_command, invert_command, set_command};
use flagger::{format_output, Representation};

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Dermot H. <dermot.thomas.haughey@gmail.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
    #[clap(short)]
    rep: Representation,
}
#[derive(Clap)]
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
#[derive(Clap)]
struct Fill {
    #[clap(short)]
    count: usize,
}

#[derive(Clap)]
struct Set {
    #[clap(short)]
    count: usize,
}

#[derive(Clap)]
struct Invert {
    #[clap(short)]
    count: usize,
    #[clap(short)]
    width: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let number = match opts.subcmd {
        SubCommand::Fill(fill_opts) => fill_command(fill_opts.count),

        SubCommand::Set(set_opts) => set_command(set_opts.count),
        SubCommand::Invert(invert_opts) => invert_command(invert_opts.count, invert_opts.width),
    };
    let output = format_output(number, opts.rep);

    println!("{}", output)
}
