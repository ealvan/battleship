pub mod board{

    use crate::{
        items::pieces::{Direction,Piece}, 
        point::coordinate::Point,
        user::account::User
    };
    const tached: &str = "⛝";
    pub struct Table{
        pub rows: u8,
        pub columns: u8,
        pub space: Vec<Point>
    }
    impl Table{
        pub fn new(rows:u8,columns:u8) -> Table{
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
            let capacity = self.columns*self.rows;//TAKE CARE OF OVERFLOW
            //15*14 = 210, but you said "self.columns*self.rows) as i8". 210 as i8
            for i in 0..capacity{
                self.space.push(self.create_point_by_index(i));
            }
        }
        pub fn from_point(&self,x:i8,y:i8) -> Point{
            let p = self.get_point(x, y).unwrap();
            Point { x: p.x, y: p.y, is_active: p.is_active }
        }
        pub fn get_point(&self,x:i8,y:i8) -> Result<&Point, String>{
            let value = (x as usize*usize::from(self.columns) + y as usize);
            if value > self.space.len(){
                return Err(format!("Coordinate ({},{}) maps out of range to {} in {} spots",x,y,value,self.space.len()));
            }
            Ok(self.space.get(value).unwrap())
        }
        pub fn get_index_by_primitive_point(&self, x:i8, y:i8) -> Result<usize,String> {
            let value = (x as usize*usize::from(self.columns) + y as usize);
            if value > self.space.len(){
                return Err(format!("Coordinate ({},{}) maps out of range to {} in {} spots",x,y,value,self.space.len()));
            }
            Ok(value)
        }
        pub fn get_index_by_point(&self, point: &Point) -> Result<usize,String> {
            let value = (point.x as usize*usize::from(self.columns) + point.y as usize);
            if value > self.space.len(){
                return Err(format!("Coordinate ({},{}) maps out of range to {} in {} spots",point.x,point.y,value,self.space.len()));
            }
            Ok(value)
        }
        
        pub fn get_mut_point(&mut self, x:i8, y:i8) -> Result<&mut Point, String>{
            let value = (x as usize*usize::from(self.columns) + y as usize);
            if value > self.space.len(){
                return Err(format!("Coordinate {},{} maps out of range to {} in {} spots",x,y,value,self.space.len()));
            }
            Ok(self.space.get_mut(value).unwrap())
        }
        pub fn can_hold(&self, p: &Point) -> bool{
            p.is_valid() && p.x < self.rows as i8 && p.y < self.columns as i8
        }
        pub fn can_put(&self, p: &Point) -> bool{
            if self.can_hold(p) == false{
                return false;
            }
            let p = self.get_point(p.x,p.y).unwrap();
            p.is_active != true
        }
        pub fn get_point_by_index(&self, k:i8) -> &Point{
            let x = k/self.columns as i8;
            let y = k%self.columns as i8;
            match self.get_point(x,y){
                Ok(point) => point,
                Err(why) => panic!("This index is out of range {why}")
            }
        }
        pub fn can_be_road(&self, from: (i8,i8), spots: i8, direction: &Direction) -> Result<Vec<Point>,String>{
            let dir = direction.get_vector();            
            let points = (0..spots).map(|item| {
                let mut position = Point::new(0, 0);
                position.x = from.0 + dir.0*item;
                position.y = from.1 + dir.1*item;
                position
            }).collect::<Vec<Point>>();
            let feasible = points.iter().all(|item| self.can_put(&item));
            match feasible{
                true => Ok(points),
                false=>Err(format!("Some points in the road are not feasible to draw"))
            }
        }
        pub fn create_point_by_index(&self, k:u8) -> Point{
            let x =( k / self.columns) as i8;
            let y = (k % self.columns) as i8;
            Point::new(x,y)
        }
        pub fn get_primitive_point_by_index(&self, k: u8) -> (i8,i8) {
            let x = (k / self.columns) as i8;
            let y = (k % self.columns) as i8;
            (x,y)
        }
        pub fn change_state(&mut self,points: Vec<&Point>){
            
            for point in points{
                if let Ok(point) = self.get_mut_point(point.x,point.y){
                    point.is_active = true;
                };
            }
        }
        pub fn show(&self) -> () {
            //0..7
            //7..14
            let mut index:usize = 0;
            println!("Working to show a beautiful table");
            for _row in 0..self.rows{

                let current_row = self.space.get(
                    index..(index+self.columns as usize)).expect("Error getting the table slice");

                let repr_row = current_row.iter()
                    .fold(
                        String::new(), 
                        | acc, item| 
                        {
                            let spot = match item.is_active {
                                true => format!("▩"),
                                false => format!("⬚"),
                            };
                            format!("{acc} {spot}")
                        }
                    );
                    
                println!("{}",repr_row);
                index += self.columns as usize;
            }
        }
        pub fn show_for_user(&self, user: &User, attacked_points: &Vec<Point>) -> (){
            let pieces= &user.pieces;
            let mut str_table = vec!["⬚";(self.columns*self.rows) as usize];
            for piece in pieces{
                let points = piece.get_points();
                for point in points{
                    let index = match self.get_index_by_point(point){
                        Ok(index) => index,
                        Err(why) => panic!("{why}")
                    };
                    str_table[index] = &user.character;
                }
            }
            for at_p in attacked_points.iter(){
                let index = match self.get_index_by_point(&at_p){
                    Ok(index) => index,
                    Err(why) => panic!("{why}")
                };
                str_table[index] = tached;
            }
            self.show_table(str_table);
        }
        fn show_table(&self, table_repr: Vec<&str>) -> (){
            let mut index:usize = 0;
            for _row in 0..self.rows{
                let current_row = table_repr.get(index..(index+self.columns as usize)).expect("Error getting the table slice");
                let repr_row = current_row.iter().fold(
                    String::new(), 
                    |acc, item|{
                        format!("{acc} {item}")
                    }
                );
                println!("{}",repr_row);
                index += self.columns as usize;
            }
        }
        
        // pub fn get_space(&self){
            
        // }
    }
}