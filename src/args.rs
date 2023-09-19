use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct YabfrArgs {
    /// the file to run
    pub file: String,
}

pub fn parse_args() -> YabfrArgs {
    let args = YabfrArgs::parse();
    args
}
