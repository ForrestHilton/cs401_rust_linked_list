use std::{io::Empty, rc::Rc};

// enum is as large as its largest member
// &List is likely to put the list on the stack
// Box means that there is only one pointer
#[derive(Debug)]
enum List {
    Empty,
    Node(i64, Rc<List>),
}

impl List {
    fn new() -> Self {
        List::Empty
    }

    fn len(&self) -> i32 {
        match self {
            List::Node(_, next) => next.len() + 1,
            List::Empty => 0,
        }
    }

    fn pushfront(&self, n1: i64) -> Self {
        match self {
            List::Empty => List::Node(n1, Rc::new(List::Empty)),
            List::Node(n, ptr) =>  List::Node(n1, Rc::new(List::Node(*n, ptr.clone()))),
        }
    }
}

fn main() {
    let mut lst = List::new();
    lst = lst.pushfront(4);
    lst = lst.pushfront(5);
    println!("{:?}", lst.len());
    println!("{:?}", lst.len());
}
