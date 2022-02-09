use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value_t = 80)]
    pub port: u16,

    #[clap(long)]
    pub cors_hostname: String,

    #[clap(long, default_value = "https")]
    pub cors_protocol: String,
}

pub fn get_args() -> Args {
    Args::parse()
}
