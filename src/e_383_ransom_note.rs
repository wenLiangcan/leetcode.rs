//Given two stings ransomNote and magazine, return true if ransomNote can be
//constructed from magazine and false otherwise.
//
// Each letter in magazine can only be used once in ransomNote.
//
//
// Example 1:
// Input: ransomNote = "a", magazine = "b"
//Output: false
// Example 2:
// Input: ransomNote = "aa", magazine = "ab"
//Output: false
// Example 3:
// Input: ransomNote = "aa", magazine = "aab"
//Output: true
//
//
// Constraints:
//
//
// 1 <= ransomNote.length, magazine.length <= 10⁵
// ransomNote and magazine consist of lowercase English letters.
//
// Related Topics Hash Table String Counting 👍 1299 👎 272


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(!can_construct("a".to_owned(), "b".to_owned()));
    }

    #[test]
    fn test_2() {
        assert!(!can_construct("aa".to_owned(), "ab".to_owned()));
    }

    #[test]
    fn test_3() {
        assert!(can_construct("aa".to_owned(), "aab".to_owned()));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() <= magazine.len() {
        ransom_note.len() == count_intersect(ransom_note, magazine)
    } else {
        false
    }
}

fn count_intersect(nums1: String, nums2: String) -> usize {
    let mut map: HashMap<char, usize> = nums1.chars().into_iter()
        .fold(HashMap::new(), |mut m, i| {
            m.entry(i)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            m
        });

    let mut count = 0;
    for i in nums2.chars() {
        match map.entry(i) {
            Entry::Occupied(c) => {
                let c = c.into_mut();
                if *c > 0 {
                    count += 1;
                    *c -= 1;
                }
            },
            _ => ()
        }
    }

    count
}
//leetcode submit region end(Prohibit modification and deletion)
