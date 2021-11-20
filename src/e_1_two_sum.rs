//Given an array of integers nums and an integer target, return indices of the
//two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may
//not use the same element twice.
//
// You can return the answer in any order.
//
//
// Example 1:
//
//
//Input: nums = [2,7,11,15], target = 9
//Output: [0,1]
//Output: Because nums[0] + nums[1] == 9, we return [0, 1].
//
//
// Example 2:
//
//
//Input: nums = [3,2,4], target = 6
//Output: [1,2]
//
//
// Example 3:
//
//
//Input: nums = [3,3], target = 6
//Output: [0,1]
//
//
//
// Constraints:
//
//
// 2 <= nums.length <= 10⁴
// -10⁹ <= nums[i] <= 10⁹
// -10⁹ <= target <= 10⁹
// Only one valid answer exists.
//
//
//
//Follow-up: Can you come up with an algorithm that is less than O(n²) time
//complexity? Related Topics Array Hash Table 👍 26321 👎 848

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut output = two_sum(vec![2, 7, 11, 15], 9);
        output.sort();
        assert_eq!(output, vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let mut output = two_sum(vec![3, 2, 4], 6);
        output.sort();
        assert_eq!(output, vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let mut output = two_sum(vec![3, 3], 6);
        output.sort();
        assert_eq!(output, vec![0, 1]);
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, v) in nums.into_iter().enumerate() {
        let diff = target - v;
        if let Some(j) = map.get(&diff) {
            return vec![i as i32, *j as i32];
        } else {
            map.insert(v, i);
        }
    }

    panic!()
}
//leetcode submit region end(Prohibit modification and deletion)
