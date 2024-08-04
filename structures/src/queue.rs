// TODO: make struct generic:
// TODO: basic ops (enqueue - dequeue):
// TODO: implement common and appropriate collection traits:
// TODO: thread safe:
// TODO: debug impl:
// TODO: all of the above using linked list:
// TODO: optimized drop impl for linked list implementation:
// TODO: all of the above using array:

use std::{cell::RefCell, rc::Rc};

pub struct Queue_dsa<T> {
    len: usize,
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

pub struct QueueItem_dsa<T> {
    data: T,
    next: Option<Link<T>>,
}

type Link<T> = Rc<RefCell<QueueItem_dsa<T>>>;

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
        let new_item = Rc::new(RefCell::new(QueueItem_dsa { data, next: None }));

        match self.tail.take() {
            Some(old_tail) => {
                // queue has at leasdt 1 item
                // old_tail's next will be new_item
                // new tail will point to the new_item

                // clone calls on the Rc is only incrementing the strong count.
                // actual data is not cloned, so it is performant (for single thread though)
                old_tail.borrow_mut().next = Some(new_item.clone());
                self.tail = Some(new_item.clone());
            }
            None => {
                // the queue is empty. head & tail points to the same item.
                self.head = Some(new_item.clone());
                self.tail = Some(new_item.clone());
            }
        };

        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        // remove item from the head side, return the data inside of it.

        match self.head.take() {
            Some(old_head) => {
                // queue has at least 1 item. and the current head has reference count 1 (head pointer)
                // making head point current head'snext will de-allocate the current head item

                self.head = old_head.borrow().next.clone();
                self.len -= 1;
                // TODO: Fix error
                // Some(old_head.borrow().data)
                None
            }
            None => None,
        }
    }

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
