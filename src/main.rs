use crate::table::board::Table;
use crate::items::pieces::Piece;
use crate::point::coordinate::Point;
use crate::user::account::User;
pub mod point;
pub mod table;
pub mod items;
pub mod user;
fn main() {
    println!("Hello, world!");
    let pieces = vec![
        Piece::CARRIER,
        Piece::BATTLESHIP,
        Piece::CRUISER,
        Piece::SUBMARINE,
        Piece::DESTROYER
    ];
    let user = User::new(
        String::from("mattvid"), 
        pieces
    );
    user.show();
    let table = Table::new(8,12);
    table.show();
    // let a = table.get_point(0,0);
    // a.show();
    // let range = table.space.get(0..2).expect("Your range cant be reached in this table");
    // for p in range{
    //     p.show();
    // }

}
