// TODO: generic: +
// TODO: push - pop: +
// TODO: iterators?: -
// TODO: collection related trait impls: -
// TODO: thread safe: -
// TODO: non recursive drop impl?: -
// TODO: stack using linked list (-), array (-)

#[derive(Debug)]
pub struct Stack_dsa<T> {
    len: usize,
    top: Option<Link<T>>,
}

#[derive(Debug)]
pub struct StackItem_dsa<T> {
    val: T,
    next: Option<Link<T>>,
}

type Link<T> = Box<StackItem_dsa<T>>;

impl<T> Stack_dsa<T> {
    pub fn new() -> Self {
        Self { len: 0, top: None }
    }

    pub fn push(&mut self, val: T) {
        // stack LIFO yapısında olduğu için, son eklenen eleman ilk elemanmış gibi davranacak.
        // dolayısıyla yeni elemanın next'i, ondan önce eklenmiş olan eleman olmalı. Yani listenin top'u.
        let new_item = Box::new(StackItem_dsa {
            next: self.top.take(),
            val,
        });
        self.top = Some(new_item);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|top_item| {
            self.top = top_item.next;
            self.len -= 1;
            top_item.val
        })
    }

    pub fn peek(&self, index: usize) -> Option<&StackItem_dsa<T>> {
        if index > self.len - 1 {
            return None;
        }

        match self.len {
            0 => None,
            _ => {
                let mut current_item_ref = self.top.as_deref();
                let mut current_item_index = 0;

                while let Some(item) = current_item_ref {
                    if current_item_index == index {
                        return current_item_ref;
                    }

                    current_item_ref = current_item_ref;
                    current_item_index += 1;
                }

                None
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
