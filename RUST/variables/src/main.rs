fn main() {
    let mut x = 5; //refedingin using mut
    x = 6;
    println!("[+] debug:: {}", x);

    let y = 4; //redefining using let
    println!("[+] debug:: {}", y);
    let y = 5;
    println!("[+] debug:: {}", y);

    const THREE_HOURS_CALC: u32 = 13;
    println!("[+] debug:: {}", THREE_HOURS_CALC);

    let z = 5;
    println!("[+] pre-scope debug:: {}", z);
    let z = z + 1;
    println!("[+] pre-scope debug:: {}", z);
    {
        let z = z * 2;
        println!("[+] scope debug:: {}", z);
    }
    println!("post-scope debug:: {}", z);

    let spaces = "     ";
    println!("spaces-debug:: {}", spaces);
    let spaces = spaces.len();
    println!("spaces-len-debug:: {}", spaces);

    let i = 0x0f_i32;
    println!("literal-debug:: {}", i);

    let c = '😻';
    println!("char-literal-debug:: {}", c);
    let a = "hello freind";
    println!("string-literal-debug:: {}", a);

    //example tuple
    //tuples are fixed length once initalized and defined 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (j,f,k) = tup;
    println!("the value of j:: {}", j);
    println!("the valuse of f:: {}", tup.1);
    //second line prints a tuple index 

    let e = [1, 2, 3, 4, 5];
    let months = ["jan", "feb", "march", "april"];
    let o: [i32; 0x05] = [0,1,2,3,4];
    //setting array with data type and size
    println!("e-debug:: {}, months-debug:: {}", e[1], months[1]);
    let g = [3;5];
    //the same as let a = [3, 3, 3, 3, 3];
    let test_array = g[2];
    println!("debug:: {}", test_array);

}
