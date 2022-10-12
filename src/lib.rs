
pub mod rl{
    use std::fs::{File, read};
    use std::io::{self, BufRead, prelude::*};
    use std::time::Instant;
    use std::path::Path;
    use std::cmp::min;
    use itertools::Itertools;
    use std::convert::TryInto;
    use std::collections::HashMap;

    const DEBUG:bool=false;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        io::BufReader::with_capacity(4096*4096, &file);
        Ok(io::BufReader::new(file).lines())
    }

    pub fn parse_txt_file(filename: &str) -> Vec<u64>{
        println!("In txt file {}", filename);
        let mut pid_list: Vec<u32> = vec![];
        let mut pc_list: Vec<u64> = vec![];

        let mut my_count:u64 = 0;
        if let Ok(lines) = read_lines(filename){
            for line in lines {
                if let Ok(info) = line {
                    // println!("{}", info);
                    let split: Vec<_> = info.split_whitespace().collect();
                    
                    let pid: u32 = split[0].parse().unwrap();
                    let pc: u64 = u64::from_str_radix(split[1], 16).unwrap();
                    
                    pid_list.push(pid);
                    pc_list.push(pc);
                }
                my_count=my_count+1;
                if my_count%10000000==0{
                    println!("count: {}", my_count);
                }
            }  
        }
        pc_list
    }

    pub fn read_from_binary(filename: &str) -> Vec<u64>{
        println!("Read from binary file {}", filename);

        let contents = read(filename)
            .expect("Something went wrong reading the file");

        let mut pc_list: Vec<u64> = vec![];
        
        let t0 = Instant::now();
        let mut count:i64 = 0;
        for i in (0..contents.len()).step_by(8) {

            count += 1;
            let first = u64::from_ne_bytes(contents[i..i+8].try_into().unwrap());
            pc_list.push(first);
        }
        println!("{} ms, count: {}", t0.elapsed().as_millis(), count);
        
        pc_list
    }


    pub fn write_to_binary(pc_list:&Vec<u64>, filename: &str){
        println!("Write to binary file {}", filename);
        let mut file = File::create(filename).unwrap();
        for pc in pc_list{
            file.write(&pc.to_le_bytes()).ok(); // *pc, 8);
        }
    }

    

    pub fn rl(pc_list: &Vec<u64>) -> Vec<u64>{
        // println!("PC list:\n{:#?}", pc_list);
        const MAX_WORKING_SET:u64 = 30000000;
        let mut rl_list: Vec<u64> = vec![];

        let mut my_count:u64 = 0;
        for i in 0..pc_list.len(){
            let mut flag:bool = false;
            for j in i+1..min(pc_list.len(), MAX_WORKING_SET.try_into().unwrap()){
                if pc_list[i] == pc_list[j]{
                    // println!("{}:{}", i, j);
                    let reuse_distance:u64 = pc_list[i+1..j].iter().cloned().unique().collect_vec().len().try_into().unwrap();//rl_uniq(i+1, j, &pc_list);
                    rl_list.push(reuse_distance);
                    flag=true;
                    if DEBUG{
                        println!("Resuse latency found: {}", reuse_distance);
                    }
                    
                    break;
                }
            }
            if !flag{
                rl_list.push(pc_list.len().try_into().unwrap());
                if DEBUG{
                    println!("Resuse latency not found!");
                }
            }
            my_count=my_count+1;
            if my_count%100000==0 {
                println!("count: {}", my_count);
            }
        }
        // println!("Reuse Distance list:\n{:#?}", rl_list);
        rl_list
    }


    fn rl_uniq_map(i:usize, j:usize, pc_list:&Vec<u64>) -> HashMap<u64, u64>{
        let mut map = HashMap::new();
        for k in i..j{
            let count = map.entry(pc_list[k]).or_insert(0);
            *count +=1;
        }
        map
    }

    pub fn rl_improved(pc_list: &Vec<u64>) -> Vec<u64>{
        let mut rl_list: Vec<u64> = vec![0xffffffffffffffff; pc_list.len()];
        println!("Generated rl vector");
        let mut walk_map:HashMap<u64, u64>= HashMap::with_capacity(pc_list.len()); // avoid frequently changing size
        // Map: <pc, pc_index>

        let mut index:usize = 0;
        let mut just_hit:bool = false;
        let mut abs_latency:usize = 0;
        let mut uniq_map: HashMap<u64, u64> = HashMap::new();
        let mut last_resue_latency:usize = 0;
        let mut last_pc:u64 = 0;

        let t0 = Instant::now();
        for pc in pc_list{
            if walk_map.contains_key(&pc){
                if just_hit{
                    if DEBUG{
                        println!("index: {} abs_latency: {}, last_reuse_latency: {}, pc_list[index - abs_latency]: {}", index, abs_latency, last_resue_latency, pc_list[index - abs_latency]);
                    }
                    
                    if pc_list[index - abs_latency] == *pc && uniq_map.contains_key(pc){
                        if DEBUG{ println!("Hit cache");}
                        

                        let last_index:usize = walk_map[pc].try_into().unwrap();
                        rl_list[last_index] = last_resue_latency.try_into().unwrap();

                        // To matain the state of uniq_map;
                        let last_pc_count = uniq_map.entry(last_pc).or_insert(0);
                        *last_pc_count += 1;

                        let pc_count = uniq_map.entry(*pc).or_insert(0);
                        *pc_count -= 1;

                        if *pc_count==0 {
                            uniq_map.remove_entry(pc);
                        }

                        last_pc = *pc;
                        // abs_latency, last_resue_latency unchanged
                    }else{
                        just_hit = false;
                        if DEBUG{
                            uniq_map = HashMap::new();
                            abs_latency = 0;
                            last_resue_latency = 0;
                            last_pc = 0;
                        }

                    }
                }
                
                if !just_hit{
                    if DEBUG{  println!("Not Hit cache"); } 
                    just_hit = true;
                    let last_index:usize = walk_map[pc].try_into().unwrap();
                    uniq_map = rl_uniq_map(last_index+1, index, pc_list);
                    
                    let reuse_latency = uniq_map.len();
                    rl_list[last_index] =  reuse_latency.try_into().unwrap();
                    abs_latency = index - last_index;
                    last_resue_latency =  reuse_latency;
                    last_pc = *pc;
                }
            }else{
                if DEBUG{ println!("Not found\n");}
                just_hit = false;
            }

            walk_map.insert(*pc, index.try_into().unwrap()); // insert pc;
            
            index += 1;
            if index%1000000==0 {
                let percentage:f64 = 100.0 * (index as f64) / (pc_list.len() as f64);
                println!("count: {}, total: {}. ({:.1}%) {} ms", index, pc_list.len(), percentage, t0.elapsed().as_millis());
            }
        }
        rl_list
    }

    pub fn rl_bins_for_pdf(rl_list: &Vec<u64>, filename: &str){
        let mut map:HashMap<u64, u64> = HashMap::new();
        for i in 0..rl_list.len(){
            let count = map.entry(rl_list[i]).or_insert(0);
            *count += 1;
        }
        let mut file = File::create(filename).unwrap();
        for (rl, count) in &map{
            file.write(&rl.to_le_bytes()).ok(); 
            file.write(&count.to_le_bytes()).ok();
        }
    }

}
