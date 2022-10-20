#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_rl() {
        let result = rl::rl_improved(&vec![1, 2, 3, 4, 1, 1, 2, 3, 5, 1, 1]);
        let ground_truth:Vec<u64> = vec![3, 3, 3, 0xffffffffffffffff,0, 3, 0xffffffffffffffff,0xffffffffffffffff,0xffffffffffffffff,0 , 0xffffffffffffffff];
        println!("{:?}", result);
        println!("{:?}", ground_truth);

        for i in 0..result.len(){
            assert!(result[i]==ground_truth[i]);
        }
    }

    #[test]
    fn test_for_rl_2() {
        let result = rl::rl_improved(&vec![1, 2, 3, 4, 1, 1, 2, 3, 5, 1, 1, 3, 3]);
        let ground_truth:Vec<u64> = vec![3, 3, 3, 0xffffffffffffffff,0, 3, 0xffffffffffffffff, 2,0xffffffffffffffff,0 , 0xffffffffffffffff, 0, 0xffffffffffffffff];
        println!("{:?}", result);
        println!("{:?}", ground_truth);

        for i in 0..result.len(){
            assert!(result[i]==ground_truth[i]);
        }
    }
    
    #[test]
    fn test_for_rl_bytes(){
        let v1 = vec![1, 2, 3, 4, 1, 1, 2, 3, 5, 1, 1, 3, 3];
        let v2 = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
        let result = rl::rl_by_bytes(&v1, &v2);
        let notfound:u64 = (v1.len()*100).try_into().unwrap();
        
        let ground_truth:Vec<u64> = vec![30, 30, 30,notfound, 0, 30, notfound, 20,notfound,0 , notfound, 0, notfound];
        println!("{:?}", result);
        println!("{:?}", ground_truth);

        for i in 0..result.len(){
            assert!(result[i]==ground_truth[i]);
        }
    }
}
