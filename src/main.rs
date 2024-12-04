use crate::table::board::Table;
use crate::items::pieces::{Piece,Direction};
use crate::point::coordinate::Point;
use crate::user::account::User;
pub mod point;
pub mod table;
pub mod items;
pub mod user;

fn main(){
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
    let mut t = Table::new(15,14);
    
    t.show();
    
    // let dir = Direction::DOWN;

    
    if let Some(point) =  t.space.get_mut(12*t.columns as usize + 12){
        point.is_active = true;
        point.show();
    }

    if let Ok(point) = t.get_point(12, 12){
        point.show();
    };
    let points = User::draw_from_root_point(&t, 4);
    for p in points {
        p.show()
    }
    t.show();

    println!("AAAAAAAA: {}", 210u8 as i8);
}