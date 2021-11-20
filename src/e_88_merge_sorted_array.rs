//You are given two integer arrays nums1 and nums2, sorted in non-decreasing
//order, and two integers m and n, representing the number of elements in nums1 and
//nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead
//be stored inside the array nums1. To accommodate this, nums1 has a length of m +
//n, where the first m elements denote the elements that should be merged, and the
//last n elements are set to 0 and should be ignored. nums2 has a length of n.
//
//
// Example 1:
//
//
//Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//Output: [1,2,2,3,5,6]
//Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
//The result of the merge is [1,2,2,3,5,6] with the underlined elements coming
//from nums1.
//
//
// Example 2:
//
//
//Input: nums1 = [1], m = 1, nums2 = [], n = 0
//Output: [1]
//Explanation: The arrays we are merging are [1] and [].
//The result of the merge is [1].
//
//
// Example 3:
//
//
//Input: nums1 = [0], m = 0, nums2 = [1], n = 1
//Output: [1]
//Explanation: The arrays we are merging are [] and [1].
//The result of the merge is [1].
//Note that because m = 0, there are no elements in nums1. The 0 is only there
//to ensure the merge result can fit in nums1.
//
//
//
// Constraints:
//
//
// nums1.length == m + n
// nums2.length == n
// 0 <= m, n <= 200
// 1 <= m + n <= 200
// -10⁹ <= nums1[i], nums2[j] <= 10⁹
//
//
//
// Follow up: Can you come up with an algorithm that runs in O(m + n) time?
// Related Topics Array Two Pointers Sorting 👍 2035 👎 212


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut l1 = vec![1, 2, 3, 0, 0, 0];
        let mut l2 = vec![2, 5, 6];
        merge(&mut l1, 3, &mut l2, 3);
        assert_eq!(l1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut l1 = vec![1];
        let mut l2 = vec![];
        merge(&mut l1, 1, &mut l2, 0);
        assert_eq!(l1, vec![1]);
    }

    #[test]
    fn test_3() {
        let mut l1 = vec![0];
        let mut l2 = vec![1];
        merge(&mut l1, 0, &mut l2, 1);
        assert_eq!(l1, vec![1]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut left_len = m as usize;
    let mut start = 0usize;
    'out: for b in nums2 {
        for i in start .. left_len {
            start += 1;
            if b < &mut nums1[i] {
                nums1[i..].rotate_right(1);
                nums1[i] = *b;
                left_len += 1;
                continue 'out;
            }
        }
        if start >= left_len {
            nums1[start] = *b;
            start += 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

fn merge_cheat(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in 0 .. n as usize {
        nums1[m as usize + i] = nums2[i];
    }
    nums1.sort();
}
