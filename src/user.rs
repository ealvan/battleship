use crate::items::pieces::{Piece,Direction};
use crate::table::board::Table;
use crate::point::coordinate::Point;
pub mod account{
    use core::panic;

    use super::{Piece,Direction};
    use super::Table;
    use super::Point;
    use rand::Rng;
    pub struct User<'a>{
        pub name: String,
        pub my_turn: bool,
        pub pieces: Vec<Piece<'a>>,
        pub n_lives: u8,//the number spots of your pieces are your lives
        pub give_up: bool
    }
    impl <'a> User<'a>{
        //maybe add an array with how many pieces, by default would be 1
        pub fn new<'b>(name: String) ->  User<'b>{
            // let lives = pieces.iter().map(|piece| piece.get_spots()).sum();
            User{
                n_lives: 0,
                name,
                pieces : vec![],
                my_turn:false,
                give_up:false
            }
        }
        pub fn find_free_point<'b>(table : &'b Table) -> Option<&'b Point>{
            let x_root_index = rand::thread_rng().gen_range(0..table.columns);
            let y_root_index = rand::thread_rng().gen_range(0..table.rows);
            //add a table condition if all spots are active            
            let free_spots = table.space.iter().filter(|p| p.is_active == false).count();
            if free_spots == 0{
                return  None;
            }
            let free_spots = table.space.iter().filter(|p| p.is_active == false);
            let free_spots: Vec<&Point> = free_spots.collect();    
            let random_spot = rand::thread_rng().gen_range(0..free_spots.len());
            Some(free_spots.get(random_spot).unwrap())
        }
        pub fn draw_from_root_point(table: &'a Table, spots: i8) -> Vec<&'a Point>{
            //just returns an array with the spots
            let directions = Direction::get_directions();
                if let Some(free_spot) = User::find_free_point(table){
                    for direction in directions{
                        let vec = direction.get_vector();
                        print!("Free spot: "); free_spot.show();
                        let end_position = (free_spot.x + vec.0*(spots-1), free_spot.y + vec.1*(spots-1));
                        let end_point = Point::new(end_position.0 ,end_position.1);
                        if table.can_hold(&end_point) == true {
                            let mut points: Vec<&Point> = Vec::new();
                            points.push(free_spot);
                            let mut next_position = (free_spot.x, free_spot.y);
                            for time in 0..spots-1{
                                let next_point = (next_position.0 + vec.0, next_position.1 + vec.1);
                                let ref_point = match table.get_point(next_point.0, next_point.1){
                                    Ok(res) => res,
                                    Err(why) => panic!("{why}"),                                
                                };
                                points.push(ref_point);
                                next_position = next_point;
                            }
                            return points;
                        }else{
                            println!("Endpoint ({},{}), cant hold in table",end_point.x,end_point.y)
                        }
                }
                    // let new_position = (free_spot.x as i8 + vec.0, free_spot.y as i8 + vec.1);
                    // let new_point = Point::new(new_position.0,new_position.1);
                }else{
                    panic!("There are any free spots to draw");
                }   
            panic!("It could figure out a new position");
        }
        pub fn attack(&self, table: &Table, point: &Point){
            if table.can_hold(point){
                println!("Attacking...");
            }else{
                println!("The point cant be in the board!");
            }
        }
        pub fn draw_piece(&self, p: &Piece, table: &Table) {
            let x_root_index = rand::thread_rng().gen_range(0..table.columns);
            let y_root_index = rand::thread_rng().gen_range(0..table.rows);
            let spots = p.get_spots();
            println!("Working to draw a piece on table");
        }
        
        pub fn get_lives(&self) -> u8{
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