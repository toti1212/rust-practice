// Struct
#[derive(Debug)]
enum Position {
    PointGuard,
    ShootingGuard,
    SmallForward,
    Center,
    PowerForward,
}

struct Player {
    name: String,
    number: u8,
    position: Position,
    max_score: u8,
}

impl Player {
    fn new(name:String, number: u8, position: Position) -> Player {
        Player {
            name: name,
            number: number,
            position: position,
            max_score: 0
        }
    }

    fn annotate(&mut self) {
        self.max_score += 1;
        println!("The player scored! {}", self.max_score);
    }
}

fn show_player() {
    let mut player1 = Player::new (
        String::from("Michael Jordan"),
        23,
        Position::ShootingGuard,
    );
    
    println!(
        "The first player we have is {}, it plays as  a {:#?} with a max score of {}",
        player1.name, player1.position, player1.max_score
    );
    // score
    player1.annotate();
    println!("Now the player {} have {} points", player1.name, player1.max_score);
}

fn main() {
    show_player();
}
