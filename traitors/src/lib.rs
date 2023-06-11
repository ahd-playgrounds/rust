#![allow(dead_code)]

trait Foo {
    fn foo(&self, arg: i32) -> i32;

    fn bar(&self) -> Box<dyn std::fmt::Display>;
}

struct F {}
impl F {
    fn bun(&self) -> &Self {
        self.foo(3);
        self.bar().to_string();
        &self
    }
}

fn combine_vecs<T>(v: Vec<T>, u: Vec<T>) -> impl Iterator<Item = T>
where
    T: Clone,
{
    v.into_iter().chain(u.into_iter()).cycle()
}

impl Foo for F {
    fn foo(&self, arg: i32) -> i32 {
        arg * arg
    }

    fn bar(&self) -> Box<dyn std::fmt::Display> {
        Box::new("hello")
    }
}

fn do_thing<T: Foo>(fooy: T, count: i32) -> i32 {
    fooy.foo(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_thing() {
        let fooer = F {};
        assert_eq!(do_thing(fooer, 8), 64)
    }

    #[test]
    fn test_chain() {
        let veca = vec![1, 2, 3, 4];
        let vecb = vec![1, 2, 3, 4];

        let vecs = combine_vecs(veca, vecb);
        assert_eq!(vecs.take(8).map(|x| x * 2).sum::<i32>(), 40);
    }
}
