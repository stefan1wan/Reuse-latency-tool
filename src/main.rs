use rltool::rl;
use std::env;

fn main() {
    println!("Usage: ./rltool <binary file path>");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    // rl::read_file(filename);
    let pc_list = rl::read_file_text(filename);
    // let pc_list = rl::read_file_binary(filename);
    rl::rl(&pc_list);
}

