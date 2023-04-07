// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![131, 131, 3341, 13, 41];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    //
    // let char_list = vec!['c', 'a', 'g', 'd'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    //
    // let integer = Point { x: 5, y: 4 };
    // let float = Point { x: 1.5, y: 5.1 };
    // let integer_and_float = Point { x: 1.0, y: 111 };
    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
