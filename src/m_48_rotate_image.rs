//You are given an n x n 2D matrix representing an image, rotate the image by 90
// degrees (clockwise).
//
// You have to rotate the image in-place, which means you have to modify the
//input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
//
//
// Example 1:
//
//
//Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//Output: [[7,4,1],[8,5,2],[9,6,3]]
//
//
// Example 2:
//
//
//Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
//Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
//
//
// Example 3:
//
//
//Input: matrix = [[1]]
//Output: [[1]]
//
//
// Example 4:
//
//
//Input: matrix = [[1,2],[3,4]]
//Output: [[3,1],[4,2]]
//
//
//
// Constraints:
//
//
// matrix.length == n
// matrix[i].length == n
// 1 <= n <= 20
// -1000 <= matrix[i][j] <= 1000
//
// Related Topics Array Math Matrix 👍 7185 👎 431


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_n_rank_3() {
        assert_eq!((0, 2), rotate_n(3, 0, 0, 0, 2));
        assert_eq!((1, 2), rotate_n(3, 0, 0, 1, 2));
        assert_eq!((2, 2), rotate_n(3, 0, 0, 2, 2));
        assert_eq!((2, 1), rotate_n(3, 0, 1, 2, 2));
        assert_eq!((2, 0), rotate_n(3, 0, 2, 2, 2));
    }


    #[test]
    fn test_rotate_1() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut m);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], m);
    }

    #[test]
    fn test_rotate_2() {
        let mut m = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];
        rotate(&mut m);
        assert_eq!(vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]], m);
    }

    #[test]
    fn test_rotate_3() {
        let mut m = vec![vec![1]];
        rotate(&mut m);
        assert_eq!(vec![vec![1]], m);
    }

    #[test]
    fn test_rotate_4() {
        let mut m = vec![vec![1, 2], vec![3, 4]];
        rotate(&mut m);
        assert_eq!(vec![vec![3, 1], vec![4, 2]], m);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let rank = matrix.len();

    for level in 0 .. (rank / 2) {
        for _c in level .. (rank - 1 - level) {
            let (mut r, mut c) = (level, _c);
            let n = rank - 1 - level * 2;
            let mut current = matrix[r][c];
            for _ in 0 .. 4 {
                let (mut r_n, mut c_n) = rotate_n(rank, level, r, c, n);
                let tmp = matrix[r_n][c_n];
                matrix[r_n][c_n] = current;
                current = tmp;
                (r, c) = (r_n, c_n);
            }
        }
    }
}

fn rotate_n(rank: usize, level: usize, r: usize, c: usize, n: usize) -> (usize, usize) {
    let lower_bound = level;
    let upper_bound = rank - 1 - level;
    let mut r_n = r;
    let mut c_n = c;
    let mut remain = n;

    while remain > 0 {
        if r_n == lower_bound && c_n < upper_bound {  // →
            c_n += remain;
            if c_n > upper_bound {
                remain = c_n - upper_bound;
                c_n = upper_bound;
            } else {
                remain = 0;
            }
        }

        if c_n == upper_bound && r_n < upper_bound {  // ↓
            r_n += remain;
            if r_n > upper_bound {
                remain = r_n - upper_bound;
                r_n = upper_bound;
            } else {
                remain = 0;
            }
        }

        if r_n == upper_bound && c_n > lower_bound {  // ←
            if lower_bound + remain > c_n {
                remain = lower_bound + remain - c_n;
                c_n = lower_bound;
            } else {
                c_n -= remain;
                remain = 0;
            }
        }

        if c_n == lower_bound && r_n > lower_bound {  // ↑
            if lower_bound + remain > r_n {
                remain = lower_bound + remain - r_n;
                r_n = lower_bound;
            } else {
                r_n -= remain;
                remain = 0;
            }
        }
    }

    (r_n, c_n)
}
//leetcode submit region end(Prohibit modification and deletion)
