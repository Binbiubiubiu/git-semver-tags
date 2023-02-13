use clap::{Parser, Subcommand};

/// The command line specifies the parameters
#[derive(Parser, Debug, Default)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    /// parse lerna style git tags
    #[arg(long, default_value_t = false)]
    pub(crate) lerna: bool,

    /// when listing lerna style tags, filter by a package
    #[arg(long, value_name = "package", requires = "lerna")]
    pub(crate) package: Option<String>,

    /// prefix to remove from the tags during their processing
    #[arg(long, value_name = "prefix")]
    pub(crate) tag_prefix: Option<String>,

    /// the current path where the command was run
    #[arg(long, value_name = "cwd")]
    pub(crate) cwd: Option<String>,

    /// ignore unstable labels
    #[arg(long)]
    pub(crate) skip_unstable: bool,

    /// subcommand
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// subcommand
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// upgrade self version
    #[cfg(feature = "self_upgrade")]
    #[cfg_attr(docsrs, doc(cfg(feature = "self_upgrade")))]
    Upgrade,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
