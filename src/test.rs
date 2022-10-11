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
}
