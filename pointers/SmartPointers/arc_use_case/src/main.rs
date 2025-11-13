use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    //Here we try to use Rc but it can't able to move into threads as it has single threaded nature 
    let a = Rc::new(10);

    let mut handles = vec![];
//     for _ in 0..5 {
//         let a_clone = Rc::clone(&a);
//         ///Rc<i32>` cannot be sent between threads safely
// ///within `{closure@src/main.rs:11:36: 11:43}`, the trait `Send` is not implemented for `Rc<i32>`
//         let handle = thread::spawn(move || {
//             println!("Thread sees a = {}", a_clone);
//         });
//         handles.push(handle);
//     }

    let a=Arc::new(10);
    for i in 0..5{
        let a_clone = Arc::clone(&a);
        let handle = thread::spawn(move ||{
             println!("{i}Thread sees a ={}",a_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
