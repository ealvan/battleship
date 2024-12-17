use point::coordinate::Point;

use crate::table::board::Table;

use crate::user::account::User;

pub mod point;
pub mod table;
pub mod items;
pub mod user;
use core::panic;
use std::convert::TryInto;
use std::io::{self, Read};
use std::vec;
fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn create_user() -> User {
    println!("Enter username:");
    let mut username = String::new();
    let mut option: String = String::new();
    io::stdin().read_line(&mut username).expect("Error at read line");
    // let isalphabetic = username.chars().all(|ch| ch.is_alphabetic());
    username = username.trim().to_string();
    println!("Welcome to battleship game: {username}");
    println!("Please select your piece type:");
    println!("(1) Key: ⚿\n(2) Coffee: ⛾\n(3)Cube: ❒");
    println!("Option (1,2 or 3): ");

    io::stdin().read_line(&mut option).expect("Cannot read your option");
    option = option.trim().to_string();
    let character = match option.as_str(){
        "1" => "⚿",
        "2" => "⛾",
        "3" => "❒",
        _ => panic!("Battleship game doesnt support that option")
    };    
    User::new_with_char(username, character.to_string())
    // User::new(username)

}
fn promp_attack(user: &mut User) -> Result<(i32,i32), String>{
    println!("Dear {}, Where you will attack? If you give up, please enter g/G",user.name);
    let mut line = String::new();

    loop{
        user.prompt_lives();
        io::stdin().read_line(&mut line).expect("Couldnt read your input, please try again");
        if line.trim().to_uppercase() == "G"{
            user.give_up = true;
            return Err(format!("You have give up, logging out the game..."));            
        }
        let values = line.trim().split(",").map(|item| match item.parse::<i32>(){
            Ok(item) => item,
            Err(_) => panic!("You dont have provided the correct syntax (x,y)"),
        }).collect::<Vec<i32>>();
        println!("{}", values.len());
        match values.len(){
            2 => {
                let x = values.get(0).expect("This index doesnt work").clone();
                let y = values.get(1).expect("This index doesnt work").clone();
                return Ok((x,y));
            },
            _ => continue
        }
    };
}

fn promp_winner(user_A:&User, user_B:&User) -> bool{
    
    if user_A.n_lives == 0{
        println!("Your enemy {} has won!",user_B.name);
        return true;
    }else if user_B.n_lives == 0{
        println!("Your enemy {} has won!",user_A.name);        
        return true;
    }
    false
}

fn show_points(user_A:&User,user_B:&User){
    println!("{} - score: {}", user_A.name, user_A.n_lives);
    println!("{} - score: {}", user_B.name, user_B.n_lives);
}

fn main(){
    let mut attacked_points: Vec<Point> = vec![];
    let mut t = Table::new(15,15);
    println!("=============FIRST USER =========");
    let mut u_A = create_user();
    println!("=============SECOND USER =========");
    let mut u_B = create_user();
    u_A.draw_pieces(&mut t);
    u_B.draw_pieces(&mut t);

    println!("\n=================================");
    println!("LET'S BEGIN THE BATTLESHIP GAME");
    println!("=================================");

    //let's decide which first:
    u_A.my_turn = !u_B.my_turn;
    loop {
        if promp_winner(&u_A, &u_B){
            break;
        }
        if u_A.my_turn{
            println!("Dear {}-{}, your pieces are: ", u_A.name,u_A.character);
            t.show_for_user(&u_A, &attacked_points);
            let coord = match promp_attack(&mut u_A){
                Ok(coords) => coords,
                Err(why) => {
                    println!("{why}");
                    break
                }
            };
            let attacked_point = Point::new(coord.0 as i8, coord.1 as i8);
            u_A.attack(&mut t, &attacked_point, &mut attacked_points, &mut u_B);


            u_B.my_turn = true;//false -> !true ->
            u_A.my_turn = false;//true -> !false
        }else{
            println!("Dear {}-{}, your pieces are: ", u_B.name,u_B.character);
            t.show_for_user(&u_B, &attacked_points);
            let coord = match promp_attack(&mut u_B){
                Ok(coords) => coords,
                Err(why) => {
                    println!("{why}");
                    break
                }
            };
            let attacked_point = Point::new(coord.0 as i8, coord.1 as i8);
            u_B.attack(&mut t, &attacked_point, &mut attacked_points, &mut u_A);

            u_A.my_turn = true;//true -> !false
            u_B.my_turn = false;//false -> !true ->            
        }
    }
    if u_A.give_up | u_B.give_up{
        println!("There is no winner, because you give up!");
        show_points(&u_A, &u_B);
    }
    
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