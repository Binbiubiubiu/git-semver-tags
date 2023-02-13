use clap::Parser;
use git_semver_tags::{captures, Args};
#[cfg(feature = "self_upgrade")]
use git_semver_tags::{self_upgrade, Commands};

fn main() {
    let args = Args::parse();
    match &args.command {
        #[cfg(feature = "self_upgrade")]
        Some(Commands::Upgrade) => self_upgrade(false).expect("upgrade success"),
        _ => {
            for tag in captures(&args).iter() {
                println!("{}", tag);
            }
        }
    }
}
