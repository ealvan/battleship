use crate::items::pieces::{Piece,Direction};
use crate::table::board::Table;
use crate::point::coordinate::Point;

pub mod account{
    use core::panic;
    use crate::items::pieces::{get_pieces_spots};

    use super::Point;

    use super::{Piece,Direction};
    use super::Table;
    use rand::Rng;
    pub struct User{
        pub name: String,
        pub my_turn: bool,
        pub pieces: Vec<Piece>,
        pub n_lives: u8,//the number spots of your pieces are your lives
        pub give_up: bool,
        pub character: String,
    }
    impl User{
        //maybe add an array with how many pieces, by default would be 1
        pub fn new(name: String) ->  User{
            // let lives = pieces.iter().map(|piece| piece.get_spots()).sum();
            User{
                n_lives: get_pieces_spots().iter().sum::<i8>() as u8,
                name,
                pieces : vec![],
                my_turn:false,
                give_up:false,
                character: String::from("▤"),
            }
        }
        pub fn new_with_char(name: String, character: String) ->  User{
            // let lives = pieces.iter().map(|piece| piece.get_spots()).sum();
            User{
                n_lives: get_pieces_spots().iter().sum::<i8>() as u8,
                name,
                pieces : vec![],
                my_turn:false,
                give_up:false,
                character,
            }
        }
        
        pub fn draw_pieces(&mut self, t: &mut Table) {
            let spots = get_pieces_spots();
            for n in spots{
                self.draw_piece(t, n);
            }
            let labels = self.pieces.iter().map(|piece| piece.get_label()).collect::<Vec<String>>();
            let pieces_str = labels.join(",");
            println!("{}, your pieces are:\n{}", self.name, pieces_str);
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
        pub fn draw_piece(& mut self, table: & mut Table, n: i8) {
            // println!("Outside draw piece");
            match User::points_from_root_point(table,n) {
                Ok(vec_points) => {
                    // println!("Inside draw piece");
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

        pub fn points_from_root_point(table: & mut Table, spots: i8) -> Result<Vec<Point>, String>{
            let directions = Direction::get_directions();
            let opportunities = 10;
            for _time in 0..opportunities{
                let free_spot = match User::find_free_point(table){
                    Some(point) => point,
                    None => panic!("There are any free spots")
                };
                for direction in directions.iter(){
                    match table.can_be_road((free_spot.x, free_spot.y), spots, direction){
                        Ok(points) => {
                            table.change_state(points.iter().collect());
                            return Ok(points);
                        },
                        Err(_) => continue,
                    };
                }  
            }
            return Err(format!("Any directions were sufficient to draw from root point"));
        }
        fn check_hit(&self, point: &Point) -> bool{
            //just check if I hit myself
            for piece in self.pieces.iter(){
                for p in piece.get_points(){
                    if point.is_equal(p){
                        return true;
                    }

                }
            }
            false
        }
        pub fn attack(&mut self, table: &mut Table, point: &Point,attacked_points: &mut Vec<Point>, enemy: &mut User){
            if table.can_hold(point){
                println!("Attacking...⛝ coord: {}",point.repr());

                let point = match table.get_mut_point(point.x, point.y){
                    Ok(val) => val,
                    Err(why) => panic!("{why}")
                };
                if point.is_active{
                    //I hit a piece is it mine or the enemy haha
                    if self.check_hit(point){
                        if self.n_lives == 0{
                            println!("Your enemy {} wins!!", enemy.name);
                            return;
                        }else{
                            println!("You hit on the foot");
                            self.n_lives = self.n_lives -1;
                            let n = Point::from(point);
                            attacked_points.push(n);
                        }
                    }
                    for p in attacked_points.iter(){
                        if point.is_equal(p){
                            println!("You hit an already hitted spot");
                        }
                    }
                    if enemy.check_hit(point){
                        if self.n_lives == 0{
                            println!("Your enemy {} wins!!", self.name);
                            return;
                        }else{
                            println!("You have nailed!");
                            enemy.n_lives = enemy.n_lives -1;
                            let n = Point::from(point);
                            attacked_points.push(n);
                        }
                    }
                }else{
                    println!("You hit nothing!");
                    point.is_active = true;
                    let p = Point::from(point);
                    attacked_points.push(p);
                }
            }else{
                println!("The point cant be in the board!");
            }
        }
        pub fn prompt_lives(&self){
            println!("You have {} lives left",self.n_lives);
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