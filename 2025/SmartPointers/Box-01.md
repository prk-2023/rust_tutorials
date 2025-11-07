# Box<T> : Box Types

(This article will explore how to use the `Box<T>` type in Rust to manage data on the heap. With Ex to
demonstrate how `Box<T>` can be used in system-level programming, particularly where heap allocation is 
needed for dynamically sized data structures.)

### 1.0 What is `Box<T>`?

In Rust, `Box<T>` is a smart pointer that provides ownership of data allocated on the heap. 

*Boxes* allow you to store data on Heap rather than the stack. What remains on the stack is the pointer to
the heap data. 

Boxes do not have performance overhead, other then storing their data on the heap instead of on the stack.

It is commonly used when:

* When you have a type whose size can't be known at compile time and you want to use a value of that type in
  context that requires extra size.

* You need to store data that is too large for the stack (e.g., large arrays or complex data structures).

* When You need to transfer ownership of data between different parts of your program without copying it.

* When you want to own a value and you care only that it's a type that implements a particular trait rather
  than being of a special type. 

* You want to ensure that memory is deallocated when it is no longer needed.

Unlike variables stored on the stack, data stored in a `Box<T>` lives on the heap, and the memory is 
automatically freed when the `Box<T>` goes out of scope.

### 2.0 Basic Syntax of `Box<T>`

```rust
// define a variable `b` to have a value of `Box` that points to the value 5 which is allocated on the heap.
let b = Box::new(5);
```
Here, `Box::new(5)` allocates memory on the heap for the value `5` and returns a `Box` pointing to it.

Boxes allow you to define types that would not be allowed to define if you didn't have boxes.

Note:
---
`Box::new(5)` : which creates a smart pointer on the stack that owns and manages a value on the heap 
is achieved in `C` using a **raw pointer and dynamic memory allocation functions like `malloc()`**.

Since C doesn't have a built-in smart pointer like `Box`, you have to manually handle the allocation 
and deallocation.


#### 2.1 Example Scenario: Using `Box<T>` in System Programming

In system programming, we often need to manage data that has a variable size or needs to be dynamically 
allocated. 

For example, consider managing a linked list where the data nodes might be dynamically allocated.

We'll create a simple linked list structure using `Box<T>` to store nodes on the heap.

### Example: Creating a Linked List Using `Box<T>`

The example below  creates a simple linked list where each node contains a `u32` value and a reference to 
the next node. The linked list will be allocated on the heap, and each `Box` will hold a pointer to the next
node.

```rust
// Defining a Linked List Node using Box
struct ListNode {
    value: u32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    // Constructor to create a new ListNode
    fn new(value: u32) -> ListNode {
        ListNode {
            value,
            next: None,
        }
    }

    // Add a new node to the list
    fn add_next(&mut self, node: ListNode) {
        let current = self;

        // Traverse until 'current.next' is None
        while let Some(ref next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(node));
    }

    // Print the list starting from the current node
    fn print(&self) {
        let mut current = self;
        while let Some(ref next_node) = current.next {
            println!("Value: {}", current.value);
            current = next_node;
        }
        println!("Value: {}", current.value);  // print the last node
    }
}

fn main() {
    // Create the first node
    let mut head = ListNode::new(1);
    
    // Add subsequent nodes
    head.add_next(ListNode::new(2));
    head.add_next(ListNode::new(3));
    head.add_next(ListNode::new(4));
    head.add_next(ListNode::new(5));
    
    // Print the list
    head.print();
}
```

### Breakdown of the Code:

1. **Struct Definition:**

   * `ListNode` struct has two fields:

     * `value` is a `u32` to store the data.
     * `next` is an `Option<Box<ListNode>>` that points to the next node in the list, allowing us to build a
       chain of nodes.

2. **Methods:**

   * `new(value: u32)` creates a new node with the specified value and sets `next` to `None` (since it
     doesn't point to any other node initially).

   * `add_next(node: ListNode)` move till the last node then accepts a new node, wraps it in a `Box`, and
     store it as the `next` node in the current `ListNode`.

   * `print()` iterates through the list, printing each node's value until it reaches the end of the list.

3. **Memory Management with `Box<T>`:**

   * Each node is allocated on the heap. The `Box<T>` ensures that memory is automatically freed when the 
     node is no longer used, which is especially useful in system programming where managing resources 
     carefully is critical.

### Understanding the Heap Allocation

In this code, the use of `Box<T>` ensures that each node is allocated on the heap. 
When we create a new node with `Box::new()`, Rust allocates space on the heap for the `ListNode` and returns 
a `Box` that points to it. The `Box` ensures that when the node is no longer needed (i.e., when the `Box` 
goes out of scope), the memory is automatically deallocated.

This is particularly useful in system-level programming where low-level memory management is crucial.

### Output:

When you run this code, the output will look like:

```
Value: 1
Value: 2
Value: 3
Value: 4
Value: 5
```

Each node’s value is printed in the order they were added to the linked list. 
The use of `Box<T>` allows the list to grow dynamically, and each node is safely managed in heap memory.

### Advantages of Using `Box<T>` in System Programming

* **Control over Memory Allocation:** `Box<T>` allows you to manage dynamic memory allocation explicitly 
  while benefiting from Rust's ownership model to ensure safety.

* **Efficient Memory Usage:** Data that doesn’t need to be kept on the stack can be moved to the heap to 
  reduce stack usage, which is especially important in systems with limited stack space (like embedded sys).

* **Automatic Memory Cleanup:** Rust’s ownership system ensures that when a `Box<T>` goes out of scope, the 
   memory it points to is automatically freed, reducing the chance of memory leaks.


### Enabling Recursive Types with Boxes:

A value of a recursive type can have another value of the same type as part of itself. 

Recursive types pose an issue because Rust needs to know at compile time how much space a type takes up. 

However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t 
know how much space the value needs. Because boxes have a known size, we can enable recursive types by 
inserting a box in the recursive type definition.


