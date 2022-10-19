use rltool::rl;
use rltool::io_funcs::io_funcs;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let pc_data_name = "pc.data";
    let rl_data_name="rl.data";
    let bins_name = "bins.data";

     // (1) Parse and generate binary
    if args.len()==2{
        println!("Parsing from file");
        let filename = &args[1];
        let txt_name = &&filename;
        let pc_list = io_funcs::parse_txt_file(txt_name);
        println!("parse {} done\n", txt_name);
        io_funcs::write_to_binary(&pc_list, pc_data_name);
        println!("generate {} done.\n", pc_data_name);
    }else{
        let pc_list = io_funcs::parse_txt_file_stdin();
        io_funcs::write_to_binary(&pc_list, pc_data_name);
        println!("generate {} done.\n", pc_data_name);
    }
    
    


   
    
    

    // (2) Read binary and calculate 
    let pc_list = io_funcs::read_from_binary(pc_data_name);
    let rl_list = rl::rl_improved(&pc_list);
    // let rl_list = rl::rl(&pc_list);
    io_funcs::write_to_binary(&rl_list, rl_data_name);
    println!("generate {} done.\n", rl_data_name);

    // (3) Parse reuse latency to bins
    let rl_list = io_funcs::read_from_binary(rl_data_name);
    println!("read rl done");
    rl::rl_bins_for_pdf(&rl_list, bins_name);
    println!("generate {} done.\n", bins_name);
}

