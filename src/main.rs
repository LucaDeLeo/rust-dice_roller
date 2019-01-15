#[macro_use] extern crate text_io;
use std::time::SystemTime;
use rand::Rng;

fn main()
{
    println!("Dice Roller!!!\nPlease enter the dice you wanna roll:");
    loop
    {
        let dice: String = read!();
        let now = SystemTime::now();
        let dice: Vec<&str> = dice.split('d').collect();
        let mut v: Vec<u16> = vec![];
        let n: u16 = dice[0].parse().expect("Please enter in the format NdM");
        let m: u16 = dice[1].parse().expect("Please enter in the format NdM");

        for _ in 0..n
        {
            v.push(rand::thread_rng().gen_range(1, m+1));
        }
        print!("{}:", v.iter().sum::<u16>());
        for i in &v
        {
            print!(" {}", i);
        }
        print!(" {:?} \n", now.elapsed());
    }
}