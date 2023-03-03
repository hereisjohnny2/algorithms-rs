use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T: Copy> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stack<T: Copy> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> Stack<T>
where
    T: Copy + Debug,
{
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, data: T) {
        let head = self.head.clone();
        match head {
            None => {
                let new_node = Box::new(Node { data, next: None });
                self.head = Some(new_node)
            }
            Some(n) => {
                let new_node = Box::new(Node { data, next: Some(n) });
                self.head = Some(new_node);
            }
        }
    }

    pub fn peek(self) -> Option<T> {
        match self.head {
            None => None,
            Some(n) => Some(n.data),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.clone();
        match head {
            None => None,
            Some(n) => {
                self.head = n.next;
                Some(n.data)
            }
        }
    }
}
