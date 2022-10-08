
pub mod rl{
    use std::fs::{File, read};
    use std::time::Instant;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn read_file_text(filename: &str) -> Vec<u64>{
        println!("In txt file {}", filename);
        let mut pid_list: Vec<u32> = vec![];
        let mut pc_list: Vec<u64> = vec![];

        if let Ok(lines) = read_lines(filename){
            for line in lines {
                if let Ok(info) = line {
                    println!("{}", info);
                    let split: Vec<_> = info.split_whitespace().collect();
                    
                    let pid: u32 = split[0].parse().unwrap();
                    let pc: u64 = u64::from_str_radix(split[1], 16).unwrap();
                    
                    pid_list.push(pid);
                    pc_list.push(pc);
                }      
            }  
        }
        pc_list
    }

    

    pub fn read_file_binary(filename: &str) -> Vec<u32>{
        println!("In binary file {}", filename);

        let contents = read(filename)
            .expect("Something went wrong reading the file");

        let mut pc_list: Vec<u32> = vec![];
        
        let t0 = Instant::now();
        let mut count = 0;
        for i in (0..contents.len()).step_by(4) {

            count += 1;
            let first = u32::from_ne_bytes(contents[i..i+4].try_into().unwrap());
            pc_list.push(first);
        }
        println!("{} ms, count: {}", t0.elapsed().as_millis(), count);
        

        // println!("With text:\n{:#?}", contents);
        // println!("With text:\n{:#?}", pc_list);
        pc_list
    }
    
    // TODO: set
    use std::collections::HashMap;
    fn rl_uniq(i:usize, j:usize, pc_list:&Vec<u64>) -> u64{

        let mut map = HashMap::new();
        for k in i..j{
            let key:u64 = pc_list[k];

            let value = map.entry(key).or_insert(0);
            *value += 1;

        }

        map.len().try_into().unwrap()
    }

    pub fn rl(pc_list: &Vec<u64>){
        println!("PC list:\n{:#?}", pc_list);
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
        println!("Reuse Distance list:\n{:#?}", rl_list);
    }

}