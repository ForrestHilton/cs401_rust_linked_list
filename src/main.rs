use std::rc::Rc;

// an enum gives almost no encapsulation while dyn Trait gives a lot.
#[derive(Debug, Clone)]
struct List<T: Clone> {
    link: Option<Rc<Cons<T>>>,
}

#[derive(Debug, Clone)]
struct Cons<T: Clone> {
    val: T,
    tail: List<T>,
}

impl<T: Clone> List<T> {
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
        List {
            link: Some(Rc::new(Cons {
                val: n1,
                tail: self.clone(),
            })),
        }
    }

    fn dropfront(&self, n: usize) -> Self {
        if n == 0 {
            self.clone()
        } else {
            match self.link {
                None => self.clone(),
                Some(ref rc) => rc.tail.dropfront(n - 1),
            }
        }
    }

    fn reverse(&self) -> Self {
        let mut head = List::new();
        let mut tail = self;
        loop {
            match tail.link {
                Some(ref cons) => {
                    head = head.pushfront(cons.val.clone());
                    tail = &cons.tail;
                }
                None => break,
            }
        }
        head
    }
}

fn main() {
    let mut lst = List::new();
    lst = lst.pushfront(1);
    lst = lst.pushfront(2);
    lst = lst.pushfront(3);
    lst = lst.pushfront(4);
    println!("{:?}", lst);
    println!("{:?}", lst.reverse());
}
