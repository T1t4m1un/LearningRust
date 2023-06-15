
#[derive(Debug)]
enum ListNode<T> {
    Struct(T, Box<ListNode<T>>),
    Nil,
}

impl<T> ListNode<T> {
    fn len(&self) -> u32 {
        match self {
            ListNode::Nil => 0,
            ListNode::Struct(_, ref next) => 1 + next.len(),
        }
    }
}

#[derive(Debug)]
struct List<T> {
    head: Box<ListNode<T>>,
}

impl<T> List<T> {
    /// Create a new empty Linklist
    fn new() -> Self {
        List { head: Box::new(ListNode::Nil) }
    }

    /// Insert at head
    fn push(&mut self, value: T) {
        let next = std::mem::replace(
            &mut self.head, Box::new(ListNode::Nil));
        self.head = Box::new(ListNode::Struct(value, next));
    }

    /// Pop at head
    fn pop(&mut self) -> Option<T>  {
        match *std::mem::replace(&mut self.head, Box::new(ListNode::Nil)) {
            ListNode::Nil => None,
            ListNode::Struct(val, next) => {
                self.head = next;
                Some(val)
            }
        }
    }

    /// Calculate the length
    fn len(&self) -> u32 {
        self.head.len()
    }
}

fn main() {
    let mut list = List::new();

    assert_eq!(list.len(), 0);
    assert_eq!(list.pop(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.len(), 3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    list.push(4);
    list.push(5);

    assert_eq!(list.len(), 3);
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

