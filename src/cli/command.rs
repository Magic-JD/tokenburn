use clap::Parser;

#[derive(Debug, Parser)]
pub struct ConfigArgs {
    #[arg(long, help = "Percent ramping up and down to the average burn rate.")]
    pub ramp: Option<i16>,

    #[arg(
        long,
        help = "How long the average should be spread over. Shorter will give high spikes but a quick fall off, longer will give a more active average but be less reactive. Value given in seconds."
    )]
    pub spread: Option<i16>,
}

#[derive(Debug, Parser)]
pub struct TaskArgs {
    #[arg(long, help = "Generate a default configuration file")]
    pub generate_config: bool,
}

#[derive(Debug, Parser)]
#[command(name = "tokenburn")]
#[command(about = "See how fast you are burning money!")]
#[command(version = env!("CARGO_PKG_VERSION"), author = "Joseph Daunt")]
#[command(after_help = "For more details, visit https://github.com/Magic-JD/tokenburn")]
pub struct Cli {
    #[command(flatten)]
    pub config: ConfigArgs,

    #[command(flatten)]
    pub task: TaskArgs,

}
