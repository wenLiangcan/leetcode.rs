//Given a positive integer n, generate an n x n matrix filled with elements
//from 1 to n² in spiral order.
//
//
// Example 1:
//
//
//Input: n = 3
//Output: [[1,2,3],[8,9,4],[7,6,5]]
//
//
// Example 2:
//
//
//Input: n = 1
//Output: [[1]]
//
//
//
// Constraints:
//
//
// 1 <= n <= 20
//
// Related Topics Array Matrix Simulation 👍 3597 👎 181

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let m = generate_matrix(3);
        assert_eq!(vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]], m);
    }

    #[test]
    fn test_2() {
        let m = generate_matrix(1);
        assert_eq!(vec![vec![1]], m);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut matrix = vec![vec![-1; n]; n];
    let mut start = (0, 0);
    let (mut r, mut c) = start;
    let mut lower_bound = 0;
    let mut upper_bound = n - 1;
    let mut i = 1;
    loop {
        if matrix[r][c] != -1 {
            break
        }
        matrix[r][c] = i;
        if lower_bound == upper_bound {
            break
        }
        let next = rotate_n(lower_bound, upper_bound, r, c, 1);
        if next == start {
            lower_bound += 1;
            upper_bound -= 1;
            c += 1;
            start = (r, c);
        } else {
            (r, c) = next;
        }
        i += 1;
    }

    matrix
}

fn rotate_n(lower_bound: usize, upper_bound: usize, r: usize, c: usize, n: usize) -> (usize, usize) {
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
