use super::towers::Towers;


fn input_usize(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to read_line");
        let parsed = buffer.trim().parse::<usize>();
        if let Ok(x) = parsed {
            break x;
        } else {
            println!("invalid input");
        }
    }
}


// Returns a tuple representing a move (the tower
// to take the disk from and the tower to place it on
// respectively)
fn get_player_move() -> (usize, usize) {
    let source = input_usize("Enter source tower (0-2)");
    let target = input_usize("Enter target tower (0-2)");
    (source, target)
}


#[allow(dead_code)]
pub fn run_game() {
    let mut towers = Towers::new(3);
    while !towers.win_state() {
        println!("\n{}", towers);
        let (source, target) = get_player_move();
        if !towers.mov(source, target) {
            println!("invalid move. sorry");
        }
    }
    println!("{}", towers);
    println!("you win. well done");
}

