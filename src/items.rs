pub mod pieces{
    pub enum Direction{
        UP,//⬆️
        DOWN,//⬇️
        LEFT,//⬅️
        RIGHT//➡️
    }
    pub enum Piece{
        CARRIER,
        BATTLESHIP,
        CRUISER,
        SUBMARINE,
        DESTROYER
    }
    impl Piece{
        pub fn get_spots(&self) -> i8{
            match self{
                Piece::CARRIER => 5,
                Piece::BATTLESHIP => 4,
                Piece::CRUISER => 3,
                Piece::SUBMARINE => 3,
                Piece::DESTROYER => 2,
            }
        }
        pub fn rotate(&self) -> (){

        }

    }
}