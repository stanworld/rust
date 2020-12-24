trait Table {
    fn paint(&self);
}

pub struct WhiteTable {
    pub width: u32,
    pub height: u32,
}

impl Table for WhiteTable {
    fn paint(&self) {
        println!("paint white with a size of {} by {}",self.width,self.height);
    }
}

#[cfg(test)]
mod tests {
    use crate::WhiteTable;
    use crate::Table;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let x = WhiteTable {
            width: 1,
            height:2,
        };
        x.paint();
        assert_eq!(x.height,2);
    }
}
