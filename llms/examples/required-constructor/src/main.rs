struct Person {
    name: String,
    age: u8,
    email: Option<String>,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
            email: None,
        }
    }

    fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}

fn main() {
    let person = Person::new("Ada".to_owned(), 30).with_email("ada@example.com".to_owned());
    println!("{} ({})", person.name, person.age);
    if let Some(email) = person.email {
        println!("{email}");
    }
}
