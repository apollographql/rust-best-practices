#![allow(dead_code)]
use std::marker::PhantomData;

struct Unset;
struct Set;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: Option<String>,
}

struct Builder<NameState = Unset, AgeState = Unset> {
    name: Option<String>,
    age: u8,
    email: Option<String>,
    _maker_state: PhantomData<(NameState, AgeState)>,
}

impl Builder<Unset, Unset> {
    const fn new() -> Self {
        Self {
            name: None,
            age: 0,
            email: None,
            _maker_state: PhantomData,
        }
    }
}

impl<NameState> Builder<NameState, Unset> {
    fn age(self, age: u8) -> Builder<NameState, Set> {
        Builder {
            age,
            name: self.name,
            email: self.email,
            _maker_state: PhantomData,
        }
    }
}

impl<AgeState> Builder<Unset, AgeState> {
    fn name(self, name: String) -> Builder<Set, AgeState> {
        Builder {
            name: Some(name),
            age: self.age,
            email: self.email,
            _maker_state: PhantomData,
        }
    }
}

impl<NameState, AgeState> Builder<NameState, AgeState> {
    fn email(self, email: String) -> Self {
        Self {
            name: self.name,
            age: self.age,
            email: Some(email),
            _maker_state: PhantomData,
        }
    }
}

impl Builder<Set, Set> {
    fn build(self) -> Person {
        Person {
            name: self
                .name
                .unwrap_or_else(|| unreachable!("Name is guarantee to be set")),
            age: self.age,
            email: self.email,
        }
    }
}

fn main() {
    let builder = Builder::new();
    let named_builder = builder.name("name".to_string());
    let named_and_aged_builder = named_builder.age(30);

    let person = named_and_aged_builder.build();

    println!("{person:?}");
}
