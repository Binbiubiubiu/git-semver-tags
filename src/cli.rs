use clap::Parser;

/// cansh
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
    #[arg(long, value_name = "prefix", default_value = "v")]
    pub(crate) tag_prefix: Option<String>,

    #[arg(skip)]
    pub(crate) cwd: Option<String>,

    #[arg(skip)]
    pub(crate) skip_unstable: bool,
}

impl Args {
    /// new
    pub fn new() -> Self {
        Default::default()
    }

    /// lerna
    pub fn lerna(&mut self) -> &mut Args {
        self.lerna = true;
        self
    }

    /// set package name
    pub fn tag_prefix<P: AsRef<str>>(&mut self, dir: P) -> &mut Args {
        self.tag_prefix = Some(dir.as_ref().to_string());
        self
    }

    /// set package name
    pub fn package_name<P: AsRef<str>>(&mut self, dir: P) -> &mut Args {
        self.package = Some(dir.as_ref().to_string());
        self
    }


    /// set current dir
    pub fn current_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Args {
        self.cwd = Some(dir.as_ref().to_string());
        self
    }

    /// skip
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
