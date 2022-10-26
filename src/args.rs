use bevy::prelude::*;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct GameArgs {
    #[clap(subcommand)]
    pub game_mode: GameMode,
}

#[derive(Debug, Subcommand)]
pub enum GameMode {
    /// Server mode
    Server(OpenServer),

    /// Client mode
    Client(ConnectClient),
}

#[derive(Debug, Args)]
pub struct OpenServer {
    /// Port to open server on
    pub open_port: u16,

    /// File
    pub filename: String,
}

#[derive(Debug, Args)]
pub struct ConnectClient {
    /// address of server
    pub server_ip: String,

    /// port to connect to
    pub connect_to_port: u16,
}
