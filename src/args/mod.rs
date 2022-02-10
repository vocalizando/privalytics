use std::env;
use std::env::VarError;
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

#[derive(Parser, Debug, Clone)]
pub struct Env {
    pub master_key: String,
}

pub fn get_env() -> Result<Env, VarError> {
    let access_key = env::var("ACCESS_KEY")?;
    Ok(Env {
        master_key: access_key,
    })
}
