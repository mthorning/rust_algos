#![allow(dead_code)]

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: i32,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn enqueue(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.tail.take(),
        });

        self.tail = Some(new_node);
        self.length = self.length + 1;
    }

    fn deque(&mut self) -> Option<T> {
        self.head.take().map(|node| {
                self.head = node.next;
                let value = node.value;
                self.length = self.length - 1;
                value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = Queue::new();

        for number in 1..11 {
            queue.enqueue(number);
            assert_eq!(queue.length, number);
        }


        for number in 10..0 {
            assert_eq!(queue.deque(), Some(number));
            assert_eq!(queue.length, number);
        }

        assert_eq!(queue.deque(), None);
    }
}
