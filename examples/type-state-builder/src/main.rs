#![allow(dead_code)]
use std::marker::PhantomData;

struct MissingName;
struct NameSet;
struct MissingAge;
struct AgeSet;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: Option<String>,
}

struct Builder<HasName, HasAge> {
    name: Option<String>,
    age: u8,
    email: Option<String>,
    _name_marker: PhantomData<HasName>,
    _age_marker: PhantomData<HasAge>,
}

impl Builder<MissingName, MissingAge> {
    fn new() -> Self {
        Builder {
            name: None,
            age: 0,
            _name_marker: PhantomData,
            _age_marker: PhantomData,
            email: None,
        }
    }

    fn name(self, name: String) -> Builder<NameSet, MissingAge> {
        Builder {
            name: Some(name),
            _name_marker: PhantomData::<NameSet>,
            age: self.age,
            _age_marker: PhantomData,
            email: None,
        }
    }

    fn age(self, age: u8) -> Builder<MissingName, AgeSet> {
        Builder {
            age,
            _age_marker: PhantomData::<AgeSet>,
            name: None,
            _name_marker: PhantomData,
            email: None,
        }
    }
}

impl Builder<NameSet, MissingAge> {
    fn age(self, age: u8) -> Builder<NameSet, AgeSet> {
        Builder {
            age,
            _age_marker: PhantomData::<AgeSet>,
            name: self.name,
            _name_marker: PhantomData::<NameSet>,
            email: None,
        }
    }
}

impl Builder<MissingName, AgeSet> {
    fn email(self, email: String) -> Self {
        Self {
            name: self.name,
            age: self.age,
            email: Some(email),
            _name_marker: self._name_marker,
            _age_marker: self._age_marker,
        }
    }

    fn name(self, name: String) -> Builder<NameSet, AgeSet> {
        Builder {
            name: Some(name),
            _name_marker: PhantomData::<NameSet>,
            age: self.age,
            _age_marker: PhantomData::<AgeSet>,
            email: self.email,
        }
    }
}

impl Builder<NameSet, AgeSet> {
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
