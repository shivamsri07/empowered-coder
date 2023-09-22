
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl Person {

    pub fn new(name: String, age: usize) -> Self {
        Person { name, age }
    }

    pub fn display(&self) {
        println!("Name :: {} | Age :: {}", self.name, self.age)
    }
}
