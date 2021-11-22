//Given a string s, find the first non-repeating character in it and return its
//index. If it does not exist, return -1.
//
//
// Example 1:
// Input: s = "leetcode"
//Output: 0
// Example 2:
// Input: s = "loveleetcode"
//Output: 2
// Example 3:
// Input: s = "aabb"
//Output: -1
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s consists of only lowercase English letters.
//
// Related Topics Hash Table String Queue Counting 👍 3952 👎 171


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first_uniq_char("leetcode".to_owned()), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(first_uniq_char("loveleetcode".to_owned()), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(first_uniq_char("aabb".to_owned()), -1);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut map: HashMap<char, (bool, usize)> = HashMap::new();
    for (i, c) in s.chars().into_iter().enumerate() {
        map.entry(c)
            .and_modify(|e| {
                let (ref mut uniqe, _) = e;
                if *uniqe {
                    *uniqe = false;
                }
            }).or_insert_with(|| {(true, i)});
    }
    map.into_iter()
        .filter(|(_, (uq, _))| {*uq})
        .map(|(_, (_, i))| {i as i32})
        .min()
        .unwrap_or(-1)
}
//leetcode submit region end(Prohibit modification and deletion)
