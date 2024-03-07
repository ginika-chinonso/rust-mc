use std::{
    error::Error, fmt::Display, fs::File, io::{self, ErrorKind, Read}, os::unix::process
};

pub fn main() {
    let v: Vec<i32> = vec![];

    // let i = File::open("hello2").unwrap();
    // let i = File::open("hello2.txt").expect("File failed to open");

    let mut file = match read_from_file("hello.txt".to_string()) {
        Ok(val) => val,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(val) => val,
                Err(err) => panic!("Failed to create file"),
            },
            _ => panic!("Failed to open file"),
        },
    };

    let mut my_name = String::new();

    match file.read_to_string(&mut my_name) {
        Ok(length) => {
            println!("File content: {}, File length: {}", my_name, length);
        }
        Err(err) => {
            println!("Couldn't read file: {}", err);
        }
    };

    let n = |my_name: String| {
        println!("{}", my_name);
    };

    n("Hello_world".to_string());

    let mut my_error = MyError;

    let any_value = collect_and_return_any_type(&mut my_error);

    // Err(Box::new(MyError))

    // let x = last_char_of_first_line(text)
}

pub fn read_from_file(file_name: String) -> Result<File, io::Error> {
    let v = File::open(file_name)?;
    Ok(v)
    //    {
    //         Ok(file) => {
    //             return Ok(file);
    //         },
    //         Err(err) => {
    //             return Err(err);
    //         }
    //    };
}


fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[derive(Debug, Clone)]
pub struct MyError;

impl Error for MyError {

}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is my error")
    }
}

pub trait AnytypeTrait<T> {
    fn get(&self);
    fn walk(&self);
    fn talk(&self) -> T;
}

impl AnytypeTrait<String> for MyError {

    fn get(&self) {
        todo!()
    }

    fn walk(&self) {
        todo!()
    }

    fn talk(&self) -> String {
        String::from("woof")
    }
}

impl AnytypeTrait<i32> for MyError {
    fn get(&self) {
        todo!()
    }

    fn walk(&self) {
        todo!()
    }

    fn talk(&self) -> i32 {
        5
    }
}


pub fn collect_and_return_any_type <T: AnytypeTrait<String>> (input1: &mut T) -> String {
    input1.talk()
}



