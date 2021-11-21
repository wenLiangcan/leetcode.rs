//Write an efficient algorithm that searches for a value in an m x n matrix.
//This matrix has the following properties:
//
//
// Integers in each row are sorted from left to right.
// The first integer of each row is greater than the last integer of the
//previous row.
//
//
//
// Example 1:
//
//
//Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
//Output: true
//
//
// Example 2:
//
//
//Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
//Output: false
//
//
//
// Constraints:
//
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 100
// -10⁴ <= matrix[i][j], target <= 10⁴
//
// Related Topics Array Binary Search Matrix 👍 5079 👎 228


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3));
    }

    #[test]
    fn test_2() {
        assert!(!search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 13));
    }

    #[test]
    fn test_3() {
        assert!(search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]], 5));
    }

    #[test]
    fn test_4() {
        assert!(search_matrix(vec![vec![1, 3, 5]], 3));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use core::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    'out: for v in matrix {
        match target.cmp(&v[0]) {
            Ordering::Equal => return true,
            Ordering::Less => return false,
            Ordering::Greater => {
                match v.last() {
                    Some(i) => {
                        match target.cmp(i) {
                            Ordering::Equal => return true,
                            Ordering::Greater => continue 'out,
                            Ordering::Less => {
                                for j in 1 .. v.len() - 1 {
                                    match target.cmp(&v[j]) {
                                        Ordering::Equal => return true,
                                        Ordering::Less => continue 'out,
                                        Ordering::Greater => continue,
                                    }
                                }
                            }
                        }
                    },
                    _ => continue,
                }
            }
        }
    }
    return false;
}
//leetcode submit region end(Prohibit modification and deletion)
