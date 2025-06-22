
use std::collections::VecDeque;
use std::sync::{Arc,Mutex};
use std::thread;

struct TaskQueue{
    queue:Mutex<VecDeque<String>>
}
impl TaskQueue{
    fn new()->Self{
        TaskQueue { queue: Mutex::new(VecDeque::new()) }
    }
    fn add_task(&self,task:String){
        let mut tasks=self.queue.lock().unwrap();
        tasks.push_back(task);   
    }
    fn len(&self)->usize{
        let len=self.queue.lock().unwrap().len();
        len
    }
    fn get_task(&self)->Option<String>{
        let mut tasks=self.queue.lock().unwrap();
        let task=tasks.pop_front();
        task
    }
}
fn main() {
    let task_queue=Arc::new(TaskQueue::new());
    let mut handles=vec![];
    for i in 0..5 {
        let task_queue_clone=Arc::clone(&task_queue);
        let handle=thread::spawn(move ||{
                let task=format!("task from queue {}",i);
                task_queue_clone.add_task(task.clone());
                if let Some(popped)=task_queue_clone.get_task(){
                    println!("Thread {} got task {}",i,popped);
                }else{
                    println!("Thread {} got no task",i);
                }
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Final length of task queue is {}",task_queue.len());
}
