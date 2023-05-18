
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long="cfg", default_value="./cfg.json")]
    pub cfg_file_name: String,
} 

pub fn parse_arguments() -> Args {
    let args = Args::parse();

    return args;
}