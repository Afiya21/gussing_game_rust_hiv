use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number");
    let secrate = rand::thread_rng().gen_range(1..=100);
    println!("the secrate number is {secrate}");
    loop{
    println!("please input your number");
   

    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("faild to read line");
    let guess: u32 =match guess.trim()
    .parse(){
        Ok(num)=> num,
        Err(_)=> continue,
    };
    
    println!("you gussed:{guess}");

    
    match guess.cmp(&secrate){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too high"),
        Ordering::Equal => {println!("you won");
        break;
    }
    }
    }
}


