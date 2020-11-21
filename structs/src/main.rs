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

fn show_player() {
    let mut player1 = Player {
        name: String::from("Michael Jordan"),
        number: 23,
        position: Position::ShootingGuard,
        max_score: 68,
    };
    player1.max_score += 1;

    println!(
        "The first player we have is {}, it plays as  a {:#?} with a max score of {}",
        player1.name, player1.position, player1.max_score
    );
}

//  Enums like struct
enum Clock {
    Analog { hours: u8 },
    Digital { hours: u8, minutes: u8 },
}

fn tell_me_the_time(clock: Clock) {
    match clock {
        Clock::Digital { hours, minutes } => {
            println!("It's {}:{} o'clock", hours, minutes);
        }
        _any => {
            println!("I don't know");
        }
    }
}

fn main() {
    show_player();
    let clock = Clock::Digital {
        hours: 12,
        minutes: 45,
    };
    tell_me_the_time(clock);
}
