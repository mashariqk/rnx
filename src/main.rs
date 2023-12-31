use clap::Parser;

use rnx::find_and_rename::chop;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the directory to fix
    #[arg(short, long)]
    path: String,

    /// Max length of the file name *INCLUDING* extension
    #[arg(short, long)]
    cap: usize,

    /// Ignore these extensions
    #[arg(short, long)]
    ignore: Option<String>,

    /// Whether to consider sub-directories
    #[clap(long, short)]
    sub: bool,

    /// Replace all non-ascii with printable ascii chars
    #[clap(long, short)]
    ascii: bool,

    /// No changes will be made if this is passed
    #[clap(long, short)]
    dry: bool,
}

fn main() {
    let args = Args::parse();
    chop(
        args.path.as_str(),
        args.sub,
        args.cap,
        args.ascii,
        args.dry,
        None,
        None,
        &args.ignore,
    )
}
