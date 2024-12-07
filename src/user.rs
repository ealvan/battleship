use crate::items::pieces::{Piece,Direction};
use crate::table::board::Table;
use crate::point::coordinate::Point;

pub mod account{
    use core::panic;
    use super::Point;

    use super::{Piece,Direction};
    use super::Table;
    use rand::Rng;
    pub struct User{
        pub name: String,
        pub my_turn: bool,
        pub pieces: Vec<Piece>,
        pub n_lives: u8,//the number spots of your pieces are your lives
        pub give_up: bool
    }
    impl User{
        //maybe add an array with how many pieces, by default would be 1
        pub fn new(name: String) ->  User{
            // let lives = pieces.iter().map(|piece| piece.get_spots()).sum();
            User{
                n_lives: 0,
                name,
                pieces : vec![],
                my_turn:false,
                give_up:false
            }
        }
        pub fn find_free_point(table : & Table) -> Option<& Point>{
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
        pub fn points_from_root_point(table: & mut Table, spots: i8) -> Result<Vec<Point>, String>{
            //just returns an array with the spots
            let directions = Direction::get_directions();
            let mut primitive_points = vec![];
            //the free pointcan be a parameter
            let free_spot = match User::find_free_point(table){
                Some(point) => point,
                None => panic!("There are any spots free")
            };
            let mut next_position = (free_spot.x, free_spot.y);
            let mut flag = false;
            for direction in directions{
                let vec = direction.get_vector();
                print!("Free spot: "); free_spot.show();
                let end_position = (free_spot.x + vec.0*(spots-1), free_spot.y + vec.1*(spots-1));
                let end_point = Point::new(end_position.0 ,end_position.1);
                if table.can_hold(&end_point) == true {
                    primitive_points.push(next_position);
                    for _time in 0..spots-1{
                        let next_point = (next_position.0 + vec.0, next_position.1 + vec.1);
                        primitive_points.push(next_point);
                        next_position = next_point;
                    }
                    break;
                }else{
                    flag = true;
                }
            }
            if flag{
                return Err(format!("Any directions were sufficient to draw from root point"));
            }
            let points = primitive_points.iter().map(|item|  Point{x:item.0,y:item.1, is_active:true}).collect::<Vec::<Point>>();
            println!("gigigigig");
            println!("{:?}", primitive_points);

            table.change_state(primitive_points);
            // for priv_point in primitive_points{
            //     match table.get_mut_point(priv_point.0, priv_point.1){
            //         Ok(point) => {
            //             point.is_active = true;
            //         },
            //         Err(why) => panic!("{why}") 
            //     };
            // }
                // let new_position = (free_spot.x as i8 + vec.0, free_spot.y as i8 + vec.1);
                // let new_point = Point::new(new_position.0,new_position.1);
            return Ok(points);
        }
        pub fn attack(&self, table: &Table, point: &Point){
            if table.can_hold(point){
                println!("Attacking...");
            }else{
                println!("The point cant be in the board!");
            }
        }
        pub fn draw_piece(& mut self, table: & mut Table, n: i8) {
            println!("Outside draw piece");
            match User::points_from_root_point(table,n) {
                Ok(vec_points) => {
                    println!("Inside draw piece");
                    let piece = match vec_points.len() {
                        4 => Piece::BATTLESHIP(vec_points),
                        5 => Piece::CARRIER(vec_points),
                        3 => Piece::CRUISER(vec_points),
                        2 => Piece::SUBMARINE(vec_points),
                        _ => panic!("Not available pieces: {}",vec_points.len())
                    };
                    self.pieces.push(piece);
                },
                Err(why) => panic!("{why}")
            }
            // let x_root_index = rand::thread_rng().gen_range(0..table.columns);
            // let y_root_index = rand::thread_rng().gen_range(0..table.rows);
            // let spots = p.get_spots();
            // println!("Working to draw a piece on table");
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

pub mod utils{
    use std::convert::TryInto;

    pub fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
        v.try_into()
            .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
    }
}