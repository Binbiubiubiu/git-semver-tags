use clap::Parser;
use git_semver_tags::{captures, Args,Commands, self_upgrade};

fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Upgrade) => {
            self_upgrade(false).expect("upgrade success")
        },
        _ => {
            for tag in captures(&args).iter() {
                println!("{}", tag);
            }
        }
    }
}
