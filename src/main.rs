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
    let bins_name = "bins.data";
    // (1) Parse and generate binary
    // let pc_list = rl::read_file_text(txt_name);
    // println!("read and parse txt done");
    // rl::write_to_binary(&pc_list, bin_name);
    // println!("generate done");

    // (2) Read binary and calculate 
    let pc_list = rl::read_from_binary(bin_name);
    let rl_list = rl::rl(&pc_list);
    rl::write_to_binary(&rl_list, out_name);
    println!("generate rl list done");
    // (3) Parse reuse latency to bins
    let rl_list = rl::read_from_binary(out_name);
    println!("read rl done");
    rl::rl_bins(&rl_list, bins_name);
    println!("generate bins done");
}

