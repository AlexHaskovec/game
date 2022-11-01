use std::net::IpAddr;

use clap::{
    Parser,
};

#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
pub enum GameArgs{
    /// Server mode
    Server(OpenServer),

    /// Client mode
    Client(ConnectClient),
}


#[derive(Debug, Parser, Clone)]
pub struct OpenServer {
    /// Port to open server on
    pub open_port: u16,

    /// File
    pub filename: String,
}

#[derive(Debug, Parser, Clone)]
pub struct ConnectClient {
    /// address of server
    pub server_ip: IpAddr,

    /// port to connect to
    pub connect_to_port: u16,
}