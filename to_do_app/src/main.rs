use core::task;
use std::fmt::Error;

#[derive(Debug)]
struct Task {
    name: String,
    done: bool,
    id: i32,
}
fn add_task(tasks: &mut Vec<Task>, name: String) {
    let current_id = tasks.len();
    let new_task = Task {
        done: false,
        id: current_id as i32 + 1,
        name,
    };
    tasks.push(new_task);
}
fn find_task(id: i32, tasks: &Vec<Task>) -> Result<String, Error> {
    for task in tasks.iter() {
        if task.id == id {
            return Ok(task.name.clone());
        }
    }
    Err(Error)
}

fn delete_task(id: i32, tasks: &mut Vec<Task>) {
    let task_id = tasks.iter().position(|task| task.id == id);

    match task_id {
        Some(task_id) => {
            tasks.remove(task_id);
            println!("task is removed with id {}", id)
        }
        None => print!("task not found with id {}", id),
    }
}

fn mark_task(id:i32,tasks:&mut Vec<Task>){
    if let Some(task)=tasks.iter_mut().find(|task| task.id == id){
        task.done = true
    }else{
        println!("Task not found with id {}",id);
    }
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    add_task(&mut tasks, "Eating".to_string());
    add_task(&mut tasks, "Sleeping".to_string());
    let task_name = find_task(1, &tasks);
    match task_name {
        Ok(task_name) => println!("The task name is :{}", task_name),
        Err(_) => println!("Some Error Occured"),
    };
    delete_task(1, &mut tasks);
    println!("{:?}",tasks);
    mark_task(2,&mut tasks);
    println!("{:?}",tasks);
}
