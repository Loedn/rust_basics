fn main() {
    // Variables
    // Vars by default are immutable, unless you prefix them with mut like age
    let name = "Alessandro";
    let mut age = 28;

    // let amount = 763298234987324; <- this won't run because it's more than 32bits so we have to declare it
    let amount:i64 = 763298234987324; // <- all is good now

    // shadowing is allowed -> you can reassign the variable if you re-declare it as follows
    let color = "blue";
    println!("{}", color);
    let color = "red";
    println!("{}", color);

    // multiple var assignment
    let (a, b, c) = ("yolo", 65, 12);


}
