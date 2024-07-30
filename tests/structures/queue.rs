#[cfg(test)]
mod tests {

    #[test]
    fn basic_ops() {
        let mut queue: structures::queue::Queue_dsa<i32> = structures::queue::Queue_dsa::new();

        assert_eq!(queue.len(), 0);

        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        queue.enqueue(6);

        assert_eq!(queue.len(), 4);
        println!("{:#?}", queue);
    }
}
