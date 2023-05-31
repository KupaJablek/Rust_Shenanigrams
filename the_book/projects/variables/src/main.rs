fn main() {
    let x = 5; // add "mut" to x to allow reassigning
    println!("The value of x is {x}");

    //x = 7 -> illegal
    //println!("the value of x is: {x}");
    // cant reassign a value to x -> because x is not mutable 
    scope_test();
}

const BIG_OL_NUMBER: u32 = 60 * 60 * 3; // this is not used and will produce a warning

fn scope_test() {
    let x = 5; // let allows for shadowing of variables. let introduces a new variable into the
               // scope, given by a pattern.
               // much like JS let

    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x is the inner scope: {x}");
    }
    println!("the value of x is: {x}");
}

fn type_change() {
    // mut keyword keep variable of same type;
    let mut var1 = 2;
    var1 = 3;

    // let keyword allows for assignment of new type 
    let spaces = "   ";
    let spaces = spaces.len();
}
