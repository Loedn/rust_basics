fn main() {
    // Variables
    // Vars by default are immutable, unless you prefix them with mut like age
    let name = "Alessandro";
    let mut age = 28; //now we can override this value like the following line
    age = 19;

    // let amount = 763298234987324; <- this won't run because it's more than 32bits so we have to declare it
    let amount:i64 = 763298234987324; // <- all is good now

    // shadowing is allowed -> you can reassign the variable if you re-declare it as follows
    let color = "blue";
    println!("{}", color);
    let color = "red";
    println!("{}", color);

    // multiple var assignment
    let (a, b, c) = ("yolo", 65, 12);

    // Scalar data types
    // ints by default are i32 == 32bits
    // floats by default are f64 == 64bits
    // let pi: f32 = 4; <- this will throw an mismatched types error  
    let pi: f32 = 4.0; // <- this works like a charm because you're converting two floats
    let million = 1_000_000; // '_' are used as number separators simply for ease of reading
    
    let is_day = true; // booleans
    let is_night = false; // booleans

    let char1 = 'A'; // '' single quotes are for single chars you can not assign 'ab'
    let chars = "Ab"; // all good
    let smile = '\u{1F601}';
    println!("{}", smile); // go ahead, try it!

    // Strings
    let cat: &str = "Fluffy"; // <- these two are slices not real strings
    let cat: &'static str = "Fluffy"; // <- these two are slices not real strings
    
    let dog = String::new();
    let mut  dog = String::from("Max");
    
    // String methods
    let owner = format!("Hi i'm {} the owner of  {}", "Mark", dog);
    println!("{}", dog.len());

    dog.push(' '); // <- pushes a white space after max
    dog.push_str("the dog");
    println!("{}", dog); // <- check it out "Max the dog"

    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog); // <- check it out "Max is my dog"
}
