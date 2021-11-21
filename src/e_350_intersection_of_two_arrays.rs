//Given two integer arrays nums1 and nums2, return an array of their
//intersection. Each element in the result must appear as many times as it shows in both
//arrays and you may return the result in any order.
//
//
// Example 1:
//
//
//Input: nums1 = [1,2,2,1], nums2 = [2,2]
//Output: [2,2]
//
//
// Example 2:
//
//
//Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
//Output: [4,9]
//Explanation: [9,4] is also accepted.
//
//
//
// Constraints:
//
//
// 1 <= nums1.length, nums2.length <= 1000
// 0 <= nums1[i], nums2[i] <= 1000
//
//
//
// Follow up:
//
//
// What if the given array is already sorted? How would you optimize your
//algorithm?
// What if nums1's size is small compared to nums2's size? Which algorithm is
//better?
// What if elements of nums2 are stored on disk, and the memory is limited such
//that you cannot load all elements into the memory at once?
//
// Related Topics Array Hash Table Two Pointers Binary Search Sorting 👍 3397 👎
// 600


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    }

    #[test]
    fn test_2() {
        let mut out = intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        out.sort();
        assert_eq!(out, vec![4, 9]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = nums1.into_iter()
        .fold(HashMap::new(), |mut m, i| {
            m.entry(i)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            m
        });

    let mut out: Vec<i32> = vec![];
    for i in nums2 {
       match map.entry(i) {
           Entry::Occupied(c) => {
               let c = c.into_mut();
               if *c > 0 {
                   out.push(i);
                   *c -= 1;
               }
           },
           _ => ()
       }
    }

    out
}
//leetcode submit region end(Prohibit modification and deletion)
