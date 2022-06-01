mod network;
mod nodes;
mod blockchain;
use std::net::{TcpStream, SocketAddr};
use crate::nodes::{return_nodes};

fn main() {
    // if it's the first time booting up in a while. add argv arguments
    //blockchain::blockchain_startup();

    let first_time_startup = false;
    if first_time_startup {
        match TcpStream::connect("192.168.0.21:7878") {
            Ok(stream) => {
                nodes::node_bootstrap(stream);
            }
            Err(error) => {
                println!("Cannot connect to seed node for node bootstrapping");
            }
        }

        let nodes = return_nodes();
        match TcpStream::connect(&nodes[..]) {
            Ok(stream) => {
                blockchain::blockchain_startup(stream);
            }
            Err(error) => {
                println!("Cannot connect to node for blockchain");
            }
        }
    }

     network::start_node();
}
