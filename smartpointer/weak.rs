use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("{:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf\n strong_count: {}, weak_count: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "leaf strong_count: {},leaf weak_count: {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        println!(
            "branch strong_count: {}, branch weak_count: {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
    }

    println!("{:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf\n strong_count: {}, weak_count: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
