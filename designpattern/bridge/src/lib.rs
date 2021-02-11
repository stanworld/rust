pub trait Window {
  fn do_work(&self) -> &'static str;
}

trait WindowImpl {
    fn actual_do_work(&self) ->&'static str;
}

pub struct MyWindow{
    window_impl: Box<dyn WindowImpl>
}

struct MyWindowImpl;
impl WindowImpl for MyWindowImpl {
    fn actual_do_work(&self) ->&'static str {
        "actual work"
    }
}

impl MyWindow {
    pub fn new() -> MyWindow {
        MyWindow {
            window_impl: Box::new(MyWindowImpl),
        }
    }
}


impl Window for MyWindow {
    fn do_work(&self) ->&'static str{
        self.window_impl.actual_do_work()
    }
}

pub fn client(window: Box<dyn Window>) -> &'static str{
     window.do_work()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() { 
        let result = client(Box::new(MyWindow::new()));
        assert_eq!(result,"actual work");

    }
}
