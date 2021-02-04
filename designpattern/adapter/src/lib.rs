use std::rc::Rc;

pub trait Target {
    fn do_work(&self) -> String {
        String::from("default work")
    }
}

pub struct Adapter {
    pub adaptee: Rc<Adaptee>,
}

impl Target for Adapter {
    fn do_work(&self) ->String {
        self.adaptee.do_real_work()
    }
}

pub struct Adaptee;
impl Adaptee {
    fn do_real_work(&self) -> String {
        String::from("real work")
    }
}

pub fn client(target: &dyn Target) ->String {
    target.do_work()
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let adapter = Adapter{
            adaptee: Rc::new(Adaptee)
        };

        let a = client(&adapter);
        assert_eq!(a, "real work");
    }
}
