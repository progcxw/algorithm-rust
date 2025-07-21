use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // 1. 两数之和
    // 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
    // 你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。
    // 你可以按任意顺序返回答案。
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            // 通过迭代器穷举nums，列出数组下标i和元素num
            if let Some(&index) = map.get(&(target - num)) {
                // 在map中查找target-num，如果找到，返回下标i和index
                return vec![i as i32, index];
            }
            // 将num和下标i存入map, 方便使用target-num查找
            map.insert(num, i as i32);
        }

        // 数组遍历完都没找到目标，则返回空数组
        vec![]
    }

    // 49. 字母异位词分组
    // 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 预分配capacity，避免扩容时的损耗，usize用于记录每个排序后的 key 对应的 results 向量下标
        let mut map: HashMap<Vec<u8>, usize> = HashMap::with_capacity(strs.len());
        let mut results : Vec<Vec<String>> = Vec::with_capacity(strs.len());
        let mut index = 0;  // 当前插入到 results 的位置索引

        for s in strs {
            // 将字符串转为字节数组并排序，作为分组 key
            let mut key = s.clone().into_bytes();
            key.sort_unstable();

            // 判断该排序后的 key 是否已经存在于 map 中
            if let Some(&i) = map.get(&key) {
                // 已存在，直接将字符串添加到对应分组
                results[i].push(s);
            } else {
                // 不存在，新建分组并插入 map
                results.push(vec![s]);
                map.insert(key, index);
                index += 1;
            }
        }

        results
    }
}