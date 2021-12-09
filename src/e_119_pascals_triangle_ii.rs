//Given an integer rowIndex, return the rowIndexᵗʰ (0-indexed) row of the
//Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly
//above it as shown:
//
//
// Example 1:
// Input: rowIndex = 3
//Output: [1,3,3,1]
// Example 2:
// Input: rowIndex = 0
//Output: [1]
// Example 3:
// Input: rowIndex = 1
//Output: [1,1]
//
//
// Constraints:
//
//
// 0 <= rowIndex <= 33
//
//
//
// Follow up: Could you optimize your algorithm to use only O(rowIndex) extra
//space?
// Related Topics Array Dynamic Programming 👍 1998 👎 246


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(get_row(1), vec![1, 1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(get_row(0), vec![1]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn get_row(row_index: i32) -> Vec<i32> {
    if row_index == 0 {
        return vec![1];
    } else {
        let mut i = 1;
        let mut r = vec![1, 1];
        loop {
            if i == row_index {
                return r;
            }
            r = r.windows(2).map(|pair| pair.iter().sum()).collect();
            r.insert(0, 1);
            r.push(1);
            i += 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
