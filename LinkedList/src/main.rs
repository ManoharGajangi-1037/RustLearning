
#[derive(Debug)]
struct Node{
    value:i32,
    next:Option<Box<Node>>
}
#[derive(Debug)]
struct LinkedList{
    head:Option<Box<Node>>
}

impl LinkedList{
     fn new()->Self{
        LinkedList{head:None}
     }
     fn push(&mut self,value:i32){
        let new_node=Box::new(Node{
            value:value,
            next:None
        });
        match self.head.as_mut(){
            Some(mut node)=>{
                while let Some(ref mut next) = node.next {
                    node = next;
                }
                node.next=Some(new_node);
            }
            None=>self.head=Some(new_node),
        }
     }
     fn pop(&mut self){
        if let Some(mut node)=self.head.take(){
            self.head=node.next.take();
        }
     }
}
fn main() {
    println!("Hello, world!");
    let mut linked_list=LinkedList::new();
    linked_list.push(2);
    linked_list.push(4);
    linked_list.pop();
    print!("{:?}",linked_list)
}
