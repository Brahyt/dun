use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    did: Option<String>,

    #[arg(short, long)]
    yesterday: bool
}

fn main() {
    let args = Args::parse();

    if let Some(did) = args.did {
        println!("It appears that you did {}", did);
    } else if args.yesterday {
        println!("Yesterdays events");
    } else {
        println!("Gotta put something");
    }
}
