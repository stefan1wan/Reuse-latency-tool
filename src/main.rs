use rltool::rl;
use std::{env, process::exit};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len()!=2{
        println!("Usage: ./rltool <binary file path>");
        exit(-1);
    }
    
    println!("{:?}", args);
    let filename = &args[1];

    let txt_name = &&filename;
    let bin_name = "pc.data";
    let out_name="rl.data";
    let bins_name = "bins.data";

    // (1) Parse and generate binary
    let pc_list = rl::parse_txt_file(txt_name);
    println!("parse {} done\n", txt_name);
    rl::write_to_binary(&pc_list, bin_name);
    println!("generate {} done.\n", bin_name);

    // (2) Read binary and calculate 
    let pc_list = rl::read_from_binary(bin_name);
    let rl_list = rl::rl_improved(&pc_list);
    // let rl_list = rl::rl(&pc_list);
    rl::write_to_binary(&rl_list, out_name);
    println!("generate {} done.\n", out_name);

    // (3) Parse reuse latency to bins
    let rl_list = rl::read_from_binary(out_name);
    println!("read rl done");
    rl::rl_bins_for_pdf(&rl_list, bins_name);
    println!("generate {} done.\n", bin_name);
}

