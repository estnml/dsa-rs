#[derive(Debug)]
pub struct LinkedList_dsa<T> {
    head: Option<Link<T>>,
    len: usize,
}

#[derive(Debug)]
pub struct LinkedListNode_dsa<T> {
    data: T,
    next: Option<Link<T>>,
}

type Link<T> = Box<LinkedListNode_dsa<T>>;

impl<'a, T> LinkedList_dsa<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(LinkedListNode_dsa {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;
    }
    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(LinkedListNode_dsa { data, next: None });

        match self.head {
            Some(ref mut first_node) => {
                let mut current = first_node;
                // move pointer to the last node
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }

                // here, the current reference is on the last node
                current.next = Some(new_node);
            }
            None => self.head = Some(new_node),
        };

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else if self.len == 1 {
            let head_node = self.head.take();
            self.len -= 1;
            head_node.map(|n| n.data)
        } else {
            // move pointer to the last - 1 th node
            let mut current = self.head.as_mut().unwrap();
            while current.next.as_mut().unwrap().next.is_some() {
                current = current.next.as_mut().unwrap();
            }

            // on the last - 1 th node here
            current.next.take().map(|node| {
                self.len -= 1;
                node.data
            })
        }
    }

    pub fn remove_node(&mut self, index: usize) -> Option<T> {
        if index > self.len - 1 {
            return None;
        }

        if self.len > 0 {
            // list has element(s)
            if index == 0 {
                self.pop_front()
            } else if index == self.len - 1 {
                self.pop_back()
            } else {
                let mut current_node_index = 0;
                let mut current_node = self.head.as_mut().unwrap();
                while current_node.next.is_some() && current_node_index < index - 1 {
                    current_node = current_node.next.as_mut().unwrap();
                    current_node_index += 1;
                }

                let node_to_remove = current_node.next.take();
                node_to_remove.map(|node| {
                    current_node.next = node.next;
                    self.len -= 1;
                    node.data
                })
            }
        } else {
            None
        }
    }

    pub fn get_node(&self, index: usize) -> Option<&LinkedListNode_dsa<T>> {
        if index > self.len - 1 {
            return None;
        }

        if self.len == 0 {
            None
        } else {
            let mut current_index = 0;
            let mut current_node_ref = self.head.as_ref().unwrap();

            while current_index < index && current_node_ref.next.is_some() {
                current_node_ref = current_node_ref.next.as_ref().unwrap();
                current_index += 1;
            }

            Some(&current_node_ref.as_ref())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}




