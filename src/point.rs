use crate::items::pieces::Direction;
pub mod coordinate{
    use super::Direction;
    pub struct Point{
        pub x: i8,
        pub y:i8,
        pub is_active: bool
    }
    impl Point{
        pub fn new(x:i8,y:i8) -> Point{
            Point{
                x,
                y,
                is_active:false
            }
        }
        pub fn from(p: &Point) -> Point{
            Point{
                x:p.x,
                y:p.y,
                is_active:false
            }
        }
        pub fn on_board(&self) -> bool{
            false
        }
        pub fn is_valid(&self) -> bool{
            self.x >= 0 && self.y >= 0
        }
        pub fn show(&self){
            println!("x: {}, y: {} is_active: {}",self.x,self.y,self.is_active);
        }
        pub fn repr(&self) -> String {
            format!("({},{})", self.x, self.y)
        }
        pub fn get_position(&self) -> (i8,i8){
            (self.x,self.y)
        }
        pub fn get_next(&self,dir: &Direction) -> Point{
            let vector = dir.get_vector();
            Point{
                x: self.x - vector.0,
                y: self.y + vector.1,
                is_active: false
            }
        }
        pub fn is_equal(&self, point: &Point) -> bool{
            point.x == self.x && point.y == self.y
        }
    }
}