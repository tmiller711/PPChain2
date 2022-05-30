use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::{thread, time::Duration};
use crate::nodes::{new_node, find_nodes};

// run name is due to change. Possibly make it main but IDK
pub fn run()
{
    let listener = TcpListener::bind("192.168.0.21:7878").unwrap();

    // if it's the first time booting up in a while. add argv arguments
    //findnodes();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut connected = true;
    while connected {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        let msg = String::from_utf8_lossy(&buffer[..]);
        let msg = msg.trim_end_matches("\0");

        //println!("{msg}");

        // do if statements to see what the msg is and see what the network should do in response
        if msg.contains("new node") {
            new_node(msg.trim_start_matches("new node : "), &stream);
            stream.write(b"accepted").unwrap();
            stream.flush().unwrap();
        }
        if msg.contains("known node") {
            let mut nodes_file = File::options().append(true).open("nodes.txt").unwrap();
            nodes_file.write_all(msg.trim_start_matches("known node : ").as_bytes()).unwrap()
        }
        if msg.contains("!DISCONNECT") {
            connected = false;
        }
    }
}