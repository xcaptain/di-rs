#[macro_use]
extern crate downcast_rs;
use downcast_rs::DowncastSync;

use std::any::{Any, TypeId};
use std::collections::HashMap;

use di_macros::Injectable;

pub trait Resolvable {
    fn resolve<T: Injectable>(&self) -> Option<&T>;
    // TODO: use impl Injectable or Box<dyn Injectable>?
    fn singleton(&mut self, obj: impl Injectable);
    fn scoped(&mut self, obj: impl Injectable);
}

#[derive(Default)]
pub struct Container {
    // TODO: TypeId changes as the state of the struct changes
    // should we use a struct as service name or keep up TypeId?
    pub svcs: HashMap<TypeId, Box<dyn Injectable>>,
}

impl Resolvable for Container {
    fn resolve<T: Injectable>(&self) -> Option<&T> {
        for (_key, value) in self.svcs.iter() {
            if value.is::<T>() {
                return value.downcast_ref::<T>();
            }
        }
        return None;
    }

    /// A singleton bind only occour once
    fn singleton(&mut self, obj: impl Injectable) {
        assert!(!self.svcs.contains_key(&obj.type_id()));
        self.svcs.insert(obj.type_id(), Box::new(obj));
    }

    fn scoped(&mut self, obj: impl Injectable) {
        self.svcs.insert(obj.type_id(), Box::new(obj));
    }
}

impl Container {
    pub fn run(&self) -> usize {
        // how to run the container
        self.resolve::<Service1>().unwrap().run_service1() + self.resolve::<Service2>().unwrap().run_service2()
    }
}

pub trait Injectable: DowncastSync + Any {
    fn inject(self, c: &mut Container);
}
impl_downcast!(sync Injectable);

#[derive(Injectable)]
pub struct Service1;

impl Service1 {
    pub fn run_service1(&self) -> usize {
        println!("hello from service1, {:?}", self.type_id());
        1
    }
}

#[derive(Injectable)]
pub struct Service2;

impl Service2 {
    pub fn run_service2(&self) -> usize {
        println!("hello from service2, {:?}", self.type_id());
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let mut c = Container::default();

        // use 2 ways to inject service into container
        Service1.inject(&mut c);
        c.singleton(Service2);

        assert_eq!(3, c.run());
    }
}
