use tokio::runtime::enter::context;

fn cenas(){
    println!("Hello boys!");
    let i : i64 = 5;let mut num = 10;
    print!("{}",i);
    num+=1;
    println!("Max i8 {}", i8::MAX);
    let is_it_true = true;
    println!("I am {1} years {0}", "15", "old");
    println!("{:.2}", 1.234);

    //tips of how to output
    println!("B: {0:b} H: {0:x} O: {0:o}",10 );

    //named arguments
    println!("{var}", var = 2);

    //white spaces
    println!("{1:a<0$}", 5, 2);

    for z in 1..10{
        println!("FOR : {}", z);
    }
}

fn fac (num : u32) -> u32 {
    (1..=num).product()
}

fn main() {
    let node = tokio::new();

    println!("boas");
}