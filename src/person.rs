pub mod person {
    pub trait Person {
        fn name(&self) -> &String;
        fn age(&self) -> usize;
        fn new(name: String, age: usize) -> Self;
    }

    pub struct PersonImpl {
        name: String,
        age: usize,
    }

    impl Person for PersonImpl {
        fn name(&self) -> &String {
            &self.name
        }

        fn age(&self) -> usize {
            self.age
        }

        fn new(name: String, age: usize) -> Self {
            PersonImpl { name, age }
        }
    }
}