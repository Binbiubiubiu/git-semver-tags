 # git-semver-tags

 ![Crates.io](https://img.shields.io/crates/v/git-semver-tags) ![Crates.io](https://img.shields.io/crates/d/git-semver-tags) ![Crates.io](https://img.shields.io/crates/l/git-semver-tags) [![CI](https://github.com/Binbiubiubiu/git-semver-tags/actions/workflows/CI.yml/badge.svg)](https://github.com/Binbiubiubiu/git-semver-tags/actions/workflows/CI.yml) [![codecov](https://codecov.io/gh/Binbiubiubiu/git-semver-tags/branch/main/graph/badge.svg?token=YTIKDKKKBV)](https://codecov.io/gh/Binbiubiubiu/git-semver-tags)


 Get all git semver tags of your repository in reverse chronological order

 ## Install

 Run
 ``` Console
 $ cargo install git-semver-tags
 ```

 ## Usage


 By default, it runs check. You can easily override this, though:

 ``` Console
 $ git-semver-tags [OPTIONS]
 ```

 A few examples:


 ``` Console
 # Run get all tags
 $ git-semver-tags

 # Run to get lerna tag
 $ git-semver-tags --lerna

 # Run the lerna tag to get the specified package name
 $ git-semver-tags --lerna --package <package>

 # Runs get tag for the specified prefix
 $ git-semver-tags --tag-prefix <prefix>

 # Run get to ignore unstable tag
 $ git-semver-tags --skip-unstable

 # Run get label under the specified path
 $ git-semver-tags --cwd <cwd>
 ```


 There's a lot more you can do! Here's a copy of the help:

 ``` Console
 Get all git semver tags of your repository in reverse chronological order

 Usage: git-semver-tags [OPTIONS]

 Options:
     --lerna                parse lerna style git tags
     --package <package>    when listing lerna style tags, filter by a package
     --tag-prefix <prefix>  prefix to remove from the tags during their processing
     --cwd <cwd>            the current path where the command was run
     --skip-unstable        ignore unstable labels
     -h, --help                 Print help information
     -V, --version              Print version information

 ```
