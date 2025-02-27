use std::ops::DerefMut;
use std::{ arch::x86_64::_MM_FROUND_NO_EXC, env::var, fmt::Debug, iter, path::Iter};
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let test = Node::new(5f32);
    let mut list = LinkedList::new();

    list.push(1);
    list.push(6);
    list.push(7);
    list.push(3);
    list.push(2);
    list.push(9);

    list.print();

    
    
}
pub struct LinkedList<T:Debug> {
    head: Option<Box<Node<T>>>,
   
}

impl<T> LinkedList<T> where T:Debug{
    

    pub fn new() -> LinkedList<T>{
        LinkedList{
            head:None,
            
        }
    }

    pub fn print(&self){
        for item in self.iter(){
            print!("{:?}", item);
            print!(" ");
        }
    }
    pub fn iter(&self) -> LinkedListIter<T>{
        LinkedListIter{
            current: self.head.as_deref(),
        }
    }
    pub fn push(&mut self, value: T) {
        let mut current = &mut self.head;
    
        while current.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
    
        *current = Some(Box::new(Node::new(value)));
    }


            
    
    }
 
    


pub struct LinkedListIter<'a,T:Debug>{
    current: Option<&'a Node<T>>,
}
impl<'a,T: Debug> Iterator for LinkedListIter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}
#[derive(Clone)]
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
