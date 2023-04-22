use std::{fs::File, io::Read};

fn main() {
    // let v = vec![10, 20, 30];
    // println!("v[100]: {}", v[100]);
    //
    // let result = panic::catch_unwind(|| {
    //     println!("hello!");
    // });
    //
    // assert!(result.is_ok());
    //
    // let result = panic::catch_unwind(|| {
    //     panic!("oh no!");
    // });
    //
    // assert!(result.is_err());

    let file = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        }
        Err(err) => {
            println!("The diary coundn't be opened: {err}");
        }
    }
}
