use std::rc::Rc;

// enum is as large as its largest member
// &List is likely to put the list on the stack
// Box means that there is only one pointer
#[derive(Debug, Clone)]
struct List<T:Clone> {
    link: Option<Rc<Cons<T>>>,
}

#[derive(Debug, Clone)]
struct Cons<T:Clone> {
    val: T,
    tail: List<T>,
}

impl List<i64> {
    fn new() -> Self {
        List { link: None }
    }

    fn len(&self) -> i32 {
        match self.link {
            Some(Rc::new(Cons { val, tail })) => tail.len() + 1,
            None => 0,
        }
    }

    fn pushfront(&self, n1: i64) -> Self {
        List {link : Some(Rc::new(Cons {val : n1, tail : self.clone()} ) ) }
    }
}

fn main() {
    let mut lst = List::new();
    lst = lst.pushfront(4);
    lst = lst.pushfront(5);
    println!("{:?}", lst.len());
    println!("{:?}", lst.len());
}
