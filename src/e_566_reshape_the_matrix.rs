//In MATLAB, there is a handy function called reshape which can reshape an m x
//n matrix into a new one with a different size r x c keeping its original data.
//
// You are given an m x n matrix mat and two integers r and c representing the
//number of rows and the number of columns of the wanted reshaped matrix.
//
// The reshaped matrix should be filled with all the elements of the original
//matrix in the same row-traversing order as they were.
//
// If the reshape operation with given parameters is possible and legal, output
//the new reshaped matrix; Otherwise, output the original matrix.
//
//
// Example 1:
//
//
//Input: mat = [[1,2],[3,4]], r = 1, c = 4
//Output: [[1,2,3,4]]
//
//
// Example 2:
//
//
//Input: mat = [[1,2],[3,4]], r = 2, c = 4
//Output: [[1,2],[3,4]]
//
//
//
// Constraints:
//
//
// m == mat.length
// n == mat[i].length
// 1 <= m, n <= 100
// -1000 <= mat[i][j] <= 1000
// 1 <= r, c <= 300
//
// Related Topics Array Matrix Simulation 👍 1616 👎 179


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4), vec![vec![1, 2, 3, 4]]);
    }

    #[test]
    fn test_not_fit() {
        assert_eq!(matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4), vec![vec![1, 2], vec![3, 4]]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let size = if mat.is_empty() {0} else { mat.len() * mat[0].len() };
    if size as i32 == r * c {
        mat.into_iter()
            .flatten()
            .collect::<Vec<i32>>()
            .chunks(c as usize)
            .map(|i| {i.to_vec()})
            .collect()
    } else {
        mat
    }
}
//leetcode submit region end(Prohibit modification and deletion)
