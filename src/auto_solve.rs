use super::towers::Towers;

// Given a pool of possible values and an array
// of taken values, returns the first value that is not taken
fn first_available<T: std::cmp::PartialEq + Copy>(pool: &[T], taken: &[T]) -> Option<T> {
    for x in pool {
        if !taken.contains(x) {
            return Some(*x);
        }
    }
    None
}


// Moves 2 disks from the src tower to the tgt tower
fn mov_two(towers: &mut Towers, src: usize, tgt: usize) {
    let buf = first_available(&[0,1,2], &[src, tgt]).unwrap();

    if !towers.mov(src, buf)  ||
        !towers.mov(src, tgt)  ||
        !towers.mov(buf, tgt) {
        panic!("cannot move 2 disks from tower {} to tower {}", src, tgt);
    }
}


// Moves n disks from the src tower to the tgt tower
fn partial_solve(towers: &mut Towers, n: usize, src: usize, tgt: usize) {
    if(n == 1) {
        towers.mov(src, tgt);
    }
    else if n == 2 {
        mov_two(towers, src, tgt);
    }
    else if n > 2 {
        let buf = first_available(&[0,1,2], &[src, tgt]).unwrap();
        partial_solve(towers, n -1, src, buf);
        towers.mov(src, tgt);
        partial_solve(towers, n-1, buf, tgt);
    }
}


// 3 moves to move 2 disks
// moving 3 disks requires moving 2 disks twice and 1 additional move (6+1 = 7 moves)
// moving 4 disks requires moving 3 disks twice and 1 additional move (7*2 + 1 = 15 moves)
// moving 5 disks requires moving 4 disks twice and 1 additional move (30 +1 = 31 moves)
// Time complexity for n disks is O(2^n)
pub fn auto_solve(towers: &mut Towers) {
    partial_solve(towers, towers.num_disks(), 0, 2);
}
