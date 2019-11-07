//arrats 数组
//
use std::mem;
pub fn run(){
    let mut numbers: [i32;6]=[1,2,3,4,5,6];
    //一个元素占4个字节

    //re-assign(重新分配值) val
     numbers[2]=20;
    println!("numbers:{:?}",numbers );
    //get single val
    println!("single val: {}",numbers[0] );

    //get arrays length
    println!("arrays length:{}",numbers.len() );

    //arrays are stack allocated(数组被堆栈分配)
    //arrays occupies 数组占用                  标准库存储中内存分配    std::mem::size_of_val(&numbers) ); 用这个不用use std
    println!("arrays occupies {} bytes",mem::size_of_val(&numbers) );

//get slice 获取切片
let slice: &[i32]=&numbers;
println!("slice:{:?}",slice );

//获取一部分
let slice: &[i32]=&numbers[0..2];
println!("slice:{:?}",slice );


}