#[cfg(test)]
mod basics {
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

        // assert_eq!(llist.get_node_at(7), Some(&77));
        println!("node: {:#?}", llist.get_node(0));
        // assert_eq!(llist.get_node_at(9), Some(&9));
        // assert_eq!(llist.get_node_at(0), Some(&11123));

        assert_eq!(llist.len(), 5);

        assert_eq!(llist.pop_back(), Some(25));
        assert_eq!(llist.pop_front(), Some(3));

        // assert_eq!(llist.get_node(0), Some(&3));
        // assert_eq!(llist.get_node(2), Some(&4));

        assert_eq!(llist.len(), 3);
        // println!("{:#?}", llist);
    }
}
