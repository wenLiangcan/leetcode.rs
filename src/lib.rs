#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

#[macro_export]
macro_rules! ll {
    ( $x:expr ) => { Some(Box::new(ListNode::new($x))) };
    ( $head:expr, $( $tail:expr ),+ ) => {
        Some(Box::new(
            ListNode {
                val: $head,
                next: ll!($( $tail ),+)
            }
        ))
    }
}