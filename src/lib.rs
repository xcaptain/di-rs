// #[macro_use]
// extern crate lazy_static;
use std::collections::HashMap;

// lazy_static! {
//     static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
//         let mut map = HashMap::new();
//         map.insert("James", vec!["user", "admin"]);
//         map.insert("Jim", vec!["user"]);
//         map
//     };
// }
// pub fn show_access(name: &str) -> &Vec<&'static str> {
//     let access = PRIVILEGES.get(name).unwrap();
//     return access;
// }

pub struct Container {
    pub svcs: HashMap<&'static str, Box<dyn Injectable>>,
}

impl Container {
    fn new() -> Self {
        Container {
            svcs:  HashMap::new(),
        }
    }
}

pub trait Injectable {
    fn inject(self, c: &mut Container);
}

pub struct Service1;
impl Injectable for Service1 {
    fn inject(self, c: &mut Container) {
        c.svcs.insert("test1", Box::new(self));
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
    }
}
