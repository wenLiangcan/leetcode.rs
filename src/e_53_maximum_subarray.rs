//Given an integer array nums, find the contiguous subarray (containing at
//least one number) which has the largest sum and return its sum.
//
// A subarray is a contiguous part of an array.
//
//
// Example 1:
//
//
//Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//Output: 6
//Explanation: [4,-1,2,1] has the largest sum = 6.
//
//
// Example 2:
//
//
//Input: nums = [1]
//Output: 1
//
//
// Example 3:
//
//
//Input: nums = [5,4,-1,7,8]
//Output: 23
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -10⁴ <= nums[i] <= 10⁴
//
//
//
// Follow up: If you have figured out the O(n) solution, try coding another
//solution using the divide and conquer approach, which is more subtle.
// Related Topics Array Divide and Conquer Dynamic Programming 👍 15920 👎 747


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;
    use std::fs::OpenOptions;

    #[test]
    fn test() {
        let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(input), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[derive(Deserialize)]
    struct TestData {
        input: Vec<i32>,
        output: i32
    }

    #[test]
    fn test_4() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/e_53_maximum_subarray/data.json");
        let data: TestData = serde_json::from_reader(OpenOptions::new()
            .read(true).open(d).unwrap()).unwrap();
        assert_eq!(max_sub_array(data.input), data.output);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut current_sum = nums[0];
    let mut largest_sum = current_sum;

    for i in nums.into_iter().skip(1) {
        current_sum = max(current_sum + i, i);
        largest_sum = max(current_sum, largest_sum);
    }

    largest_sum
}
//leetcode submit region end(Prohibit modification and deletion)
