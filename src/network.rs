use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::{thread, time::Duration};
use crate::nodes::{new_node, node_bootstrap, check_for_node, return_nodes};
use crate::blockchain::{blockchain_startup, send_blocks};
use walkdir::WalkDir;

// run name is due to change. Possibly make it main but IDK
pub fn start_node()
{
    let listener = TcpListener::bind("192.168.0.21:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //println!("{:?}", stream);

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        let msg = String::from_utf8_lossy(&buffer[..]);
        let msg = msg.trim_end_matches("\0");

        if msg != "" {
            println!("{:?}", msg);
        }

        // do if statements to see what the msg is and see what the network should do in response
        if msg.contains("new node") {
            new_node(msg.trim_start_matches("new node : "), &stream);
            stream.write(b"accepted").unwrap();
            stream.flush().unwrap();
            break;
        }
        if msg.contains("*node") {
            let msg = msg.trim_start_matches("*node : ");
            if !check_for_node(msg) {
                let msg = format!("{}\n", msg);
                let mut nodes_file = File::options().append(true).open("nodes.txt").unwrap();
                nodes_file.write_all(msg.as_bytes()).unwrap();
            }
            break;
        }
        if msg.contains("current height : ") {
            // converts their message to an int of their current chain height
            let their_height = (msg.trim_start_matches("current height : ").to_string()).parse::<i32>().unwrap();
            println!("Their height: {}", their_height);
            let current_height = WalkDir::new("blocks").into_iter().count() as i32;
            if their_height < current_height {
                // send them all the blocks they are missing
                send_blocks(stream, their_height);
            }
            
            break;
        }
        if msg.contains("!DISCONNECT") {
            break;
        }
    }
}