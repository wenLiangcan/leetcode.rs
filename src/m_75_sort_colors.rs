//Given an array nums with n objects colored red, white, or blue, sort them in-
//place so that objects of the same color are adjacent, with the colors in the
//order red, white, and blue.
//
// We will use the integers 0, 1, and 2 to represent the color red, white, and
//blue, respectively.
//
// You must solve this problem without using the library's sort function.
//
//
// Example 1:
// Input: nums = [2,0,2,1,1,0]
//Output: [0,0,1,1,2,2]
// Example 2:
// Input: nums = [2,0,1]
//Output: [0,1,2]
// Example 3:
// Input: nums = [0]
//Output: [0]
// Example 4:
// Input: nums = [1]
//Output: [1]
//
//
// Constraints:
//
//
// n == nums.length
// 1 <= n <= 300
// nums[i] is 0, 1, or 2.
//
//
//
// Follow up: Could you come up with a one-pass algorithm using only constant
//extra space?
// Related Topics Array Two Pointers Sorting 👍 7799 👎 360

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![4, 5, 2, 3, 6, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut v = vec![-1, 5, 3, 4, 0];
        sort_colors(&mut v);
        assert_eq!(v, vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test_3() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_4() {
        let mut v = vec![2, 0, 1];
        sort_colors(&mut v);
        assert_eq!(v, vec![0, 1, 2]);
    }

    #[test]
    fn test_5() {
        let mut v = vec![0];
        sort_colors(&mut v);
        assert_eq!(v, vec![0]);
    }

    #[test]
    fn test_6() {
        let mut v = vec![1, 2, 0];
        sort_colors(&mut v);
        assert_eq!(v, vec![0, 1, 2]);
    }

    #[test]
    fn test_7() {
        let mut v = vec![1, 2, 0, 0, 0];
        sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 0, 1, 2]);
    }

    #[test]
    fn test_8() {
        let mut v = vec![1, 2, 2, 2, 2, 0, 0, 0, 1, 1];
        sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 2]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn sort_colors(nums: &mut Vec<i32>) {
    quick_sort(nums, 0, nums.len() - 1);
}

fn quick_sort(nums: &mut Vec<i32>, lo: usize, hi: usize) {
    if hi < lo {
        return;
    }
    let len = hi - lo;
    if len >= 1 {
        let k = len / 2 + lo;
        let n = nums[k];
        let mut i = lo;
        let mut j = hi;

        loop {
            while nums[i] < n {
                i += 1;
            }
            while nums[j] > n {
                j -= 1;
            }

            if i >= j || nums[i] == nums[j] {
                break;
            }

            nums.swap(i, j);
        }

        quick_sort(nums, lo, k);
        quick_sort(nums, k + 1, hi);

        for x in (lo ..= k).rev() {
            let mut k_r = x;
            while k_r < hi && nums[k_r] > nums[k_r + 1] {
                nums.swap(k_r, k_r + 1);
                k_r += 1;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
