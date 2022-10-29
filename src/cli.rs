use clap::Parser;

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
}

impl Args {

    /// specify the lerna tag
    pub fn lerna(&mut self) -> &mut Args {
        self.lerna = true;
        self
    }

    /// set tag prefix filters
    pub fn tag_prefix<P: AsRef<str>>(&mut self, dir: P) -> &mut Args {
        self.tag_prefix = Some(dir.as_ref().to_string());
        self
    }

    /// set the package name filter
    pub fn package_name<P: AsRef<str>>(&mut self, dir: P) -> &mut Args {
        self.package = Some(dir.as_ref().to_string());
        self
    }

    /// set the current path where the command is run
    pub fn current_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Args {
        self.cwd = Some(dir.as_ref().to_string());
        self
    }

    /// ignore unstable labels
    pub fn skip_unstable(&mut self) -> &mut Args {
        self.skip_unstable = true;
        self
    }
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
