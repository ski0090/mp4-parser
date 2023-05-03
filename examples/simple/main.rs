use clap::Parser;
use mp4_parser::header::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let mp4 = Mp4Header::parse(args.file);
    mp4.print_comp();
}
