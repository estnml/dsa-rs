#[cfg(test)]
mod linkedlist_tests {
    use structures::linkedlist::LinkedList_dsa;

    #[test]
    fn basic_ops() {
        let mut llist = LinkedList_dsa::new();

        llist.push_back(4);
        llist.push_back(13);
        llist.push_front(16);
        llist.push_front(3);
        llist.push_back(25);
        llist.push_back(8);
        llist.pop_back();


        // println!("node: {:#?}", llist.get_node(3));

        assert_eq!(llist.len(), 5);

        assert_eq!(llist.pop_back(), Some(25));
        assert_eq!(llist.pop_front(), Some(3));

        assert_eq!(llist.len(), 3);
        // println!("{:#?}", llist);
    }
}
