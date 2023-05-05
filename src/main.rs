mod other;

use std::mem;
use test::nested_t::function as my_nested_t_function;
type Hours = u8;


mod test {
    pub mod nested_t {
        pub fn function(){
            println!("hola desde nested_t");
        }
    }
}
fn main(){

    //con "_" no te marca unused variable

    // _struct_basics();
    // _array_basics();
    let boolean: bool =  _casting_basics();
    
    println!("is boolean true? : {0}",&boolean);
    
    let time : Hours = 24;
    print!("what time is it: {time} hours");

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
    // let z = {
    //     // The semicolon suppresses this expression and `()` is assigned to `z`

    //     2 * x;
    // };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    //println!("z is {:?}", z);


    let test = 
        if boolean == false {
            10
        }
        else{
            20
        };

    println!("{test}");

    'outer: loop {
        println!("Entered the outer loop");

        '_inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        //println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    assert_eq!(result, 20);
    
    for n in 1..=10  {
        println!("n : {n}")
    }
    
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);

    let num: u8 = 13;

    match num {
        1 => print!("uno"),
        2 | 4 | 6 | 8 | 10 => print!("par / 1 al 10"),
        _ => print!("nada")
    }

    
    let is_true = match num {
        13 => "si",
        20 => "no",
        _ => "talvez"
    };

    print!("is true? {is_true}");

    let my_rectangle: Rectangle = Rectangle {
        p1: Point::_origin(),
        p2: Point::_new(3.0, 4.0),
    };
    
    println!(" my rectangle perimeter: {}", my_rectangle._perimeter());
    println!(" my rectangle area: {}", my_rectangle._area());
    
    my_nested_t_function();

    other::function();

    other::indirect_access();

    other::other_functions()
}

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point 
}

impl Point {
    fn _origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0
        }
    }

    fn _new(x: f64,y: f64)-> Point{
        Point { x: x, y: y }
    }
}

impl Rectangle {
    fn _area(&self) -> f64 {
        let Point {x: x1, y: y1 } = self.p1;
        let Point {x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn _perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
}



fn _array_basics() {


    let xs: [i32; 5] = [1, 2, 3, 4, 5]; //Basic array of integers

    let ys: [i32; 100] = [0; 100]; // All elements can be initialized to the same value.
    
    let my_array: [i32; 5] = [5; 5]; // ^ same as here 

    let xs_size_in_byts: usize = mem::size_of_val(&xs);

    let xs_size_in_bits: usize = xs_size_in_byts * 8;
    
    println!("\nfirst item of array {0}", xs[0]);

    println!("first item of array {0}", ys[0]);

    println!("third item of array {0}", my_array[2]+1);  
    
    println!("Array occupies {} bytes", xs_size_in_byts); //size of xs in bytes

    println!("Array occupies {} bits", xs_size_in_bits); //size of xs in bits
    
    println!("Number of elements in array: {0}", ys.len()); //length of ys  

    println!("\nprinting the my_array content: \nplace : value");


    for i in 0..my_array.len(){
        println!("   |{1} : {0}|",my_array[i], i);
    }

}
struct _Person {
    name: String,
    age: u8,
}

fn _struct_basics(){

    let carlos: _Person = _Person {
        name: ("carlos".to_owned()),
        age: (15) };
   
    println!("person name: {0}\nperson age: {1}",carlos.name, carlos.age);
   
    println!("happy birthday {0} now you have {1} years old", carlos.name,carlos.age+1);

    let _siu: i32 = 10;
    
    {
        println!("before being shadowed: {}", _siu);
        let _siu: &str = "abc";
        println!("shadowed in inner block: {}", _siu);
    }

    println!("outside inner block: {}", _siu);
    let _siu: i32 = 2;
    println!("shadowed in outer block: {}", _siu);

}

fn _casting_basics() -> bool{

    let x: u8 = 1u8;

    let decimal: f32 = 250.2121 + x as f32; //use "as" for explicit conversion
    
    let entero: u8 = decimal as u8;
    
    let caracter : char = (entero as u8) as char;
    //let caracter: char = entero as char;
    
    let y: u8 = decimal as u8;

    println!("decimal : {0}\nentero : {1}\ncaracter :..{2}..",decimal,entero,caracter);
    println!("decimal {y}");
    
    println!("300.8 as u8 {0}",300.0 as u8);
    true
}