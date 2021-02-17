pub trait Item{
  fn add_item(&mut self, _item: Box<dyn Item>) {

  }
  fn remove_item(&mut self, _index: usize) {

  }
  fn get_number(&self)->u32;
}

pub struct LeafItem;

impl Item for LeafItem {
    fn get_number(&self) ->u32 {
        let a: u32 = 1;
        a
    }
}

pub struct CompositeItem {
    pub items: Vec<Box<dyn Item>>
}
impl Item for CompositeItem {
    fn get_number(&self)->u32 {
        let mut result: u32 = 0;
        for i in &self.items {
            result = result + i.get_number();
        }
        result
    }
    fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }
    fn remove_item(&mut self,index: usize) {
        self.items.remove(index);
    }
}

pub fn client(item: &Box<dyn Item>)->u32{
    item.get_number()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let i1 = Box::new(LeafItem);
        let i2 = Box::new(LeafItem);
        let i3 = Box::new(LeafItem);
        assert_eq!(i1.get_number(), 1);
        let v1: Vec<Box<dyn Item>> = vec![i1,i2];
        let mut c1 = CompositeItem{
            items: v1
        };
        assert_eq!(c1.get_number(),2);
        c1.add_item(i3);
        assert_eq!(c1.get_number(),3);
        c1.remove_item(1);
        assert_eq!(c1.get_number(),2);
    }
}
