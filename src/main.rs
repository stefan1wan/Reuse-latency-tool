use rltool::rl;
use std::env;

fn main() {
    println!("Usage: ./rltool <binary file path>");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];

    let txt_name = &&filename;
    let bin_name = "pc.data";
    let out_name="rl.data";
    // (1) Parse and generate binary
    let pc_list = rl::read_file_text(txt_name);
    print!("read and parse txt done");
    rl::write_to_binary(&pc_list, bin_name);
    print!("generate done");

    // (2) Read binary and calculate 
    // let pc_list = rl::read_from_binary(bin_name);
    // let rl_list = rl::rl(&pc_list);
    // rl::write_to_binary(&rl_list, out_name);
}

