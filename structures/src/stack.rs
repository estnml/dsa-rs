// TODO: generic: +
// TODO: push - pop - peek: +
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
    data: T,
    next: Option<Link<T>>,
}

type Link<T> = Box<StackItem_dsa<T>>;

impl<T> Stack_dsa<T> {
    pub fn new() -> Self {
        Self { len: 0, top: None }
    }

    pub fn push(&mut self, data: T) {
        // stack LIFO yapısında olduğu için, son eklenen eleman ilk elemanmış gibi davranacak.
        // dolayısıyla yeni elemanın next'i, ondan önce eklenmiş olan eleman olmalı. Yani listenin top'u.
        let new_item = Box::new(StackItem_dsa {
            next: self.top.take(),
            data,
        });
        self.top = Some(new_item);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|top_item| {
            self.top = top_item.next;
            self.len -= 1;
            top_item.data
        })
    }

    pub fn top(&self) -> Option<&StackItem_dsa<T>> {
        self.top.as_deref()
    }

    pub fn peek(&self, index: usize) -> Option<&StackItem_dsa<T>> {
        if index > self.len - 1 {
            return None;
        }

        match self.len {
            0 => None,
            _ => {
                // index, listeye eklenen ilk elemandan başlıyor.
                // yani en büyük index son eklenen elemanda oluyor.
                let mut current_item_index = self.len - 1;
                let mut current_item_ref = self.top.as_deref();

                while let Some(item) = current_item_ref {
                    if current_item_index == index {
                        return current_item_ref;
                    }

                    current_item_ref = item.next.as_deref();
                    current_item_index -= 1;
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
