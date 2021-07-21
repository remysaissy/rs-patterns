
#[derive(Debug, PartialEq)]
pub struct Foo {
    name: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    name: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        Self {
            name: String::from(""),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name.push_str(name);
        self

    }

    pub fn build(self) -> Foo {
        Foo {
            name: self.name,
        }
    }
}

#[cfg(test)]
mod test {

    use super::Foo;

    #[test]
    fn test_builder() {
        let res = Foo {
            name: String::from("foobar"),
        };
        let v = Foo::builder()
                    .with_name("foobar")
                    .build();
        assert_eq!(res, v);
    }
}