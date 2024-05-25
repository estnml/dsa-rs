#![warn(missing_debug_implementations)]

// roadmap:
// TODO: 1) implement base Linked List for i32 type. (+)
// TODO: 2) implement basic CRUD (-)
// TODO: 3) make implementation generic. (-)
// TODO: 4)? adapt algorithm for usage with traits (-)

#[derive(Debug)]
struct LinkedListNodeDsaRs {
    data: i32,
}

#[derive(Debug)]
pub struct LinkedListDsaRs<'a> {
    head: &'a Option<LinkedListNodeDsaRs>,
    tail: &'a Option<LinkedListNodeDsaRs>,
    capacity: i32,
    len: i32,
    items: [LinkedListNodeDsaRs; 0],
}

impl<'a> LinkedListDsaRs<'a> {
    pub fn new() -> LinkedListDsaRs<'a> {
        LinkedListDsaRs {
            capacity: 0,
            head: &None,
            tail: &None,
            len: 0,
            items: []
        }
    }

    pub fn add_item(&mut self, new_item:i32) {
        
    }
}
