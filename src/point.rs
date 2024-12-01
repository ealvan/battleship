pub mod coordinate{
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
                is_active:true
            }
        }
        pub fn from() -> Point{
            Point{
                x:0,
                y:0,
                is_active:true
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
    }
}