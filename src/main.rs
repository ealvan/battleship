use crate::table::board::Table;
use crate::items::pieces::{Piece,Direction};
use crate::point::coordinate::Point;
use crate::user::account::User;

pub mod point;
pub mod table;
pub mod items;
pub mod user;
use core::panic;
use std::convert::TryInto;
use std::io::{self, Read};
fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn create_user() -> User {
    println!("Enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Error at read line");
    // let isalphabetic = username.chars().all(|ch| ch.is_alphabetic());
    username = username.trim().to_string();
    println!("Welcome to battleship game: {username}");
    User::new(username)

}

fn main(){
    let mut t = Table::new(15,15);
    
    let mut uA = create_user();
    let mut uB = create_user();
    uA.draw_pieces(&mut t);
    uB.draw_pieces(&mut t);
    t.show();
    
    /*
    Point -> is a struct with x,y,is_active
    Table -> if the points owner
    Piece -> has points' mutable references
    User -> is the owner of pieces
    ---------------------------------------
    When you create a piece, you need to reference some points from table
    When a User draw his pieces, you inmutable references points, to know which is active or not.
*/
    //CREATE TABLE

    
    // let dir = Direction::DOWN;

    
    // if let Some(point) =  t.space.get_mut(12*t.columns as usize + 12){
    //     point.is_active = true;
    //     point.show();
    // }
    
    // let mut user = User::new("Pipo".to_string());
    // user.draw_piece(&mut t, 5);
    // user.draw_piece(&mut t, 3);
    // let mut points;
    // for piece in user.pieces{               
    //     points = match piece{
    //             Piece::CARRIER(spots) => spots,
    //             Piece::BATTLESHIP(spots) => spots,
    //             Piece::CRUISER(spots) => spots,
    //             Piece::SUBMARINE(spots) => spots,
    //             Piece::DESTROYER(spots) => spots,
    //     };
    //     for p in points{
    //         p.show();
    //     }
    // }
    
    // t.show();

    

    // if let Ok(points) = User::points_from_root_point(&t, 5){
    //     let arr = to_array::<&Point, 5>(points);
    //     let new_piece = Piece::CARRIER(arr);
    //     user.pieces.push(new_piece);
    // }
    // t.show();

}