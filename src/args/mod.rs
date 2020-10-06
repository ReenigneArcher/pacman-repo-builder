mod outdated;
mod print_config;
mod sort;

pub use outdated::{OutdatedArgs, OutdatedDetails};
pub use print_config::PrintConfigArgs;
pub use sort::SortArgs;

use argh::*;

#[derive(Debug, FromArgs)]
#[argh(description = "Build a custom pacman repository from a collection of PKGBUILD directories")]
pub struct Args {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Sort(SortArgs),
    PrintConfig(PrintConfigArgs),
    Outdated(OutdatedArgs),
}
