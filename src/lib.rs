#[macro_use]
extern crate downcast_rs;
use downcast_rs::DowncastSync;

use std::any::Any;
use std::collections::HashMap;

pub trait Resolvable {
    fn resolve(&self, name: &str) -> &Box<dyn Injectable>;
}

#[derive(Default)]
pub struct Container {
    pub svcs: HashMap<&'static str, Box<dyn Injectable>>,
}

impl Resolvable for Container {
    fn resolve(&self, name: &str) -> &Box<dyn Injectable> {
        self.svcs.get(name).unwrap()
    }
}

pub trait Injectable: DowncastSync + Any {
    fn inject(self, c: &mut Container);
}
impl_downcast!(sync Injectable);

pub struct Service1;
impl Injectable for Service1 {
    fn inject(self, c: &mut Container) {
        c.svcs.insert("service1", Box::new(self));
    }
}

impl Service1 {
    pub fn run_service1(&self) {
        // 6719967427312869732
        println!("hello from service1, {:?}", self.type_id());
    }
}

pub struct Service2;
impl Injectable for Service2 {
    fn inject(self, c: &mut Container) {
        c.svcs.insert("service2", Box::new(self));
    }
}

impl Service2 {
    pub fn run_service2(&self) {
        // 702520684108117355
        println!("hello from service2, {:?}", self.type_id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let mut c = Container::default();
        Service1.inject(&mut c);
        Service2.inject(&mut c);

        assert!(c.svcs.contains_key("service1"));
        assert!(c.svcs.contains_key("service2"));

        c.resolve("service1")
            .downcast_ref::<Service1>()
            .unwrap()
            .run_service1();
        c.resolve("service2")
            .downcast_ref::<Service2>()
            .unwrap()
            .run_service2();
    }
}
