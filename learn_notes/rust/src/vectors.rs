//vectors 向量 矩阵

use std::mem;
pub fn run(){
    let mut numbers: Vec<i32>=vec![1,2,3,4,5,6];
    //一个元素占4个字节

    //re-assign(重新分配值) val
     numbers[2]=20;
      //add on to vector添加元素
    numbers.push(5);
    numbers.push(6);

    //pop off last value弹出最后一个值
    numbers.pop();

    println!("numbers:{:?}",numbers );

    //add on to vector
    numbers.push(5);
    numbers.push(6);

    //get single val
    println!("single val: {}",numbers[0] );

    //get vectors length
    println!("vectors length:{}",numbers.len() );

    //vectors are stack allocated(矩阵被堆栈分配)
    //vectors occupies 矩阵占用                  标准库存储中内存分配    std::mem::size_of_val(&numbers) ); 用这个不用use std
    println!("vectors occupies {} bytes",mem::size_of_val(&numbers) );

//get slice 获取切片
let slice: &[i32]=&numbers;
println!("slice:{:?}",slice );

//获取一部分
let slice: &[i32]=&numbers[0..2];
println!("slice:{:?}",slice );

//loop through vector values
for x in numbers.iter(){
    println!("numbers:{}",x );
}

//loop and mutate values循环和突变值
for x in numbers.iter_mut(){
    *x *=2; //每一个元素乘2

}
println!("numbers: {:?}",numbers );
}