use std::{thread, time::Duration};

fn main() {
    const DAY_WHICH: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    const DAY_GIFTS: [&str; 13] = [
        "A partridge in a pear tree",
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    thread::sleep(Duration::from_secs(1));

    println!("The Twelve Days of Christmas");

    let mut day = 1;

    while day <= 12 {
        thread::sleep(Duration::from_secs(1));
        println!("On the {} day of Christmas,", DAY_WHICH[day - 1]);
        thread::sleep(Duration::from_secs(1));
        println!("my true love sent to me");
        if day == 1 {
            thread::sleep(Duration::from_secs(1));
            println!("{}.", DAY_GIFTS[0]);
        } else {
            for i in (1..=day).rev() {
                if i == 1 {
                    thread::sleep(Duration::from_secs(1));
                    println!("And {}.", DAY_GIFTS[i]);
                } else {
                    thread::sleep(Duration::from_secs(1));
                    println!("{},", DAY_GIFTS[i]);
                }
            }
        }

        println!(" ");
        day += 1;
    }
}
