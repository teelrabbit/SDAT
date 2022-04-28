use std::io; //importing the std(the standurd libray) and we use the io module (means input output)
use rand::Rng; 
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Guess the number!");
        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("Please input your gues.");

        //let can be used to create a new varible example: "let rabbit = 5"
        //these varibles are by defualt made immutable
        //to make a varible mutable add the "mut" before the varible name
        //example: let mut rabbit = 5;
        let mut gues = String::new();
        //creating a mutable function named gues that returns an instance of a string
        //it makes the string empty by assiating it with the ::new
        //reciving user input
        //we need to call the stdin function from the io module
        //this will allow me to get user input

        io::stdin()
            .read_line(&mut gues)
            //the readline meathod is used to handle user input 
            //it takes in a mutable variable named gues as declared above
            .expect("Failed to read line");
        
            let gues: u32 = gues.trim().parse().expect("Please type a number");
        
            println!("You guessed: {}", gues);
            //the {} is used to be replaced by the gues
        //let gues: u32
            match gues.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => println!("You win!"),
            }
        }  

}