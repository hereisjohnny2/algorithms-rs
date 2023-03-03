pub mod stack;
pub mod sort;

#[cfg(test)]
mod tests {
    use crate::{stack::{Node, Stack}, sort::bubble_sort};

    use super::*;

    #[test]
    fn should_sort_with_bubble_sort() {
        let mut arr = [1, 3, 7, 4, 2];

        let result = bubble_sort(&mut arr);
        assert_eq!(result, [1, 2, 3, 4, 7]);
    }

    #[test]
    fn should_create_a_node() {
        let node_a = Node {
            data: 10,
            next: None,
        };
        let node_b = Node {
            data: 20,
            next: Some(Box::new(node_a.clone())),
        };

        assert_eq!(10, node_a.data);
        assert_eq!(None, node_a.next);

        assert_eq!(20, node_b.data);
        assert_eq!(10, node_b.next.unwrap().data);
    }

    #[test]
    fn should_create_a_stack() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(None, stack.clone().peek());

        stack.push(10);
        assert_eq!(Some(10), stack.clone().peek());

        stack.push(20);
        assert_eq!(Some(20), stack.clone().peek());

        stack.pop();
        assert_eq!(Some(10), stack.clone().peek());

        stack.pop();
        assert_eq!(None, stack.clone().peek());
    }
}
