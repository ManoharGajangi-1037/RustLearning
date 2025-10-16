trait Command {
    fn run(&self);
}

impl Command for InitCommand {
    fn run(&self) {
        println!("Initialized by {}",self.creator);
    }
}

impl Command for TransferCommand{
    fn run(&self){
        println!("transfer of {} :{} {}",self.from,self.to,self.amount)
    }
}

impl Command for CloseCommand{
    fn run(&self) {
        println!("closed {}",self.creator)
    }
}
struct InitCommand {
    creator: String,
}
struct TransferCommand {
    from: String,
    to: String,
    amount: i32,
}
struct CloseCommand {
    creator: String,
}

enum CommandType {
    Init(InitCommand),
    Transfer(TransferCommand),
    Close(CloseCommand),
}

fn main() {
    let command=InitCommand{
        creator:"Megha".to_string()
    };
    let actual_command=CommandType::Init(command);
    match actual_command{
        CommandType::Init(actual_command)=>{
            actual_command.run();
        }
        CommandType::Close(actual_command)=>{
           actual_command.run();
        }
        CommandType::Transfer(actual_command)=>{
           actual_command.run();
        }
    }
}
