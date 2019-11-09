//    funcsions --used to store blocks of code for re-use
pub fn run(){
    greeting("hello", "mali",&23);

    // Bind functions values to varables
    let get_sum=add(5,2,3);
    println!("sum:{}",get_sum );


    // closuer
    let n3:i32=1;
    let mali:i32=23;
    let add_nums=|n1:i32,n2:i32|             
    n1+n2+n3+mali;
    println!("C SUM:{}",add_nums(5,8) );
}

fn greeting(greet:&str,name:&str,ops:&i32)
{
    println!("{} {},nice to meet you!I am {}",greet,name,ops );
}

fn add(n1:u64,n2:u64,n3:u64) -> u64{
    n1+n2+n3

}
