fn main() {

    // consts are always immutable and type annotations are required
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    let spaces = "   ";
    
    // take old spaces, extract out length with len(), shadow it with new name spaces
    // will throw error if declared spaces as mutable, no shadowing allowed when mutable
    // variables are at hand
    let spaces = spaces.len();

    // let var_name: type = name; 
    // general way of annotating type


    // compound types
    let tup: (i32, f64, &str) = (1, 2.0, "word");

    // use pattern to map on to tuple, easier to refer to what is stored in the tuple
    // sets up tuple for "destructuring"
    // destructuring is breaking the above tuple into the below three parts:
    let (a, b, c) = tup;

    // aaccessing at each index
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hunny = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // arrays, allocated on stack rather than heap
    // fixed size, for something of non-fixed size (growable) use vector
    // which is stored on heap
    // when you're unsure of when to use a vector or array, opt for vector
    let a = [1, 2, 3, 4, 5];

    // accessing array elements
    let one = a[0];
    let two = a[1];

    // the following will throw a panic, or error
    // let ten = a[10];
    // other languages such as c will actually access invalid memory this way
    // rust immediately exits instead of accessing




}
