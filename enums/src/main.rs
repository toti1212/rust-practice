enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => {
            println!("It is about {} o'clock", hours);
        }
        Clock::Digital(hours, minutes) => {
            println!("Its {}:{} in my digital clock.", hours, minutes);
        }
        Clock::Analog(hours, minutes, seconds) => {
            println!("Its {}:{} with {} seconds.", hours, minutes, seconds);
        }
    }
}

fn main() {
    let sundial = Clock::Sundial(12);
    let analog = Clock::Digital(12, 45);
    let digital = Clock::Analog(12, 45, 12);
    tell_time(sundial);
    tell_time(analog);
    tell_time(digital);
}
