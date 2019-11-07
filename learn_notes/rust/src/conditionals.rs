//conditionals -use to check the condition of something and act on the result
//条件 - 用于检查某物的状况，并作用于结果
//&&and           ||  or
pub fn run(){
 let score:u8=60;
 let cheak_id:bool=true;
 let knows_id:bool=false;
 //if  /else
 if score>=60 && cheak_id ||knows_id{
     println!("you are pass", );
 }
 else if score<60 && cheak_id {
     println!("you are not  pass",);

 }else {
     println!("I nedd to see yor ID" );

 }

}