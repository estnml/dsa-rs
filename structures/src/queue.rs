// TODO: generic: -
// TODO: enqueue - dequeue: -
// TODO: iterators?: -
// TODO: collection related trait impls: -
// TODO: thread safe: -
// TODO: non recursive drop impl?: -
// TODO: queue using linked list (-), array (-)
// TODO: impl Debug: -

pub struct Queue_dsa<T> {
    len: usize,
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

pub struct QueueItem_dsa<T> {
    data: T,
    next: Option<Link<T>>,
}

type Link<T> = std::rc::Rc<std::cell::RefCell<QueueItem_dsa<T>>>;

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
        todo!()
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
