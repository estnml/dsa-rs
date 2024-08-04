#[cfg(test)]
mod tests {
    use structures::queue::Queue_dsa;

    #[test]
    fn passes_len() {
        let mut que = Queue_dsa::new();
        assert_eq!(0, que.len());

        que.enqueue(3);
        que.enqueue(8);

        assert_eq!(2, que.len());

        que.dequeue();
        assert_eq!(1, que.len());
    }
}
