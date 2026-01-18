use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 1. Box<T> for allocating on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    // 2. Rc<T> for multiple ownership
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // 3. RefCell<T> for interior mutability
    let _value = Rc::new(RefCell::new(5));

    // Just demonstrating RefCell usage standalone for simplicity
    let x = RefCell::new(42);
    println!("x before: {:?}", x);

    *x.borrow_mut() += 1;
    println!("x after: {:?}", x);
}
