#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }
    fn push(&mut self, value: T) {
        self.elements.push(value)
    }
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
    fn peek(&mut self) -> Option<&T> {
        self.elements.last()
    }
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.pop();
    stack.pop();

    println!("Stack is!{:?}", stack.is_empty());
}
