use std::io;
use std::io::Write;
use std::collections::HashMap;
use colored::*;

#[derive(Debug)]
// struct for colored output
struct Log
{
    err:    &'static str,
    succs:  &'static str,
    info:   &'static str,
    reset:  &'static str
} 

#[derive(Debug)]
struct Manager
{
    local_todos: HashMap<String, HashMap<&'static str, String>>,
    // big hash map that holds hashmaps
    log: Log 
    // using struct for colored output in a log-like style

}

impl Manager
{
    fn add_todo(&mut self, title: &str)
    {
        if self.local_todos.contains_key(title)
        {
            println!(r#"{}A to-do with title '{}' already exists{}"#, self.log.err, title, self.log.reset);
            return;
        }
        
        // creates a sub hash map that contains data about your todo under the 'title'
        let mut sub_todo = HashMap::new();
        sub_todo.insert("title", title.to_string());
        sub_todo.insert("state", String::from("ongoing"));

        // adding the sub to-do content to the big juicy daddy to-do
        self.local_todos.insert(title.to_string(), sub_todo);

        println!(r#"{}Added to-do{}"#, self.log.succs, self.log.reset);
    }

    fn remove_todo(&mut self, title: &str)
    {
        if !self.local_todos.contains_key(title)
        {
            println!(r#"{}To-do title '{}' has not been created yet{}"#, self.log.err, title, self.log.reset);
            return;
        }
        
        self.local_todos.remove(title);
        // comment.

        println!(r#"{}To-do with title '{}' was removed{}"#, self.log.err, title,self.log.reset);
    }

    fn update_state(&mut self, title: &str)
    {
        if !self.local_todos.contains_key(title)
        {
            println!(r#"{}To-do title '{}' doesnt exist{}"#, self.log.err, title, self.log.reset);
            return;
        }

        // getting to the sub hashmap
        if let Some(todo) = self.local_todos.get_mut(title)
        {
            // getting to the state from the sub hashmap
            if let Some(state) = todo.get_mut("state")
            {
                if *state == "ongoing"  { *state = "finished".to_string(); }
                else if *state == "finished" { *state = "ongoing".to_string();}
                else
                {
                    println!(r#"{}Invalid state found, defaulting to unfinished{}"#, self.log.info, self.log.reset);
                    *state = "ongoing".to_string();
                }
            }
        }

        println!(r#"{}State updated{}"#, self.log.succs, self.log.reset);
    }

    fn list_todos(&self)
    {
        println!("{}Your list of to-dos:{}", self.log.info, self.log.reset);
        for (key, value) in self.local_todos.iter() 
        {
            let state = value.get("state").expect("keys");
            // {{{{{{{{{{{{{{{{{{{{{}}}}}}}}}}}}}}}}}}}}}
            println!("{}{}{} - {}", self.log.info, key, self.log.reset, if state == "finished" {state.green()} else {state.red()} );
        } 
    }
}

fn main() 
{
    let log = Log {
        err:    "\x1b[31m",
        succs:  "\x1b[32m",
        info:   "\x1b[35m",
        reset:  "\x1b[0m"
    };

    let mut mng = Manager {
        local_todos: HashMap::new(),
        log
    };
    
    loop
    {
        print!("cmd > ");
        io::stdout().flush().unwrap(); // clears the output so print!() can display correctly

        let mut input_buffer = String::new(); // user input buffer
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Invalid input");

        // token-like vec containing all args
        let input: Vec<&str> = input_buffer.as_str().trim().split(" ").collect();
        let cmd: &str = input[0].trim(); // first arg which means command 

        // second arg which usually means title
        let mut arg1: &str = ""; // the fuck you mean "value is never used" stupid rust_analyzer
        if let Some(_) = input.get(1) // checks and sets the value if therr is an argument
        { 
            arg1 = input[1].trim(); 
        }
        else 
        { 
            arg1 = ""; 
        }

        // checking for cmds
        match cmd.trim() {
        // argument based commands
            "add" => { // add a to do
                if arg1 != "" { mng.add_todo(arg1) }
                else { println!("{}Not enough args{}", mng.log.err, mng.log.reset) }
            }

            "remove" => { // unset a to do
                if arg1 != ""  { mng.remove_todo(arg1) }
                else { println!("{}Not enough args{}", mng.log.err, mng.log.reset) }
            }

            "update" => { // update a todo's state
                if arg1 != "" { mng.update_state(arg1) }
                else { println!("{}Not enough args{}", mng.log.err, mng.log.reset) }
            }

        // normal commands without args
            "list" => mng.list_todos(), // list every set to-do

            "exit" => { // exit the program
                println!("{}Exiting...{}", mng.log.info, mng.log.reset);
                std::process::exit(1);
            }
        // fallback
            _ => {
                if !cmd.is_empty() { println!("{}Invalid command '{}'{}", mng.log.err, cmd, mng.log.reset); }
            }
        }
    }
}
