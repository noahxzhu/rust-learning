use std::io;

fn main() {
    loop {
        println!("Please input the level of fabonacci:");

        let mut level = String::new();

        io::stdin()
            .read_line(&mut level)
            .expect("Failed to read line");

        let level: usize = match level.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        fabonacci(level);
    }
}

fn fabonacci(level: usize) {
    let mut result = vec![1; level];

    if level > 2 {
        for i in 2..level {
            result[i] = result[i - 2] + result[i - 1];
        }
    }
    for value in result {
        print!("{value} ");
    }
    println!(" ");
}
