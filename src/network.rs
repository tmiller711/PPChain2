use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::{thread, time::Duration};
use crate::nodes::{new_node, node_bootstrap, check_for_node};

// run name is due to change. Possibly make it main but IDK
pub fn run()
{
    let listener = TcpListener::bind("192.168.0.21:7878").unwrap();

    // if it's the first time booting up in a while. add argv arguments
    //node_bootstrap();
    
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
            println!("{msg}");
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
        if msg.contains("!DISCONNECT") {
            break;
        }
    }
}