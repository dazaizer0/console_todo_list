pub mod edit_todo {
    use std::io;
    use std::collections::HashMap;

    pub fn add_to_todo_list(map: &mut HashMap<i32, String>) -> &mut HashMap<i32, String> {
        let mut text: String = String::new();
    
        io::stdin().read_line(&mut text).expect("error -> adding to todo");
    
        let final_text: String = text.trim().to_string();
        let mut len: i32 = map.len() as i32;
        
        len = len + 1 as i32;
        map.insert(len, final_text);
    
        return map;
    }

    pub fn remove_from_todo_list(map: &mut HashMap<i32, String>, index: i32) -> &mut HashMap<i32, String> {
        map.remove(&index);

        return map;
    }
}
