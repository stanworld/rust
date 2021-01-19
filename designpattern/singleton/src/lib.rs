// the biggest chanllenges for singleton pattern is to ensure thread-safety

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct SingletonItem {
  pub p1: u32,
}

pub fn get_singleton_item() -> &'static Lazy<Mutex<SingletonItem>> {
    static GLOBAL_DATA: Lazy<Mutex<SingletonItem>> = Lazy::new(|| {
        let m = SingletonItem {
            p1: 10, 
        };

        Mutex::new(m)
    });
    &GLOBAL_DATA
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
       {
         let mut a1 = get_singleton_item().lock().unwrap();
         assert_eq!(a1.p1,10);
         a1.p1 = 11;
         assert_eq!(a1.p1,11);
         // use a1 inside a scope, such that it is unlocked automatically and b can lock it again
       }
       let b = get_singleton_item().lock().unwrap();
       assert_eq!(b.p1,11);
    }
}
