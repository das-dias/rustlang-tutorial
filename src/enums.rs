// enums are types which have a few definitive values

enum Movement{
    // variants of movement
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: &Movement){
    // perform action depending on movement
    // smilar to a switch case in C
    match m {
        Movement::Up => println!("Avatar moving Up!"),
        Movement::Down => println!("Avatar moving Down!"),
        Movement::Left => println!("Avatar moving Left!"),
        Movement::Right => println!("Avatar moving Right!")
    }
}

pub fn run(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;

    move_avatar(&avatar1);
    move_avatar(&avatar2);
    move_avatar(&avatar3);
    // not possible! variable avatar1 will be moved in memory
    // once ownership will be transferred
    //move_avatar(avatar1);// error

    // to fix it, parse a borrowed value to move avatar
    move_avatar(&avatar2);
    move_avatar(&avatar3);
}