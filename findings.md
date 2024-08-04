## FINDINGS
> will be updated
#### pointers and smart pointers
- normal pointers:
    - &: reference. it has a relationship with lifetimes.
        - what is lifetime

- smart pointers:
    - Box: 
        - it is used to allocate data on the heap.
        - The pointer to the data is stored on the STACK.
    - Rc: 
        - a control block is created on the heap.
        - this control block stores reference count, some other info and a pointer to the actual data on the heap. 
        - in the stack, a pointer is created that points to control block.
        - when the reference count on the control block is reached zero, the value will be de-allocated.
        - When Rc gets cloned, the reference count is incremented on the control block. Actual data is not cloned.
        - **the actual data inside the Rc type is immutable**. But **with Interior Mutability** this behaviour can be changed.



#### interior Mutability
