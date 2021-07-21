
// Not a pattern with real tests... It is rather here to keep track of it.

struct InnerFoo {
    v: u32,
}
impl InnerFoo {
    fn new() -> Self {
        Self {
            v: 32,
        }
    }
}

pub struct Foo {
    inner: InnerFoo,
    p: u32,
}
impl Foo {
    pub fn new() -> Self {
        Self {
            inner: InnerFoo::new(),
            p: 42,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Foo;

    #[test]
    fn test_new_type() {
        let v = Foo::new();
        assert_eq!(42, v.p);
        assert_eq!(32, v.inner.v);
    }
}