#![allow(dead_code)]
use std::{cell::OnceCell, rc::Rc};

#[derive(Debug, Default)]
struct MyStruct {
    distance: usize,
    root: Option<Rc<OnceCell<MyStruct>>>,
}

fn main() {
    let root = MyStruct::default();
    let root_cell = Rc::new(OnceCell::new());
    if let Err(previous) = root_cell.set(root) {
        eprintln!("Previous Root {previous:?}");
    }
    let child_1 = MyStruct {
        distance: 1,
        root: Some(root_cell.clone()),
    };

    let child_2 = MyStruct {
        distance: 2,
        root: Some(root_cell),
    };

    println!("CHild 1: {child_1:?}");
    println!("CHild 2: {child_2:?}");
}
