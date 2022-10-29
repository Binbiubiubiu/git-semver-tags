use clap::Parser;
use git_semver_tags::{captures, Args};

fn main() {
    let args = Args::parse();
    for tag in captures(&args).iter(){
        println!("{}",tag);
    }
}
