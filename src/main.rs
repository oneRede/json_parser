mod lexer;
fn main() {
    let s = "\"key\"";
    println!("{:?}", s.len());
    for c in s.chars(){
        println!("{:?}", c);
    }
    
}
