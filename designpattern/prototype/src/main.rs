mod lib;

use lib::FurnitureFactory;
use lib::produce;
use lib::*;
fn main() {
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

    produce(factory1);
    produce(factory2);
}