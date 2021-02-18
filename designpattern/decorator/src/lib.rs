use std::rc::Rc;

pub trait Component {
    fn do_work(&self)->u32;
}

pub struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn do_work(&self)->u32 {
        println!("do concrete work.");
        10
    }
}

pub trait Decorator: Component {
  fn new(component: Rc<dyn Component>) -> Self;
}

pub struct ConcreteDecorator {
    component: Rc<dyn Component>,
}

impl Decorator for ConcreteDecorator {
    fn new(component: Rc<dyn Component>) -> Self{
        ConcreteDecorator{component}
    }
}

impl Component for ConcreteDecorator {
    fn do_work(&self)->u32 {
        self.my_decoration()+self.component.do_work()
        
    }
}
impl ConcreteDecorator {
    fn my_decoration(&self)->u32 {
        println!("this is concrete decoration");
        11
    }
}

pub fn client(component: &impl Component){
    let a =component.do_work();
    println!("result: {}",a);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let a = ConcreteComponent;
        assert_eq!(a.do_work(),10);
        let b = ConcreteDecorator::new(Rc::new(a));
        assert_eq!(b.do_work(),21);
    }
}
