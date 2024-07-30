#[cfg(test)]
mod tests {
    use structures::stack;

    #[test]
    fn basic_ops() {
        let mut stack = structures::stack::Stack_dsa::new();

        assert!(stack.is_empty());

        stack.push(3);
        stack.push(5);
        stack.push(7);
        stack.push(9);
        stack.push(11);

        // println!("stack item: {:#?}", stack.peek(0));

        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(11));
        assert_eq!(stack.pop(), Some(9));
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(3));

        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());

        stack.push(5);
        stack.push(7);
        stack.push(9);

        assert_eq!(stack.len(), 3);
    }
}
