//Given an array of intervals where intervals[i] = [starti, endi], merge all
//overlapping intervals, and return an array of the non-overlapping intervals that
//cover all the intervals in the input.
//
//
// Example 1:
//
//
//Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
//Output: [[1,6],[8,10],[15,18]]
//Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
//
//
// Example 2:
//
//
//Input: intervals = [[1,4],[4,5]]
//Output: [[1,5]]
//Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//
//
//
// Constraints:
//
//
// 1 <= intervals.length <= 10⁴
// intervals[i].length == 2
// 0 <= starti <= endi <= 10⁴
//
// Related Topics Array Sorting 👍 10577 👎 455

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| {
        let a_l = a[0];
        let a_diff = a[1] - a[0];
        let b_l = b[0];
        let b_diff = b[1] - b[0];

        a_l.cmp(&b_l).then(b_diff.cmp(&a_diff))
    });
    intervals.into_iter().fold(vec![], |mut acc, v| {
        let last = acc.last();
        if let Some(l) = last {
            if l[1] >= v[0] {
                let h = if l[1] > v[1] { l[1] } else { v[1] };
                *acc.last_mut().unwrap() = vec![l[0], h];
            } else {
                acc.push(v);
            }
        } else {
            acc.push(v);
        }
        acc
    })
}
//leetcode submit region end(Prohibit modification and deletion)
