// product A
use downcast_rs::*;

pub trait Table: DowncastSync {
  fn paint(&self);
  fn self_clone(&self) -> Box<dyn Table>;
}

impl_downcast!(sync Table);

#[derive(Clone)]
pub struct WhiteTable {
    pub width: u32,
    pub height: u32,
    pub color: String,
}

impl Table for WhiteTable {
    fn paint(&self) {
        println!("paint {} with a size of {} by {}",self.color,self.width,self.height);
    }
    fn self_clone(&self) -> Box<dyn Table> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct YellowTable {
    pub width: u32,
    pub color: String,
    pub height: u32,
}

impl Table for YellowTable {
    fn paint(&self) {
        println!("paint {} with a size of {} {}",self.color,self.width,self.height);
    }
    fn self_clone(&self) -> Box<dyn Table> {
        Box::new(self.clone())
    }
}

//product B
pub trait Chair: DowncastSync {
    fn sing(&self);
    fn self_clone(&self) -> Box<dyn Chair>;
}

impl_downcast!(sync Chair);

#[derive(Clone)]
pub struct GoodChair {
   pub weight: f32,
   pub qoa: String,
}

impl Chair for GoodChair {
    fn sing(&self) {
        println!("Sing {} with a weight of {}", self.qoa,self.weight);
    }
    fn self_clone(&self) -> Box<dyn Chair> {
        Box::new(self.clone())
    }
}
#[derive(Clone)]
pub struct BadChair {
   pub weight: f64,
   pub  dim: (u32,u32),
}
impl Chair for BadChair {
    fn sing(&self){
        println!("Sing {} with a dim {} by {}", self.weight,self.dim.0,self.dim.1);
    }
    fn self_clone(&self) -> Box<dyn Chair> {
        Box::new(
         self.clone()
        )
    }
}

pub struct FurnitureFactory {
    pub chair: Box<dyn Chair>,
    pub table: Box<dyn Table>,
}

impl FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        self.chair.self_clone()
    }
    fn create_table(&self) -> Box<dyn Table> {
        self.table.self_clone()
    }
}


pub fn produce(factory: FurnitureFactory) {
   let table = factory.create_table();
   let chair = factory.create_chair();
   table.paint();
   chair.sing();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // prototype chairs and tables
        let x1 =  GoodChair{
            weight:50.1,
            qoa:"good".to_string(),
        };

        let y1 =  WhiteTable{
            width: 30,
            height: 40,
            color:"white".to_string(),
        };



        let x2 =  BadChair{
            weight:50.1,
            dim:(200,200),
        };

        let y2 =   YellowTable{
            width: 30,
            height: 40,
            color:"yellow".to_string(),
        };

        // more practical is to store those prototypes inside a prototype manager and access them by a key 

        let factory1: FurnitureFactory = FurnitureFactory{
            table:Box::new(y1),
            chair:Box::new(x1)
        };

        let factory2: FurnitureFactory = FurnitureFactory{
            table:Box::new(y2),
            chair:Box::new(x2)
        };

        let chair1 = factory1.create_chair();
        let table1 = factory1.create_table();
        
        let chair2 = factory2.create_chair();
        let table2 = factory2.create_table();
         


        // cast to real object from Box<dyn trait> !!!
        if let Some(chair) = chair1.downcast_ref::<GoodChair>(){
            assert_eq!(chair.weight,50.1);
        }
        else {
            assert_eq!(0,1);
        }
        if let Some(chair) = chair2.downcast_ref::<BadChair>(){
            assert_eq!(chair.dim.0,200);
        }
        else {
            assert_eq!(1,2);
        }

        let res1 = table1.downcast::<WhiteTable>();
        assert_eq!(res1.is_err(),false);

        let res2 = table2.downcast::<YellowTable>();
        match res2 {
            Ok(yellowtable) => assert_eq!(yellowtable.height,40),
            Err(_) => panic!("Casting failure : it is still a Table"),
        }
    }
}
