pub fn run(){
    //default is "i32"
    let _x =1;
    //default is "f64"
    let _t=20.5;
    //add explicit type
    let _y: i64=15454552;
    //find max_size
    println!("MAX i32 :{}",std::i32::MAX);
     println!("MAX i64:{}",std::i64::MAX);

     //boolean
     let is_activity= true;
  //get boolean from expression
     let is_greater =10<5;

     let a1='a';
     let face='\u{1F600}';
     println!("{:?}",(_x,_t,_y,is_activity,is_greater,a1,face));
     
   





}