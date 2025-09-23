use std::any::Any;
use std::fmt::{Debug, Formatter, Pointer};

trait Pet : Debug {
    fn age(&self) -> u32;
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: u32,
}

impl Dog {
    pub fn new(name: String, age: u32) -> Dog {Dog{name, age}}
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn name(&self) -> &str {&self.name}
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u32,
}

impl Cat {
    pub fn new(name: String, age: u32) -> Cat { Cat {name, age}}
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn name(&self) -> &str {&self.name}
}


impl Pet for Dog {
    fn age(&self) -> u32 {self.age()}
}

impl Pet for Cat {
    fn age(&self) -> u32 {self.age()}
}

fn oldest(v: &Vec<Box<dyn Pet>>) -> Option<&Box<dyn Pet>> {
    v.iter().max_by_key(|pet| pet.age())
}

fn main() {
    println!("Starting");

    let cat1 = Cat {name: String::from("Miz"), age: 15};
    let dog1 = Dog::new(String::from("Tom"), 20);

    let mut v: Vec<Box<dyn Pet>> = Vec::new();
    v.push(Box::new(cat1));
    v.push(Box::new(dog1));
    
    match  oldest(&v) {
        Some(pet) => {
            println!("The oldest pet is {pet:?}");
        },
        None => println!("No pet")
    }
}
