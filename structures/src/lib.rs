#[allow(non_camel_case_types)]
pub mod linkedlist;

#[cfg(test)]
mod test {
    use crate::linkedlist::dsa_linkedlist;

    #[test]
    fn basic_ops() {
        let mut list = dsa_linkedlist::new();

        list.push_back(24);
        list.push_back(35);
        list.push_back(43);
        list.push_front(31);
        list.push_front(32);

        assert_eq!(list.len(), 5);

        list.push_back(35);
        list.push_back(24);
        list.push_back(24);

        assert_eq!(list.pop(), Some(24));
        list.push_front(32);
        list.push_back(35);
        list.push_back(43);
        assert_eq!(list.pop_front(), Some(32));
        list.push_front(31);
        list.push_front(31);
        list.push_back(43);
        list.push_front(32);

        assert_eq!(list.pop_front(), Some(32));
        assert_eq!(list.pop(), Some(43));

        assert_eq!(list.len(), 11);
    }

    #[test]
    fn iter() {
        let mut list = dsa_linkedlist::new();

        list.push_back(24);
        list.push_back(35);
        list.push_back(43);
        list.push_front(31);
        list.push_front(32);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&32));
        assert_eq!(iter.next(), Some(&31));
        assert_eq!(iter.next(), Some(&24));
        assert_eq!(iter.next(), Some(&35));
        assert_eq!(iter.next(), Some(&43));
        assert_eq!(iter.next(), None);
    }
}
