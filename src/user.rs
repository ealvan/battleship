use crate::items::pieces::Piece;
use crate::table::board::Table;
use crate::point::coordinate::Point;
pub mod account{
    use super::Piece;
    use super::Table;
    use super::Point;
    
    pub struct User{
        pub name: String,
        pub my_turn: bool,
        pub pieces: Vec<Piece>,
        pub n_lives: i8,//the number spots of your pieces are your lives
        pub give_up: bool
    }
    impl User{
        pub fn new(name: String, pieces:Vec<Piece>) -> User{
            let lives = pieces.iter().map(|piece| piece.get_spots()).sum();
            User{
                n_lives: lives,
                name,
                pieces,
                my_turn:false,
                give_up:false
            }
        }
        pub fn attack(&self, table: &Table, point: &Point){
            if table.can_hold(point){
                println!("Attacking...");
            }else{
                println!("The point cant be in the board!");
            }
        }
        pub fn get_lives(&self) -> i8{
            self.n_lives
        }
        pub fn get_turn(&self) -> bool{
            self.my_turn
        }
        pub fn show(&self) -> (){
            let data = format!("name: {}\ngive_up: {}\nmyturn:{}\nlives:{}",self.name,self.give_up, self.my_turn,self.get_lives());
            println!("{data}");
        }

    }
}