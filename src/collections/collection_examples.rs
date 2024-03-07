use std::{
    collections::{BTreeMap, HashMap},
    hash::BuildHasher,
};

pub fn main() {
    // println!("Collection exxamples");

    // let array : [usize; 5] = [2; 5];

    // let mut vector = vec![5];
    // let vec2: Vec<&str> = Vec::new();

    // array.to_vec().pop();
    // vector.pop();
    // vector.push(6);

    // let x = vector.get(0).unwrap_or(&6);

    // vector.first();

    // let name = String::from("Timi");

    // let second_char = name.as_bytes().get(90).unwrap_or(&0);

    // println!("The second character of timi is {}", second_char);

    // println!("The valiue of x is: {}", x);

    let mut new_vec = vec![4, 5, 6, 2, 1];

    // new_vec.reverse();

    for ele in &mut new_vec {
        *ele += 50;
        println!("{ele}");
    }

    // println!("Second iteration");

    // for i in 0..new_vec.len() {
    //     println!("{}", new_vec[i]);
    // }

    let mut sch = web3bridge { total_students: 9 };

    update_total_students_inplace(&mut sch, 11);

    println!("After first mutation: {:?}", sch);

    let v = update_total_students_dont_consume(&mut sch, 20);

    println!("The value of V: {:?}", v);

    println!("After second mutation: {:?}", sch);

    let n = update_total_students_and_consume(sch, 15);

    println!("After mutate and consume: {:?}", n);

    let people: Vec<Gender> = vec![Gender::Male(6), Gender::Female(5)];

    let new_string = String::new();

    let n = String::from("A");

    let m = "A".to_string();

    let o = n + &m;

    println!("Here is o: {o}");

    let hello = String::from("Здравствуйте");

    let hello_str = "Здравствуйте";

    println!("{} and {}", hello.len(), hello_str.len());

    // for (index, char) in hello_str.as_bytes().iter().enumerate() {
    //     println!("At position index: {index}, the character is {char}");
    // }

    let mut scores_map: BTreeMap<&str, usize> = BTreeMap::new();

    scores_map.insert("Arsenal", 6);
    scores_map.insert("Chelsea", 10);

    for club in scores_map.values() {
        println!("{club}");
    }

    if let Some(g) = scores_map.get("Arsenal") {
        println!("{g}");
    }

    let m = scores_map.entry("Man U").or_insert(0);
    *m += 1;

    // println!("{m}");
    // *m = 8;

    println!("Up Man U: {}", scores_map.get("Man U").unwrap());
}

// struct Myhasher;

// impl BuildHasher for Myhasher{
//     type Hasher;

//     fn build_hasher(&self) -> Self::Hasher {
//         todo!()
//     }
// }

pub struct Place {
    x: u64,
    y: String,
}

pub struct Point {
    k: String,
    h: f64,
}

pub struct Air {
    h: Vec<Place>,
    j: Vec<Point>,
}

pub enum Gender {
    Male(i32),
    Female(u32),
}

#[derive(Debug, Clone)]
struct web3bridge {
    total_students: usize,
}

pub fn update_total_students_inplace(school: &mut web3bridge, new_total: usize) {
    school.total_students = new_total;
}

pub fn update_total_students_and_consume(mut school: web3bridge, new_total: usize) -> web3bridge {
    school.total_students = new_total;
    school
}

pub fn update_total_students_dont_consume(school: &mut web3bridge, new_total: usize) -> web3bridge {
    let mut m = school.clone();
    m.total_students = new_total;
    m
}
