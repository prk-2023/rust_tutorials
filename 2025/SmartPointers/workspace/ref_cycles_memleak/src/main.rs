use std::cell::RefCell;
use std::rc::{Rc, Weak};

// --- 1. The Structure (Node) ---
// Note: Parent is Weak<Node> to prevent the cycle leak.
#[derive(Debug)]
struct Node {
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    id: u32,
}

// --- 2. Custom Drop Implementation for Tracking Cleanup ---
impl Drop for Node {
    fn drop(&mut self) {
        println!(
            "🗑️ [DROP] Node ID {} has been dropped (memory freed).",
            self.id
        );
    }
}

fn main() {
    println!("--- Program Start ---");

    // Create an outer scope { } to ensure 'p' and 'c' are dropped here.
    {
        // Create Parent (p)
        let p = Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            id: 1,
        });

        // Create Child (c)
        let c = Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            id: 2,
        });

        println!("(Info) Initial Strong Count P: {}", Rc::strong_count(&p)); // Output: 1
        println!("(Info) Initial Strong Count C: {}", Rc::strong_count(&c)); // Output: 1

        // --- 3. Establish the Cyclic Relationship ---

        // 1. Child holds a WEAK reference back to the Parent. (No count increase)
        *c.parent.borrow_mut() = Rc::downgrade(&p);

        // 2. Parent holds a STRONG reference to the Child. (Increments C's count)
        p.children.borrow_mut().push(Rc::clone(&c));

        println!("---------------------------------");
        println!("(Info) Strong Count P after link: {}", Rc::strong_count(&p)); // Still 1
        println!("(Info) Strong Count C after link: {}", Rc::strong_count(&c)); // Output: 2 (c + p.children)

        // --- 4. Accessing Weak Data (Upgrade) ---
        // To use the parent data from the child, we must upgrade the Weak reference.
        if let Some(parent_rc) = c.parent.borrow().upgrade() {
            println!(
                "(Access) Child {} successfully upgraded to Parent {}.",
                c.id, parent_rc.id
            );
        } else {
            println!("(Access) Failed to upgrade (Parent already dropped).");
        }
    } // <-- SCOPE ENDS HERE: 'p' and 'c' are dropped.

    println!("---------------------------------");
    println!("--- Program End ---");
    // Memory cleanup is verified by the print messages from the Drop trait.
}
