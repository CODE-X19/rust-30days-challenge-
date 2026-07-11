use std::io;
fn main() {

   let mut task_list  = Vec::new();
    loop {
 println!("==== TO DO APP===");
    println!("to add task enter : Add Task");
    println!("to View Tasks enter : View Tasks");
    println!("to Remove Task enter : Remove Task");
    println!("to Exit enter : Exit");
 
            let mut modify_task = String::new();
              io::stdin().read_line(&mut modify_task).unwrap();
             let modify  = modify_task.trim().to_lowercase();
    
               match modify.as_str(){
                  "add task" => {
                        let mut task_input= String::new();
                        println!("Enter your task");
                        io::stdin().read_line(&mut task_input).unwrap();
                         let task= task_input.trim().to_string();
                         task_list.push(task);
                         println!("Task added succesfully ");
                    },
                     "view tasks" =>{
                        if task_list.is_empty() {
                            println!("No tasks available, add some task first")
                        }else {
                        println!("here is youy task");
                           for (index,task) in task_list.iter().enumerate() {
                            println!("{}.  {}", index +1, task );
                        }    
                        }
                         
                    },
                    "remove task" => {

                        if task_list.is_empty() {
                            println!("No tasks available, add some task first")
                        }else {
                               for (index,task) in task_list.iter().enumerate() {
                            println!("{}. {}", index +1, task );
                        }    
                        println!("which task did you want to remove enter the index to remove it ");
                        let mut remove_input = String::new();
                        io::stdin().read_line(&mut remove_input).unwrap();
                        let remove : usize = match remove_input.trim().parse(){
                            Ok(num)=> num,
                            Err(_)=> {
                                println!("Invalide input");
                                 continue;
                            }
                        };
                        if remove > task_list.len() || remove == 0 {
                            println!("Invalid task number");
                        }else {

                            task_list.remove(remove-1 );
                            println!("Task removed successfully!")
                        }
                        }
                     
                    },
                    "exit"=> {break;},


                    _ => {println!("Invaide inpute");}
    
                
      
                } 
   
}
}

