use std::{cmp::min, cmp::Ordering, collections::HashMap, vec, i32::MAX};
// 测试在leetcode上完成
struct Solution;
impl Solution {
    //556. 下一个更大元素 III
    pub fn next_greater_element(n: i32) -> i32 {
        let mut mem_digits = vec![];
        let mut digits_map = HashMap::<i32, Vec<i32>>::new();
        let mut index = 0;
        let mut nn:i64 = n as i64;
        while nn > 0 {
            let curr_digit = (nn % 10) as i32;
            for v in curr_digit + 1..=9 {
                if digits_map.contains_key(&v) {
                    let latest_idx = digits_map.get(&v).unwrap().last().unwrap();
                    mem_digits[*latest_idx as usize] = curr_digit;
                    nn -= curr_digit as i64;
                    nn += v as i64;
                    mem_digits.sort();
                    mem_digits.iter().for_each(|&curr| {
                        nn *= 10;
                        nn += curr as i64;
                    });
                    return if nn > i32::MAX as i64 { -1 } else { nn as i32};
                }
            }
            mem_digits.push(curr_digit);
            digits_map
                .entry(curr_digit)
                .or_insert_with(|| Vec::new())
                .push(index);
            index += 1;
            nn /= 10;
        }
        -1
    }

    //剑指 Offer II 091. 粉刷房子
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; 3];
        let mut dp_next = vec![0; 3];
        let change_val = |dp1: &mut Vec<i32>, dp2: &Vec<i32>| {
            for i in 0..dp2.len() {
                dp1[i] = dp2[i];
            }
        };
        costs.iter().for_each(|v| {
            dp_next[0] = v[0] + min(dp[1], dp[2]);
            dp_next[1] = v[1] + min(dp[0], dp[2]);
            dp_next[2] = v[2] + min(dp[1], dp[0]);
            change_val(&mut dp, &dp_next);
        });
        return min(min(dp[0], dp[1]), dp[2]);
    }

    //剑指 Offer II 008. 和大于等于 target 的最短子数组
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::max_value();
        let mut l = 0;
        let mut r = 0;
        let mut curr = nums[0];
        while r < nums.len() {
            if curr < target {
                r += 1;
                if r == nums.len() {
                    break;
                }
                curr += nums[r];
            } else {
                res = res.min((r - l + 1) as i32);
                if l < r {
                    curr -= nums[l];
                    l += 1;
                } else {
                    r += 1;
                    if r == nums.len() {
                        break;
                    }
                    curr += nums[r];
                }
            }
        }
        if curr < target && l == 0 && r == nums.len() {
            res = 0;
        }
        res
    }

    //剑指 Offer II 006. 排序数组中两个数字之和
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();
        let mut l = 0;
        let mut r = n - 1;
        loop {
            match target.cmp(&(numbers[l] + numbers[r])) {
                Ordering::Greater => l += 1,
                Ordering::Less => r -= 1,
                Ordering::Equal => break,
            }
        }
        vec![l as i32, r as i32]
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
