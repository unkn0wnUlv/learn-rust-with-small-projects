pub static DEBUG: bool = true;

// Should I make this a macro?
pub fn debug(s : String) -> i32 {
    if DEBUG {
        println!("DEBUG: {}", s);
    }
    0
} 

