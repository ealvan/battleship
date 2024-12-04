// use crate::table::board::Table;
// use crate::items::pieces::Piece;
// use crate::point::coordinate::Point;
// use crate::user::account::User;
// pub mod point;
// pub mod table;
// pub mod items;
// pub mod user;
struct Point{
    x:i8,
    y:i8,
    is_active: bool
}
impl Point{
    pub fn new(x:i8,y:i8) -> Point{
        Point{
            x,
            y,
            is_active:false
        }
    }
    pub fn show(&self){
        println!("x: {}, y: {} is_active: {}",self.x,self.y,self.is_active);
    } 
}
enum Piece<'a>{
    A([&'a mut Point;5]),
    B([&'a mut Point;3]),
    C([&'a mut Point;4])
}
impl <'a> Piece<'a>{
    fn show(&self) -> (){
        match self {
            Piece::A(points) => Piece::print(points),
            Piece::B(points) => Piece::print(points),
            Piece::C(points) => Piece::print(points),
        }
    }
    fn print(ps : &[&mut Point]){
        println!("AVER......");
        println!("{}",ps.len());
    }
}
fn main() {
    let mut a = Point::new(0, 0);
    let mut b = Point::new(0, 0);
    let mut c = Point::new(0, 0);
    let mut d = Point::new(0, 0);
    let mut e = Point::new(0, 0);

    let p = Piece::A([&mut a,&mut b,&mut c,&mut d,&mut e]);
    p.show();
    
    let m = Piece::B([&mut a,&mut b,&mut c]);
    m.show();
    // let p = Point{
    //     x:0,y:0,is_active:false
    // };
    // let p1 = Point{
    //     x:0,y:0,is_active:false
    // };
    // let arr = [&p,&p1];
    // let rp = arr[0];
    // rp.show();

    // let a = Piece::B;
    // match a {
    //     Piece::A => println!("A"),
    //     Piece::B => println!("B"),
    //     Piece::C => println!("C"),
    // }
    // println!("Hello, world!");
    // let pieces = vec![
    //     Piece::CARRIER,
    //     Piece::BATTLESHIP,
    //     Piece::CRUISER,
    //     Piece::SUBMARINE,
    //     Piece::DESTROYER
    // ];
    // let user = User::new(
    //     String::from("mattvid"), 
    //     pieces
    // );
    // user.show();
    // let table = Table::new(8,12);
    // table.show();
    // let a = table.get_point(0,0);
    // a.show();
    // let range = table.space.get(0..2).expect("Your range cant be reached in this table");
    // for p in range{
    //     p.show();
    // }

}
