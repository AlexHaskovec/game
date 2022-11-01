use std::net::{IpAddr,Ipv4Addr};

use bevy::prelude::*;

use crate::{args::GameArgs,};
use clap::Parser;

pub struct ArgParsePlugin;

impl Plugin for ArgParsePlugin {
    fn build(&self, app: &mut App) {
        let cl_args = GameArgs::parse();
        app.insert_resource(cl_args); //Where does this live now?
    }
}


// //These methods offer an alternative way to obtain the CLAs
// fn server_get_port()->u16{
//     let a = GameArgs::parse();
//         match a {
//             GameArgs::Server(server) => {
//                 return server.open_port;
//             },
//             GameArgs::Client(client) => {
//                 return 0;
//             },
//         }
// }

// fn server_get_filename()->String{
//     let a = GameArgs::parse();
//     match a{
//         GameArgs::Server(server) => return server.filename,
//         GameArgs::Client(client) => return String::from("NOFILE"),
//     }
// }

// fn client_get_server_addr() -> IpAddr{
//     let a = GameArgs::parse();
//     match a{
//         GameArgs::Server(server) => return IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
//         GameArgs::Client(client) => return client.server_ip,
//     }
// }

// fn client_get_port_num() -> u16{
//     let a = GameArgs::parse();
//     match a{
//         GameArgs::Server(server) => return 0,
//         GameArgs::Client(client) => return client.connect_to_port,
//     }
// }