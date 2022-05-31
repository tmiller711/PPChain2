mod network;
mod nodes;
mod blockchain;

fn main() {
    // if it's the first time booting up in a while. add argv arguments
    //nodes::node_bootstrap();
    //blockchain::blockchain_startup();

     network::start_node();
}
