mod entities;
use entities::person::Person;

fn main() {
    Person::new(String::from("jim"));
    println!("Hello, world!");
}
