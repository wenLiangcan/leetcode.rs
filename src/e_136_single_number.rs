//Given a non-empty array of integers nums, every element appears twice except
//for one. Find that single one.
//
// You must implement a solution with a linear runtime complexity and use only
//constant extra space.
//
//
// Example 1:
// Input: nums = [2,2,1]
//Output: 1
// Example 2:
// Input: nums = [4,1,2,1,2]
//Output: 4
// Example 3:
// Input: nums = [1]
//Output: 1
//
//
// Constraints:
//
//
// 1 <= nums.length <= 3 * 10⁴
// -3 * 10⁴ <= nums[i] <= 3 * 10⁴
// Each element in the array appears twice except for one element which appears
//only once.
//
// Related Topics Array Bit Manipulation 👍 7733 👎 265

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(single_number(vec![1]), 1);
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, ()> = HashMap::new();
    for i in nums.iter() {
        if let None = m.remove(i) {
            m.insert(i.to_owned(), ());
        }
    }
    m.keys().last().unwrap().to_owned()
}
//leetcode submit region end(Prohibit modification and deletion)
