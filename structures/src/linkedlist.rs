pub struct dsa_linkedlist<T> {
    head: Option<Box<dsa_linkedlist_node<T>>>,
    len: usize,
}

struct dsa_linkedlist_node<T> {
    data: T,
    next: Option<Box<dsa_linkedlist_node<T>>>,
}

impl<T> dsa_linkedlist<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = dsa_linkedlist_node { data, next: None };

        match self.get_head_mut() {
            Some(head_node) => {
                let mut current_node = head_node;
                while let Some(ref mut next_node) = current_node.next {
                    current_node = next_node;
                }
                current_node.next = Some(Box::new(new_node));
            }
            None => {
                self.head = Some(Box::new(new_node));
            }
        };

        self.len += 1;
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = dsa_linkedlist_node {
            data,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else if self.len == 1 {
            self.len -= 1;
            self.head.take().map(|node| node.data)
        } else {
            let mut current_node = &mut self.head;

            while let Some(ref mut node) = current_node {
                // if current node's next has some node and that node's next is None, it means it is the last node
                //
                // start from head node, take a reference to head's next node
                // and check for head.next.next is None.
                // if it is None, take() head.next (node.next.take())
                if node.next.as_ref().map_or(false, |next| next.next.is_none()) {
                    self.len -= 1;
                    return node.next.take().map(|node| node.data);
                }
                current_node = &mut node.next;
            }
            None
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, None) {
            Some(old_head) => {
                self.len -= 1;
                self.head = old_head.next;
                Some(old_head.data)
            }
            None => None,
        }
    }

    pub fn get_head_mut(&mut self) -> &mut Option<Box<dsa_linkedlist_node<T>>> {
        &mut self.head
    }

    pub fn get_head(&self) -> &Option<Box<dsa_linkedlist_node<T>>> {
        &self.head
    }
}

pub struct dsa_linkedlist_iter<'a, T> {
    next: Option<&'a dsa_linkedlist_node<T>>,
}

impl<T> dsa_linkedlist<T> {
    pub fn iter(&self) -> dsa_linkedlist_iter<T> {
        dsa_linkedlist_iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for dsa_linkedlist<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        write!(f, "head -> ")?;
        while let Some(ref node) = current {
            write!(f, "{:?} -> ", node.data)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}

impl<'a, T> std::iter::Iterator for dsa_linkedlist_iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}
