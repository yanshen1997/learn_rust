use std::{vec, cmp::Ordering};
// 测试在leetcode上完成
struct Solution;
impl Solution {
    //剑指 Offer II 006. 排序数组中两个数字之和
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();
        let mut l = 0;
        let mut r = n - 1;
        loop {
            match target.cmp(&(numbers[l] + numbers[r])) {
                Ordering::Greater => l += 1,
                Ordering::Less=> r -= 1,
                Ordering::Equal => break,
            }
        }
        vec![l as i32,r as i32]
    }

    //剑指 Offer II 003. 前 n 个数字二进制中 1 的个数
    pub fn count_bits(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }
        if n == 1 {
            return vec![0, 1];
        }
        let mut res = vec![0, 1];
        let mut curr = 2;
        for i in 2..n + 1 {
            let curr_idx = (i & (curr - 1)) as usize;
            let curr_val = *res.get(curr_idx).unwrap();
            res.push(curr_val + 1);
            if i & (i - 1) == 0 {
                curr = i;
            }
        }
        res
    }
}
