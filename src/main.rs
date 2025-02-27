use std::{ fmt::Debug};

fn main() {
    let test = Node::new(5f32);
    test.draw();
    
    
}
pub struct LinkedList<T:Debug> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

pub struct Node<T:Debug> {
    value: T,
    next: Option<Box<Node<T>>>,

    
}

impl<T> Node<T> where T:Debug{
    pub fn draw<>(&self){
        print!("{:?}", self.value);
        print!(" ");


    }
    pub fn new(T: T)   -> Node<T>
    where T:Debug{
        Node{
            value: T,
            next: None,
        }

    }
}
