// TODO: make struct generic:
// TODO: basic ops (enqueue - dequeue):
// TODO: implement common and appropriate collection traits:
// TODO: thread safe:
// TODO: multithread usable:
// TODO: debug impl:
// TODO: all of the above using linked list:
// TODO: optimized drop impl for linked list implementation:
// TODO: all of the above using array:

use std::rc::Rc;

pub struct Queue_dsa<T> {
    len: usize,
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

pub struct QueueItem_dsa<T> {
    data: T,
    next: Option<Link<T>>,
}

type Link<T> = Rc<QueueItem_dsa<T>>;

impl<T> Queue_dsa<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn head(&self) -> Option<&QueueItem_dsa<T>> {
        todo!()
    }
    pub fn tail(&self) -> Option<&QueueItem_dsa<T>> {
        todo!()
    }

    pub fn enqueue(&mut self, data: T) {
        // queue has no elements: self and tail points same item, len++ -> self.tail.take()
        // queue has one element: tail points old head's next, len++ -> self.tail.take()
        // queue has at least one element: tail points old tail's next, len++ -> self.tail.take()

        let new_item = Rc::new(QueueItem_dsa { data, next: None });

        match self.len {
            0 => {
                self.head = Some(new_item.clone());
                self.tail = Some(new_item.clone());
            }
            _ => {
                self.tail.take().map(|old_tail| {
                    old_tail.next = Some(new_item.clone());
                });

                self.tail = Some(new_item.clone());
            }
        };

        self.len += 1;
    }
    pub fn dequeue(&mut self, data: T) {}

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Queue_dsa<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Queue_dsa")
            .field("len", &self.len)
            .field("head", &self.head)
            .field("tail", &self.tail)
            .finish()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for QueueItem_dsa<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueueItem_dsa")
            .field("data", &self.data)
            .field("next", &self.next)
            .finish()
    }
}
