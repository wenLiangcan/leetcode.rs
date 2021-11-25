//Implement a first in first out (FIFO) queue using only two stacks. The
//implemented queue should support all the functions of a normal queue (push, peek, pop,
//and empty).
//
// Implement the MyQueue class:
//
//
// void push(int x) Pushes element x to the back of the queue.
// int pop() Removes the element from the front of the queue and returns it.
// int peek() Returns the element at the front of the queue.
// boolean empty() Returns true if the queue is empty, false otherwise.
//
//
// Notes:
//
//
// You must use only standard operations of a stack, which means only push to
//top, peek/pop from top, size, and is empty operations are valid.
// Depending on your language, the stack may not be supported natively. You may
//simulate a stack using a list or deque (double-ended queue) as long as you use
//only a stack's standard operations.
//
//
//
// Example 1:
//
//
//Input
//["MyQueue", "push", "push", "peek", "pop", "empty"]
//[[], [1], [2], [], [], []]
//Output
//[null, null, null, 1, 1, false]
//
//Explanation
//MyQueue myQueue = new MyQueue();
//myQueue.push(1); // queue is: [1]
//myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
//myQueue.peek(); // return 1
//myQueue.pop(); // return 1, queue is [2]
//myQueue.empty(); // return false
//
//
//
// Constraints:
//
//
// 1 <= x <= 9
// At most 100 calls will be made to push, pop, peek, and empty.
// All the calls to pop and peek are valid.
//
//
//
// Follow-up: Can you implement the queue such that each operation is amortized
//O(1) time complexity? In other words, performing n operations will take overall
//O(n) time even if one of those operations may take longer.
// Related Topics Stack Design Queue 👍 2541 👎 188


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert!(queue.empty());
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
struct MyQueue {
    asc: Vec<i32>,
    desc: Vec<i32>
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 *
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue {
            asc: vec![],
            desc: vec![]
        }
    }

    fn push(&mut self, x: i32) {
        self.desc.push(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(i) = self.asc.pop() {
            i
        } else {
            while let Some(i) = self.desc.pop() {
                self.asc.push(i);
            }
            self.asc.pop().unwrap()
        }
    }

    fn peek(&mut self) -> i32 {
        if let Some(i) = self.asc.last() {
            *i
        } else {
            while let Some(i) = self.desc.pop() {
                self.asc.push(i);
            }
            *self.asc.last().unwrap()
        }
    }

    fn empty(&self) -> bool {
        self.asc.is_empty() && self.desc.is_empty()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
