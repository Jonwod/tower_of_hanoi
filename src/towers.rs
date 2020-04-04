use std::fmt;

pub struct Disk {
    pub size: u32,
}

pub struct Towers {
    towers: [Vec<Disk>; 3],
}


impl fmt::Display for Towers {

    /*
    | | |
    | | 2
    | 1 3
    _____
    */

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.num_disks();
        let mut s = String::new();

        for i in 0..height {
            for tower in &self.towers {
                let h = height - i - 1;
                let x = if h < tower.len() {
                    tower[h].size.to_string()
                } else {
                    String::from("|")
                };
                s.push_str(&(x + " "));
            }
            s.push_str("\n");
        }

        // Construct the base
        s.push_str("_");
        for _ in 1..self.towers.len() {
            s.push_str("__");
        }

        write!(f, "{}", s)
    }
}


impl Towers {
    pub fn new(num_disks: u32) -> Towers {
        Towers{
            towers: [
                {
                    let mut v = Vec::new();
                    for i in 0..num_disks {
                        v.push(Disk{size: num_disks - i});
                    }
                    v
                },
                Vec::new(),
                Vec::new(),
            ]
        }
    }


    pub fn num_disks(&self) -> usize {
        let mut num = 0;
        for tower in &self.towers {
            num += tower.len();
        }
        num
    }


    pub fn win_state(&self) -> bool {
        self.towers.last().unwrap().len() == self.num_disks()
    }


    // Moves a disk from the top of the source tower
    // to the top of the target tower. Returns false
    // if unable to move a disk.
    pub fn mov(&mut self, source: usize, target: usize) -> bool {
        if source >= self.towers.len() || target >= self.towers.len() {
            return false;
        }
        if self.towers[source].len() == 0 {
            return false;
        }
        if self.towers[target].len() > 0  &&
           self.towers[target].last().unwrap().size < self.towers[source].last().unwrap().size{
            return false;
        }

        let disk = self.towers[source].pop().unwrap();
        self.towers[target].push(disk);

        true
    }
}
