// product A
pub trait Table {
  fn paint(&self);
}

pub struct WhiteTable {
    pub width: u32,
    pub height: u32,
    pub color: String,
}

impl Table for WhiteTable {
    fn paint(&self) {
        println!("paint {} with a size of {} by {}",self.color,self.width,self.height);
    }
}

pub struct YellowTable {
    pub width: u32,
    pub color: String,
    pub height: u32,
}

impl Table for YellowTable {
    fn paint(&self) {
        println!("paint {} with a size of {} {}",self.color,self.width,self.height);
    }
}

//product B
pub trait Chair {
    fn sing(&self);
}

pub struct GoodChair {
    weight: f32,
    qoa: String,
}

impl Chair for GoodChair {
    fn sing(&self) {
        println!("Sing {} with a weight of {}", self.qoa,self.weight);
    }
}

pub struct BadChair {
    weight: f64,
    dim: (u32,u32),
}
impl Chair for BadChair {
    fn sing(&self){
        println!("Sing {} with a dim {} by {}", self.weight,self.dim.0,self.dim.1);
    }
}

pub trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair>;
    fn create_table(&self) -> Box<dyn Table>;
}

pub fn produce(factory: impl FurnitureFactory) {
   let table = factory.create_table();
   let chair = factory.create_chair();
   table.paint();
   chair.sing();
}

pub struct WhiteGoodFunitureFactory;

impl FurnitureFactory for WhiteGoodFunitureFactory {
   fn create_chair(&self) -> Box<dyn Chair> {
       Box::new(
           GoodChair{
               weight:50.1,
               qoa:"good".to_string(),
           }
       )
   }

   fn create_table(&self) -> Box<dyn Table> {
      Box::new(
          WhiteTable{
              width: 30,
              height: 40,
              color:"white".to_string(),
          }
      )

   }
}

pub struct GreenBadFunitureFactory;

impl FurnitureFactory for GreenBadFunitureFactory {
   fn create_chair(&self) -> Box<dyn Chair> {
       Box::new(
           BadChair{
               weight:50.1,
               dim:(200,200),
           }
       )
   }

   fn create_table(&self) -> Box<dyn Table> {
      Box::new(
          YellowTable{
              width: 30,
              height: 40,
              color:"yellow".to_string(),
          }
      )

   }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let x = WhiteGoodFunitureFactory;
        let y = GreenBadFunitureFactory;

        let chair1 = x.create_chair();
        let table1 = x.create_table();

        // how to cast chair1 to WhiteGood

    }
}
