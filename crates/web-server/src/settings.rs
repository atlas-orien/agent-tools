use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "agent-tools-server")]
#[command(about = "HTTP service for agent-tools")]
pub struct Settings {
    #[arg(long, default_value = "0.0.0.0:18080")]
    pub bind: String,
}
