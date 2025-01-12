use clap::Parser;

#[derive(Parser)]
pub struct CliArguments {
    #[clap(short, long)]
    pub file: Option<String>,
}
