#![allow(dead_code)]

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_works() {
        let values = vec!["one", "two", "three", "four", "five"];
        let mut stack = Stack::new();

        for value in values.iter() {
            stack.push(value);
        }

        for value in values.iter().rev() {
            assert_eq!(Some(value), stack.pop());
        }

        assert_eq!(None, stack.pop());
    }
}
