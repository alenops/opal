pub fn run(){
    let mut hello= String::from("abdugy like");
   
//push char
    hello.push('s');

    //push string
    hello.push_str(" mali");


       //get length 长度
    println!("length:  {}",hello.len());

    //capacity in bytes 容量（字节）
    println!("capacity:{}",hello.capacity());

       

    //cheak is empyt 是否空
    println!("is empty:{}",hello.is_empty());

    //contains 包括
    println!("contains  'mali': {}",hello.contains("mali"));

    //replace取代
    println!("replace：{}",hello.replace("mali","malini"));

//loop through stringby whitespace 空格分割
for likes in hello.split_whitespace(){
  println!("{}",likes);
}

//create string with capacity
let mut s=String::with_capacity(10);
s.push('a');
s.push('b');

//assertion testing断言测试
assert_eq!(2,s.len() );

println!("{}",s );

 println!("{}",hello);

    





  

}