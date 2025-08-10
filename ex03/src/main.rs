fn own_and_print(s: String)
{
    println!("s = {s}");
}

fn main() {
    //pourquoi passer par .to_string() ? 
    let s: String = "string".to_string();
    //String is a heap type / &str is a stack type ?
    own_and_print(s);
    //to compile with the next line uncommented, we need to "borrow"
    //the value of s using & in the function call (&s) and in the function prototype &String)
    //println!("s = {s}");
}
