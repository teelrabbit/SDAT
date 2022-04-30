fn main() {
    let x = add(5);
    println!("Hello, world!");
    another_function(0x15);
    print_label_measurement(0x14, 'G');
    student_test(0x02, 4.5);
    println!("just a number {}", five());
    println!("add func:: {}", x);
}

fn another_function(x: i32) {
    println!("Another function:: {}", x);
}

fn print_label_measurement(value: i32, label: char) {
    println!("measuremnt-label:: {} {}", value, label);
}

fn student_test(uno: i32, dos: f64) {
    println!("test-debug:: {}, {}", uno, dos);
}

fn five() -> i32 { //showing return value 
    5
}

fn add(x: i32) -> i32 {
    x + 1
}