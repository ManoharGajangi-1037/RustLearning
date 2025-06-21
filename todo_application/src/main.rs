use std::io::{self, Write};


#[derive(Debug)]
struct Task{
    id:u32,
    title:String,
    done:bool
}

struct TodoList{
    tasks:Vec<Task>,
    next_id:u32
}
impl TodoList{
    fn new()->Self{
        Self { tasks: Vec::new(), next_id: 1 }
    }
    fn add_task(&mut self,title:String){
        let task=Task{
            done:false,
            id:self.next_id,
            title:title
        };
        self.tasks.push(task);
        self.next_id+=1;
    }
    fn mark_done(&mut self,id:u32)
    {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id==id){
            task.done=true;
        }else{
            println!("Task Not found");
        }
    }

    fn list_tasks(&self){
        for task in &self.tasks{
           let status = if task.done { "✔" } else{"Not done"};
           println!("[{}] {} {}",status,task.id,task.title);
        }
    }
}
fn main() {
    println!("Hello, world!");
    let mut todo=TodoList::new();
    loop{
        println!("\n Commands :add ,done <id>,list,exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit"{
             break;
        }
        else if input == "list"{
            todo.list_tasks();
        }
        else if input.starts_with("add"){
            let title = input .strip_prefix("add").unwrap().trim();
            if !title.is_empty(){
                todo.add_task(title.to_string());
            }else{
                println!("Provide a title");
            }
        } 
        else if input.starts_with("done") {
            if let Some(id_str) = input.split_whitespace().nth(1){
                 if let Ok(id) =id_str.parse::<u32>(){
                       todo.mark_done(id);
                 }
            }
        }else{
            println!("unknown command");
        }
        
    }
}
