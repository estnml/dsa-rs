## FINDINGS
#### pointers and smart pointers
- normal pointers:
    - &: reference. it has a relationship with lifetimes.

- smart pointers:
    - Box: 
        - it is used to allocate data on the heap.
        - The pointer to the data is stored on the STACK.
    - Rc: 
        - a control block is created on the heap.
        - this control block stores reference count and a pointer to the actual data on the heap.
        - in the stack, a pointer is created that points to control block.
        - when the reference count on the control block is reached zero, the value will be de-allocated.
        - When Rc gets cloned, the reference count is incremented on the control block. Actual data is not cloned.