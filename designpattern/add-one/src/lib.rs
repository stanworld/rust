pub fn add_one(x: i32) ->i32 {
   // let mut rng = rand::thread_rng();
  //  let y =rng.gen::<i32>();
    x  +1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}