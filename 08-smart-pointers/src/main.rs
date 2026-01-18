use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // --- 1. Box<T> for allocating on the heap ---
    // Box provides exclusive ownership of heap-allocated data.
    let b = Box::new(5);
    println!("b = {}", b);

    // --- 2. Rc<T> (Reference Counted) for multiple ownership ---
    // Rc allows multiple pointers to own the same heap allocation.
    // It keeps track of the number of references to decide when to drop the data.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Rc::clone increment the reference count instead of deep copying.
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // _c goes out of scope here, decrementing the reference count.
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // --- 3. RefCell<T> for interior mutability ---
    // RefCell allows you to mutate data even when you have an immutable reference.
    // It enforces borrowing rules (one mutable or many immutable) at RUNTIME.
    let _value = Rc::new(RefCell::new(5));

    // Standalone RefCell demonstration.
    let x = RefCell::new(42);
    println!("x before: {:?}", x);

    // borrow_mut() returns a RefMut smart pointer, allowing us to change the value.
    *x.borrow_mut() += 1;
    println!("x after: {:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_counting() {
        let a = Rc::new(Cons(5, Rc::new(Nil)));
        assert_eq!(Rc::strong_count(&a), 1);

        let _b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);

        {
            let _c = Rc::clone(&a);
            assert_eq!(Rc::strong_count(&a), 3);
        }

        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn test_refcell_mutation() {
        let x = RefCell::new(10);
        *x.borrow_mut() += 5;
        assert_eq!(*x.borrow(), 15);
    }
}
