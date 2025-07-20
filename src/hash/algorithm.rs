use std::collections::HashMap;

// 1. 两数之和
// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
// 你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。
// 你可以按任意顺序返回答案。
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            // 通过迭代器穷举nums，列出数组下标i和元素num
            if let Some(&index) = map.get(&(target - num)) {
                // 在map中查找target-num，如果找到，返回下标i和index
                return vec![i as i32, index];
            }
            // 如果没有找到，将num和下标i存入map
            map.insert(num, i as i32);
        }
        // 数组遍历完都没找到目标，则返回空数组
        vec![]
    }
}