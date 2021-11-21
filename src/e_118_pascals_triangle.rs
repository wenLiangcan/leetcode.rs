//Given an integer numRows, return the first numRows of Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly
//above it as shown:
//
//
// Example 1:
// Input: numRows = 5
//Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
// Example 2:
// Input: numRows = 1
//Output: [[1]]
//
//
// Constraints:
//
//
// 1 <= numRows <= 30
//
// Related Topics Array Dynamic Programming 👍 4095 👎 175

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(generate(5),
                   vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(generate(1), vec![vec![1]]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    (2 ..= num_rows).fold(vec![vec![1]], |mut acc, _| {
        let mut v: Vec<i32> = acc.last().unwrap()
            .windows(2)
            .map(|p| {p.iter().sum()})
            .collect();
        v.insert(0, 1);
        v.push(1);
        acc.push(v);
        acc
    })
}
//leetcode submit region end(Prohibit modification and deletion)
