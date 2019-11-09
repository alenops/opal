// Loops -used to iterate until a condition is met


pub fn run(){
    let mut x=0;

    // infenate loop
    loop {
    x += 1;
    println!("number:{}",x );

    if x == 20{
        break;
    }
}

// while loop (fizzbuzz)
// while x<=150{
//     if x%15==0{
//         println!("fizzbuzz", );
//     }else if x%3==0{
//         println!("fizz", );
//     }else if x%5==0{
//         println!("buzz", );
    
//     }else{ println!("{}",x );}
//     x +=1;
// }

// for range loop
for x in 0..151{
    if x%25==0{
        println!("fizzbuzz", );
    }else if x%10==0{
        println!("fizz", );
    }else if x%3==0{
        println!("buzz", );
    
    }else{ println!("{}",x );}

}

}