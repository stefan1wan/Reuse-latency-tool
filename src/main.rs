use rltool::rl;
use std::env;

fn main() {
    println!("Usage: ./rltool <binary file path>");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let pc_list = rl::read_file_text(filename);
    rl::rl(&pc_list);
}

