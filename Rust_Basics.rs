fn main() {
    
    //STRING, FORMAT STRINGS
    println!("{} days", 31);
    
    println!("{a} {b} {c}",
            a = "Who",
            b = "are",
            c = "you?");
            
    println!("{number:>5}", number=3);
    println!("{number:0>5}", number=3);
    println!("{number:0<5}", number=3);
    
    // println!("My name is {0}, {1} {0}", "Bond");
    
    //FUNCTION
    fn add(a: i32, b:i32) -> i32{
        a + b
    }
    
    let x = add(3030, 393);
    println!("{}", x);
    
    //DEBUG PRINT
    let life = 13;
    println!("{:?}", life);
    
    //CONDITIONALS
    let num = 99;
    if num > 99{
        if num > 200{
            println!("Huge number");
        }
        else{
            println!("Big number");
        }
    }
    else{
        println!("Small number");
    }
    
    //LOOPING
    let mut a = 0;
    loop {
        if a == 5{
            break;
        }
        println!("{}", a);
        a = a + 1;
    }
    
    let mut z = 0;
    while z != 5{
        println!("{}", z);
        z = z + 1;
    }
    
}