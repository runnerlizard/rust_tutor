use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::borrow::Borrow;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: Rc<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.boorow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
