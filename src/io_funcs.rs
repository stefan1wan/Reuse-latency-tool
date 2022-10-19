
pub mod io_funcs{
    use std::fs::{File, read};
    use std::io::{self, BufRead, prelude::*};
    use std::time::Instant;
    use std::path::Path;
    use std::convert::TryInto;

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

    pub fn parse_txt_file_stdin() -> Vec<u64>{
        let mut pid_list: Vec<u32> = vec![];
        let mut pc_list: Vec<u64> = vec![];

        let mut my_count:u64 = 0;

        let mut lines = io::stdin().lock().lines();
        while let Some(line) = lines.next()  {
                let info = line.unwrap();
                    // println!("{}", info);
                let split: Vec<_> = info.split_whitespace().collect();
                
                let pid: u32 = split[0].parse().unwrap();
                let pc: u64 = u64::from_str_radix(split[1], 16).unwrap();
                
                pid_list.push(pid);
                pc_list.push(pc);
            
                my_count=my_count+1;
                if my_count%10000000==0{
                    println!("count: {}", my_count);
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

}