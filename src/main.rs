mod hash;
use crate::hash::algorithm::Solution;

fn main() {
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("two_sum result: {:?}", result);

    let anagrams = Solution::group_anagrams(vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ]);
    println!("group_anagrams result: {:?}", anagrams);
}