//Given an array nums of size n, return the majority element.
//
// The majority element is the element that appears more than ⌊n / 2⌋ times.
//You may assume that the majority element always exists in the array.
//
//
// Example 1:
// Input: nums = [3,2,3]
//Output: 3
// Example 2:
// Input: nums = [2,2,1,1,1,2,2]
//Output: 2
//
//
// Constraints:
//
//
// n == nums.length
// 1 <= n <= 5 * 10⁴
// -2³¹ <= nums[i] <= 2³¹ - 1
//
//
//
//Follow-up: Could you solve the problem in linear time and in O(1) space?
//Related Topics Array Hash Table Divide and Conquer Sorting Counting 👍 7183 👎 297

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(majority_element(vec![3, 3, 4]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(majority_element(vec![1]), 1);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let boundary = nums.len() / 2;
    let mut current = nums[0];
    let mut count = 1usize;

    for i in nums.into_iter().skip(1) {
        if count > boundary {
            return current;
        }
        if i != current {
            current = i;
            count = 1;
        } else {
            count += 1;
        }
    }
    current
}
//leetcode submit region end(Prohibit modification and deletion)
