//Write an efficient algorithm that searches for a value target in an m x n
//integer matrix matrix. This matrix has the following properties:
//
//
// Integers in each row are sorted in ascending from left to right.
// Integers in each column are sorted in ascending from top to bottom.
//
//
//
// Example 1:
//
//
//Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[1
//8,21,23,26,30]], target = 5
//Output: true
//
//
// Example 2:
//
//
//Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[1
//8,21,23,26,30]], target = 20
//Output: false
//
//
//
// Constraints:
//
//
// m == matrix.length
// n == matrix[i].length
// 1 <= n, m <= 300
// -10⁹ <= matrix[i][j] <= 10⁹
// All the integers in each row are sorted in ascending order.
// All the integers in each column are sorted in ascending order.
// -10⁹ <= target <= 10⁹
//
// Related Topics Array Binary Search Divide and Conquer Matrix 👍 7190 👎 122

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_vec() {
        let r = search_vec(&vec![1, 4, 7, 11, 15], 0, 4, 5);
        assert_eq!((1, Ordering::Greater), r);
        let r = search_vec(&vec![2, 5, 8, 12, 19], 0, 1, 5);
        assert_eq!((1, Ordering::Equal), r);
        let r = search_vec(&vec![1, 4, 7, 11, 15], 0, 4, 20);
        assert_eq!((4, Ordering::Greater), r);
    }

    #[test]
    fn test_1() {
        let m = vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]];
        assert!(search_matrix(m, 5));
    }

    #[test]
    fn test_2() {
        let m = vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]];
        assert!(!search_matrix(m, 20));
    }

    #[test]
    fn test_3() {
        let m = vec![vec![1, 4], vec![2, 5]];
        assert!(search_matrix(m, 2));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use core::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut start = 0;
    let mut end = matrix[0].len() - 1;

    for r in matrix {
        let (i, order) = search_vec(&r, start, end, target);
        match order {
            Ordering::Equal => return true,
            Ordering::Less => {
                if i > 0 {
                    end = i - 1;
                } else {
                    return false;
                }
            },
            _ => end = i,
        }
    }

    false
}

fn search_vec(v: &Vec<i32>, mut start: usize, mut end: usize, target: i32) -> (usize, Ordering) {
    loop {
        let index = (end - start) / 2 + start;

        let order = target.cmp(&v[index]);
        match order {
            Ordering::Equal =>
                return (index, Ordering::Equal),
            Ordering::Less => {
                if index == 0 {
                    return (index, order)
                }
                end = index - 1;
            },
            Ordering::Greater =>
                start = index + 1,
        }

        if start > end {
            return (index, order);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
