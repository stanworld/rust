mod lib;
use lib::*;
fn main(){
    let i1 = Box::new(LeafItem);
    let i2 = Box::new(LeafItem);
    let i3: Box<dyn Item> = Box::new(LeafItem);
    assert_eq!(i1.get_number(), 1);
    let v1: Vec<Box<dyn Item>> = vec![i1,i2];
    let mut c1: Box<dyn Item> = Box::new(
        CompositeItem{
            items: v1
        }
    );
    let a=client(&c1);
    println!("final result 1: {}",a);
    let b=client(&i3);
    println!("final result 2: {}",b);
    c1.add_item(i3);
    let c=client(&c1);
    println!("final result 3: {}",c);
    c1.remove_item(0);
    let d =client(&c1);
    println!("final result 4: {}",d);
}