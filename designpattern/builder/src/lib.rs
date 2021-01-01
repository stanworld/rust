#[derive(Debug)]
pub enum Color {
    White,
    Blue,
}

pub trait Room {
    fn get_room_color(&self) -> Color;
}

pub trait Table{
    fn get_table_color(&self) -> Color;
}

pub struct House<T: Room,U:Table> {
   pub parts:(Option<T>,Option<U>),
}

impl<T:Room,U:Table> House<T,U> {
    pub fn new(room:Option<T>,table: Option<U>) -> House<T,U>{
        House {
            parts:(room,table),
        }
    }
    pub fn list_all_parts(&self) {
       match &self.parts {
         (Some(room),Some(table)) => {
            let c1 = room.get_room_color();
            let c2 = table.get_table_color();
            println!("Create a house with room color {:?} and table color {:?}",c1,c2);
         },
         (_,_) => { println!("No parts built yet!"); }
       }
    }
}

pub struct WhiteRoom {
}

impl Room for WhiteRoom {
    fn get_room_color(&self) -> Color {
        Color::White
    }
}

pub struct WhiteTable {
}

impl Table for WhiteTable {
    fn get_table_color(&self) -> Color {
        Color::White
    }
}

pub struct BlueRoom {
}

impl Room for BlueRoom {
    fn get_room_color(&self) -> Color {
        Color::Blue
    }
}

pub struct BlueTable {
}

impl Table for BlueTable {
    fn get_table_color(&self) -> Color {
        Color::Blue
    }
}



pub trait HouseBuilder<T: Room,U:Table> {
    // steps of building
   fn build_room(&mut self);
   fn build_table(&mut self);
   // only return final product of builder
   fn build_house(&mut self)-> &House<T,U>;
}


pub struct WhiteHouseBuilder {
    house: House<WhiteRoom,WhiteTable>,
}

impl WhiteHouseBuilder {
    // default product, serve as parts holder
   pub fn new() -> WhiteHouseBuilder {
       WhiteHouseBuilder{
           house: House::new(None,None)
       }
   }
}
// generic trait becomes specific
impl HouseBuilder<WhiteRoom,WhiteTable> for WhiteHouseBuilder{
    fn build_room(&mut self){
       let x = WhiteRoom{};
       self.house.parts.0 = Some(x);
    }

    fn build_table(&mut self){
        let x = WhiteTable{};
        self.house.parts.1 = Some(x);
    }
    // only return a reference final product of builder
    fn build_house(&mut self)-> &House<WhiteRoom,WhiteTable>{
         &self.house
    }
} 

pub struct BlueHouseBuilder {
    house: House<BlueRoom,BlueTable>,
}

impl BlueHouseBuilder {
    pub fn new() -> BlueHouseBuilder {
        BlueHouseBuilder {
              house: House::new(None,None),
        }
    }
}

impl HouseBuilder<BlueRoom,BlueTable> for BlueHouseBuilder{
    fn build_room(&mut self){
        let x = BlueRoom{};
        self.house.parts.0 = Some(x);
     }
 
     fn build_table(&mut self){
         let x = BlueTable{};
         self.house.parts.1 = Some(x);
     }
     // only return a reference final product of builder
     fn build_house(&mut self)-> &House<BlueRoom,BlueTable>{
          &self.house
     }
}


pub fn director<T:Room,U:Table> (mut builder: Box<dyn HouseBuilder<T,U>>) {
    builder.build_table();
    builder.build_room();
    let house = builder.build_house();
    house.list_all_parts();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let mut builder = WhiteHouseBuilder::new();
        let house_none = builder.build_house();
        assert_eq!(house_none.parts.0.is_none(),true);
        builder.build_room();
        builder.build_table();
        let house_yes =builder.build_house();
        assert_eq!(house_yes.parts.1.is_some(),true);
    }
}
