pub fn run(){
    //print console
    println!("yahximu rust dunyasi");
    //basic formatting
    println!("{} xinjiang {}","abdugini","kashkalik");

    //positinal arguments
    println!("{0} bulsa {1}'s boyfrieanf so {0} very love {1} ","abduginy","mali");

    //named argument
    println!("{name} likes to play {activity}",name = "abduginy",activity = "table tennis");

    //placeholder traits
    println!("binary: {:b}  hex: {:x}  octal: {:o}",10,10,10);

    //placeholder for debug trait
    println!("{:?}",(12,true,"hello"));

    //basic math
    println!("10+10={}",10+10);
}