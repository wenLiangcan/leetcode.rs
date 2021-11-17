//Given an integer array nums, return true if any value appears at least twice
//in the array, and return false if every element is distinct.
//
//
// Example 1:
// Input: nums = [1,2,3,1]
//Output: true
// Example 2:
// Input: nums = [1,2,3,4]
//Output: false
// Example 3:
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
//Output: true
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -10⁹ <= nums[i] <= 10⁹
//
// Related Topics Array Hash Table Sorting 👍 2919 👎 891

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test_3() {
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for i in nums.into_iter() {
        if !set.insert(i) {
            return true;
        }
    }
    return false;
}
//leetcode submit region end(Prohibit modification and deletion)
