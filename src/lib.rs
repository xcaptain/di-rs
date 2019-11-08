#[macro_use]
extern crate downcast_rs;
use downcast_rs::DowncastSync;

use std::collections::HashMap;

pub trait Resolvable {
    fn resolve(&self, name: &str) -> &Box<dyn Injectable>;
}

pub struct Container {
    pub svcs: HashMap<&'static str, Box<dyn Injectable>>,
}

impl Resolvable for Container {
    fn resolve(&self, name: &str) -> &Box<dyn Injectable> {
        self.svcs.get(name).unwrap()
    }
}

impl Container {
    pub fn new() -> Self {
        Container {
            svcs: HashMap::new(),
        }
    }
}

pub trait Injectable: DowncastSync {
    fn inject(self, c: &mut Container);
}
impl_downcast!(sync Injectable);

pub struct Service1;
impl Injectable for Service1 {
    fn inject(self, c: &mut Container) {
        c.svcs.insert("test1", Box::new(self));
    }
}

impl Service1 {
    pub fn hello(&self) {
        println!("hello from service1");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let mut c = Container::new();
        let t1 = Service1;
        t1.inject(&mut c);

        assert!(c.svcs.contains_key("test1"));

        let s1 = c.resolve("test1");
        s1.downcast_ref::<Service1>().unwrap().hello();
    }
}
