mod towers;
use towers::Towers;
mod auto_solve;
use auto_solve::auto_solve;
mod game;
// use game::run_game;




fn main() {
    let mut towers = Towers::new(9);
    println!("Before: \n{}", towers);
    println!("Auto solving...");
    auto_solve(&mut towers);
    println!("After: \n{}", towers);
    // run_game();
}
