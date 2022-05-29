use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs::File;

// run name is due to change. Possibly make it main but IDK
pub fn run()
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn findnodes()
{
    
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let msg = String::from_utf8_lossy(&buffer[..]);
    let msg = msg.trim_end_matches("\0");

    // do if statements to see what the msg is and see what the network should do in response
    // possibly 'if "new node" in msg {}
    if msg.contains("new node") {
        newnode(msg.trim_start_matches("new node : "))
        // add the node to current list and send the node to all nodes that this one already had
        // send the node to all nodes currently known
        // if node is not in file then add it
    }
}

fn newnode(msg: &str) {
    // check if node is already in file
    let nodesfile = File::open("nodes.txt").unwrap();
    let reader = BufReader::new(nodesfile);
    let mut alreadyknown = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == msg {
            println!("alerady here nigga");
            alreadyknown = true;
            break;
        }
    }

    if alreadyknown == false {
        let mut nodesfile = File::options().append(true).open("nodes.txt").unwrap();
        let msg = format!("{}{}", msg, "\n");
        nodesfile.write_all(msg.as_bytes()).unwrap()
    }
}