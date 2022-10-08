
pub mod rl{
    use std::fs::{File, read};
    use std::io::prelude::*;
    use std::io::{self, BufRead};
    use std::time::Instant;
    use std::path::Path;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        io::BufReader::with_capacity(4096*4096, &file);
        Ok(io::BufReader::new(file).lines())
    }

    pub fn read_file_text(filename: &str) -> Vec<u64>{
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
                if(my_count%10000000==0){
                    println!("count: {}", my_count);
                }
            }  
        }
        pc_list
    }

    pub fn write_to_binary(pc_list:&Vec<u64>, filename: &str){
        let mut file = File::create(filename).unwrap();
        // Write a slice of bytes to the file
        // file.write_all(&pc_list);
        // file.write_all_vectored(&pc_list);
        for pc in pc_list{
            file.write(&pc.to_le_bytes()).ok(); // *pc, 8);
        }
    }

    pub fn read_from_binary(filename: &str) -> Vec<u64>{
        println!("In binary file {}", filename);

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
    

    use std::collections::HashSet;
    fn rl_uniq(i:usize, j:usize, pc_list:&Vec<u64>) -> u64{
        let mut map = HashSet::new();
        for k in i..j{
            map.insert(pc_list[k]);
        }
        map.len().try_into().unwrap()
    }


    pub fn rl(pc_list: &Vec<u64>) -> Vec<u64>{
        // println!("PC list:\n{:#?}", pc_list);
        let mut rl_list: Vec<u64> = vec![];

        for i in 0..pc_list.len(){
            let mut flag:bool = false;
            for j in i+1..pc_list.len(){
                if pc_list[i] == pc_list[j]{
                    // println!("{}:{}", i, j);
                    let reuse_distance = rl_uniq(i+1, j, &pc_list);
                    rl_list.push(reuse_distance);
                    flag=true;
                    break;
                }
            }
            if !flag{
                rl_list.push(pc_list.len().try_into().unwrap());
            }

        }
        // println!("Reuse Distance list:\n{:#?}", rl_list);
        rl_list
    }

}