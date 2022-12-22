# Stack & Heap

#### Stack
The stack is a region of memory that is used to store local variables and function call frames in a program. It is organized in a Last-In, First-Out (LIFO) order, which means that the last value pushed onto the stack is the first one that is popped off.

In Rust, the stack is used to store local variables and function call frames. Local variables are created and destroyed automatically as the program runs, and they are stored in a LIFO order. Function call frames are created when a function is called, and they are destroyed when the function returns.

The stack has a number of useful properties:

- It is fast to allocate and deallocate memory, because it does not require any dynamic memory allocation.
- It has a fixed size, which means that it is not possible to allocate more memory on the stack than it has available.
- It is easy to reason about, because values on the stack have a well-defined lifetime.

However, the stack also has some limitations:

- It has a fixed size, which means that it is not possible to allocate arbitrarily large amounts of memory on the stack.
- Values on the stack cannot outlive the function they were created in, because they are automatically destroyed when the function returns.

#### Heap
The heap is a region of memory that is used to store dynamic data structures, such as vectors, strings, and boxes. It is a general-purpose memory pool that can be used to allocate and deallocate memory at runtime.

In Rust, the heap is used to store values that have a longer lifetime than the function they were created in. These values are allocated using dynamic memory allocation, which involves requesting memory from the operating system at runtime.

The heap has a number of useful properties:

- It allows values to have a longer lifetime than the function they were created in.
- It allows values to be shared between multiple threads.
- It allows the size of a value to be determined at runtime, rather than at compile time.

However, the heap also has some limitations:

- It is slower to allocate and deallocate memory than the stack, because it requires dynamic memory allocation.
- It is more difficult to reason about, because the lifetime of a value on the heap is not always clear.

#### Rust stack vs Thread-local stack
The thread-local stack, on the other hand, is a stack that is specific to a particular thread of execution. Each thread in a program has its own stack, which is used to store local variables and function call frames for that thread.

In Rust, the stack is used to store local variables and function call frames for all threads in a program. Each thread has its own stack, which is used to store local variables and function call frames for that thread.
