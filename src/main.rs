use std::rc::Rc;

#[derive(Debug, Clone)]
struct List<T:Clone> {
    link: Option<Rc<Cons<T>>>,
}

#[derive(Debug, Clone)]
struct Cons<T:Clone> {
    val: T,
    tail: List<T>,
}

impl<T:Clone> List<T> {
    fn new() -> Self {
        List { link: None }
    }

    fn len(&self) -> i32 {
        match self.link {
            Some(ref cons) => cons.tail.len() + 1,
            None => 0,
        }
    }

    fn pushfront(&self, n1: T) -> Self {
        List {link : Some(Rc::new(Cons {val : n1, tail : self.clone()} ) ) }
    }
}

fn main() {
    let mut lst = List::new();
    lst = lst.pushfront(4);
    println!("{:?}", lst.len());
    lst = lst.pushfront(5);
    println!("{:?}", lst.len());
}
