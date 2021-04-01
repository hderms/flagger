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
#[derive(Clap, Clone)]
struct Fill {
    #[clap(short)]
    count: usize,
    #[clap(short, default_value="x")]
    rep: Representation,
    #[clap(long)]
    comment: bool,
}

#[derive(Clap, Clone)]
struct Set {
    #[clap(short)]
    count: usize,
    #[clap(short, default_value="x")]
    rep: Representation,
    #[clap(long)]
    comment: bool,
}

#[derive(Clap, Clone)]
struct Invert {
    #[clap(short)]
    count: usize,
    #[clap(short)]
    width: usize,
    #[clap(short, default_value="x")]
    rep: Representation,
    #[clap(long, about="Adds a comment to the end of the line describing what the flag is")]
    comment: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let number = match opts.subcmd.clone() {
        SubCommand::Fill(fill_opts) => fill_command(fill_opts.count),
        SubCommand::Set(set_opts) => set_command(set_opts.count),
        SubCommand::Invert(invert_opts) => invert_command(invert_opts.count, invert_opts.width),
    };

    let rep = match opts.subcmd.clone() {
        SubCommand::Fill(fill_opts) => fill_opts.rep,
        SubCommand::Set(set_opts) => set_opts.rep,
        SubCommand::Invert(invert_opts) => invert_opts.rep,
    };

    let comment = match opts.subcmd.clone() {
        SubCommand::Fill(fill_opts) => {
            if fill_opts.comment {
                format!(" #full bitmap of {} bits", fill_opts.count)
            } else {
                String::new()

            }

        },
        SubCommand::Set(set_opts) => {
            if set_opts.comment {
                format!(" #single bit set at index {}", set_opts.count)
            } else {
                String::new()

            }

        },
        SubCommand::Invert(invert_opts) => {
            if invert_opts.comment {
                format!(" #inverted bitmap at index {} of width {}", invert_opts.count - 1, invert_opts.width)
            } else {
                String::new()

            }

        },
    };
    let output = format_output(number, rep);

    println!("{}{}", output, comment)
}
