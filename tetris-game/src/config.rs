use clap::Parser;

/// Customize the gameplay
#[derive(Debug, Parser)]
pub struct Config {
    /// Number of columns
    #[arg(short = 'W', long, default_value_t = tetris_core::board::WIDTH)]
    pub width: u32,
    /// Number of rows
    #[arg(short = 'H', long, default_value_t = tetris_core::board::HEIGHT)]
    pub height: u32,
}
