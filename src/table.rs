pub mod board{
    use core::error;

    use crate::point::coordinate::Point;
    pub struct Table{
        pub rows: i8,
        pub columns: i8,
        pub space: Vec<Point>
    }
    impl Table{
        pub fn new(rows:i8,columns:i8) -> Table{
            let mut tmp = Table{
                rows,
                columns,
                space: Vec::with_capacity((rows as usize) * (columns as usize))
            };
            tmp.initialize_space();
            tmp
        }
        pub fn initialize_space(&mut self) -> (){
            //I think this must be private and use at new()
            //and static function(general function)
            let capacity = self.columns*self.rows;
            for i in 0..capacity{
                self.space.push(Point::new(0,0));
            }
        }
        pub fn from_point(self,x:i8,y:i8) -> Point{
            let p = self.get_point(x, y);
            Point { x: p.x, y: p.y, is_active: p.is_active }
        }
        pub fn get_point(&self,x:i8,y:i8) -> &Point{
            let value = (x*self.columns + y) as usize;
            match self.space.get(value){
                Some(point) => point,
                None => panic!("Point in x: {}, y: {}, doesnt exists at table with rs:{},cs:{}!",x,y,self.rows,self.columns)
            }
        }
        pub fn can_hold(&self, p: &Point) -> bool{
            p.is_valid() && p.x < self.rows && p.y < self.columns
        }
        // pub fn get_space(&self){
            
        // }
    }
}