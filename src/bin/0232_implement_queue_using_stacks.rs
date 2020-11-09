struct MyQueue {
    elements: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self { elements: vec![] }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.elements.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.elements.remove(0)
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        self.elements[0]
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue() {
        let mut my_queue = MyQueue::new();
        my_queue.push(1);
        assert_eq!(my_queue.elements, vec![1]);
        my_queue.push(2);
        assert_eq!(my_queue.elements, vec![1, 2]);
        assert_eq!(my_queue.peek(), 1);
        assert_eq!(my_queue.elements, vec![1, 2]);
        assert_eq!(my_queue.pop(), 1);
        assert_eq!(my_queue.elements, vec![2]);
        assert_eq!(my_queue.empty(), false);
    }
}
