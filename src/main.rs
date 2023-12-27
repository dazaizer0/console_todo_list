use std::io;
use std::collections::HashMap;

mod fns;

fn main() {
    // create todo map and way variable
    let mut _map: HashMap<i32, String> = HashMap::new();
    let mut _way: i32 = 0;

    // todo list loop
    while _way != 4  {
        // print ways
        println!("1 = add -> [what], 2 = remove -> [index], 3 = show list -> , 4 = exit -> ");
        print!("> ");
        
        // get way number
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
        
        // convert input 
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("wrong input");
                return;
            }
        };
        
        // way = input
        _way = number;
        
        // way nr 1
        if _way == 1 {

            fns::edit_todo::add_to_todo_list(&mut _map);
            println!("element added...\n");
        }

        // way nr 2
        else if _way == 2 { 

            // get index
            let mut index = String::new();
            io::stdin().read_line(&mut index).expect("error");
            
            // convert input
            let index: i32 = match index.trim().parse() {
                
                Ok(num) => num,
                Err(_) => {
                    println!("wrong input");
                    return;
                }
            };
            
            // remove element of todo list
            fns::edit_todo::remove_from_todo_list(&mut _map, index);
            println!("finish...\n");
        }

        // way nr 3
        else if _way == 3 {
            // show todo list
            println!("\n");
            for (key, value) in &_map{
                
                println!("nr: {}: {}", key, value);
            }

            println!("\n");
            println!("finish...");
        }
        
        // way nr 4 -> end main loop
        else if _way == 4 {
            print!("finish...");
        }
        else {
            print!("none")
        }
    }
}


