pub mod board{

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
                self.space.push(self.create_point_by_index(i));
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
        pub fn get_point_by_index(&self, k:i8) -> &Point{
            let x = k/self.columns;
            let y = k%self.columns;
            self.get_point(x,y)
        }
        pub fn create_point_by_index(&self, k:i8) -> Point{
            let x = k / self.columns;
            let y = k % self.columns;
            Point::new(x,y)
        }
        pub fn show(&self) -> () {
            //0..7
            //7..14
            let mut index = 0;
            println!("Working to show a beautiful table");
            for _row in 0..self.rows{
                let current_row = self.space.get(index..index+usize::from(self.columns as u8)).expect("Error getting the table slice");
                let repr_row = current_row
                    .iter()
                    .fold(
                        String::new(), 
                        | acc, item| 
                        format!("{}..{}",acc, item.repr())
                    );
                println!("{}",repr_row);
                index += self.columns as usize;
            }
        }
        // pub fn get_space(&self){
            
        // }
    }
}