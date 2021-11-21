//Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be
//validated according to the following rules:
//
//
// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9
//without repetition.
//
//
// Note:
//
//
// A Sudoku board (partially filled) could be valid but is not necessarily
//solvable.
// Only the filled cells need to be validated according to the mentioned rules.
//
//
//
//
// Example 1:
//
//
//Input: board =
//[["5","3",".",".","7",".",".",".","."]
//,["6",".",".","1","9","5",".",".","."]
//,[".","9","8",".",".",".",".","6","."]
//,["8",".",".",".","6",".",".",".","3"]
//,["4",".",".","8",".","3",".",".","1"]
//,["7",".",".",".","2",".",".",".","6"]
//,[".","6",".",".",".",".","2","8","."]
//,[".",".",".","4","1","9",".",".","5"]
//,[".",".",".",".","8",".",".","7","9"]]
//Output: true
//
//
// Example 2:
//
//
//Input: board =
//[["8","3",".",".","7",".",".",".","."]
//,["6",".",".","1","9","5",".",".","."]
//,[".","9","8",".",".",".",".","6","."]
//,["8",".",".",".","6",".",".",".","3"]
//,["4",".",".","8",".","3",".",".","1"]
//,["7",".",".",".","2",".",".",".","6"]
//,[".","6",".",".",".",".","2","8","."]
//,[".",".",".","4","1","9",".",".","5"]
//,[".",".",".",".","8",".",".","7","9"]]
//Output: false
//Explanation: Same as Example 1, except with the 5 in the top left corner
//being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is
//invalid.
//
//
//
// Constraints:
//
//
// board.length == 9
// board[i].length == 9
// board[i][j] is a digit 1-9 or '.'.
//
// Related Topics Array Hash Table Matrix 👍 3870 👎 642

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let board = vec![
            vec!['5','3','.','.','7','.','.','.','.']
            ,vec!['6','.','.','1','9','5','.','.','.']
            ,vec!['.','9','8','.','.','.','.','6','.']
            ,vec!['8','.','.','.','6','.','.','.','3']
            ,vec!['4','.','.','8','.','3','.','.','1']
            ,vec!['7','.','.','.','2','.','.','.','6']
            ,vec!['.','6','.','.','.','.','2','8','.']
            ,vec!['.','.','.','4','1','9','.','.','5']
            ,vec!['.','.','.','.','8','.','.','7','9']];
        assert!(is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid() {
        let board = vec![
            vec!['8','3','.','.','7','.','.','.','.']
            ,vec!['6','.','.','1','9','5','.','.','.']
            ,vec!['.','9','8','.','.','.','.','6','.']
            ,vec!['8','.','.','.','6','.','.','.','3']
            ,vec!['4','.','.','8','.','3','.','.','1']
            ,vec!['7','.','.','.','2','.','.','.','6']
            ,vec!['.','6','.','.','.','.','2','8','.']
            ,vec!['.','.','.','4','1','9','.','.','5']
            ,vec!['.','.','.','.','8','.','.','7','9']];
        assert!(!is_valid_sudoku(board));
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut is_valid = board.iter()
        .all(|r| {
            let s: Vec<_> = r.iter().filter(|i| {**i != '.'}).collect();
            s.len() == s.iter().collect::<HashSet<_>>().len()
        });

    is_valid = is_valid && {
        let mut columns_are_valid = true;
        for c in 0 .. 9 {
            let mut s = HashSet::new();
            let mut count = 0usize;
            for r in 0 .. 9 {
                let n = board[r][c];
                if n != '.' {
                    count += 1;
                    s.insert(n);
                }
            }
            columns_are_valid = columns_are_valid && s.len() == count;
            if !columns_are_valid {
                break;
            }
        }
        columns_are_valid
    };

    is_valid && {
        let mut subboxes_are_valid = true;
        'out: for b_c in 0 .. 3 {
            for b_r in 0 .. 3 {
                let mut s = HashSet::new();
                let mut count = 0usize;
                let c_offset = b_c * 3;
                let r_offset = b_r * 3;
                for c in 0 .. 3 {
                    let c = c + c_offset;
                    for r in 0 .. 3 {
                        let n = board[r + r_offset][c];
                        if n != '.' {
                            count += 1;
                            s.insert(n);
                        }
                    }
                }
                subboxes_are_valid = subboxes_are_valid && count == s.len();
                if !subboxes_are_valid {
                    break 'out;
                }
            }
        }
        subboxes_are_valid
    }
}
//leetcode submit region end(Prohibit modification and deletion)
