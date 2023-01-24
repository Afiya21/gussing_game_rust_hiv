use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("guessing game");
    let secrate = rand::thread_rng().gen_range(0..=100);
    println!("secrate number is :{secrate}");
    loop{
    println!("please guess the number");

    let mut guess= String::new();
    io::stdin()
    .read_line(&mut guess).
    expect("Error");

    let guess :u32 =match guess.trim().parse(){
        Ok(num)=> num,
        Err(_)=>continue,
    };
    println!("your guess number is :{guess}");
    match guess.cmp(&secrate){
        Ordering::Less=>println!("too small"),
        Ordering::Greater=> println!("too big"),
        Ordering::Equal=>{
            println!("you won");
            break;
        }

    }

}
}