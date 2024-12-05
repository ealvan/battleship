use crate::point::coordinate::Point;

pub mod pieces{
    use super::Point;
    pub enum Direction{
        UP,//⬆️
        DOWN,//⬇️
        LEFT,//⬅️
        RIGHT,//➡️
        UP_RIGHT,//1,1
        UP_LEFT,//1,-1
        DOWN_LEFT,//-1,-1
        DOWN_RIGHT,//-1,1
    }
    pub const CARRIER_SPOTS : usize = 5;
    pub const BATTLESHIP_SPOTS : usize = 4;
    pub const CRUISER_SPOTS : usize = 3;
    pub const SUBMARINE_SPOTS : usize = 3;
    pub const DESTROYER_SPOTS : usize = 2;

    pub enum Piece<'a>{
        CARRIER(Vec<&'a Point>),
        BATTLESHIP(Vec<&'a Point>),
        CRUISER(Vec<&'a Point>),
        SUBMARINE(Vec<&'a Point>),
        DESTROYER(Vec<&'a Point>),
    }
    impl<'a> Piece<'a>{
        pub fn get_spots(&self) -> u8{
            match self{
                Piece::CARRIER(spots) => spots.len() as u8,
                Piece::BATTLESHIP(spots) => spots.len() as u8,
                Piece::CRUISER(spots) => spots.len() as u8,
                Piece::SUBMARINE(spots) => spots.len() as u8,
                Piece::DESTROYER(spots) => spots.len() as u8,
            }
        }
        pub fn rotate(&self) -> (){

        }

    }
    impl Direction{
        pub fn get_vector(&self) -> (i8,i8){
            match self{
                Direction::DOWN => (0,-1),
                Direction::UP => (0,1),
                Direction::LEFT => (-1,0),
                Direction::RIGHT => (1,0),
                Direction::UP_RIGHT =>( 1,1),
                Direction::UP_LEFT => (1,-1),
                Direction::DOWN_LEFT => (-1,-1),
                Direction::DOWN_RIGHT => (-1,1),
            }
        }
        pub fn get_directions() -> [Direction;8]{
            [
                Direction::DOWN ,
                Direction::UP ,
                Direction::LEFT ,
                Direction::RIGHT ,
                Direction::UP_RIGHT ,
                Direction::UP_LEFT ,
                Direction::DOWN_LEFT ,
                Direction::DOWN_RIGHT 
            ]
        }
    }
}