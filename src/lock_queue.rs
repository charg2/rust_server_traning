use std::collections::VecDeque;
use std::sync::Mutex;

pub struct LockQueue<T>
{
    queue: Mutex<VecDeque<T>>,
}

impl<T> LockQueue<T>
{
    pub fn new() -> Self
    {
        LockQueue
        {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn push(&self, item: T)
    {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
    }

    pub fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front().unwrap()
    }

    pub fn try_pop(&self) -> Option<T>
    {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_queue() {
        let queue = LockQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
    }

    #[test]
    fn test_try_pop() {
        let queue = LockQueue::new();
        assert_eq!(queue.try_pop(), None);
        queue.push(1);
        assert_eq!(queue.try_pop(), Some(1));
        assert_eq!(queue.try_pop(), None);
    }
}