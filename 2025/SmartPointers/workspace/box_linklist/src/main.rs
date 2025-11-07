// Defining a Linked List Node using Box
struct ListNode {
    value: u32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    // Constructor to create a new ListNode
    fn new(value: u32) -> ListNode {
        ListNode { value, next: None }
    }

    // Add a new node to the list
    fn add_next(&mut self, node: ListNode) {
        let mut current = self;
        //Traverse untill current.next is None ( i.e last node )
        while let Some(ref mut next_node) = current.next {
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
        println!("Value: {}", current.value); // print the last node
    }
}

fn main() {
    {
        let b = Box::new(5); // Heap allocation
        println!("b = {}", b);
    }

    // Create the first node
    let mut head = ListNode::new(1);

    // Add subsequent nodes
    head.add_next(ListNode::new(2));
    head.add_next(ListNode::new(3));
    head.add_next(ListNode::new(4));
    head.add_next(ListNode::new(5));
    head.add_next(ListNode::new(6));
    head.add_next(ListNode::new(7));

    // Print the list
    head.print();
}
