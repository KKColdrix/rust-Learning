
pub mod nested;
mod inaccesible;

pub fn function() {
    println!("called `other::function()`");
}

fn private_function() {
    println!("called `other::private_function()`");
}

pub fn other_functions(){
    nested::function();
    inaccesible::public_function();
}
pub fn indirect_access() {
    print!("called `other::indirect_access()`, that\n> ");
    
    private_function();
}
