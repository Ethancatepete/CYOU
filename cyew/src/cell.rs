
//Either two states
#[derive(Clone, Copy, PartialEq, Eq)] //Duplicate the same with partial equity(a!=b) and !(a==b), and equity (x==y)
pub enum State { //data in different states; it is a interface
    A(&'static str),
    B(&'static str),
    C(&'static str),
    D(&'static str),
    E(&'static str),
    F(&'static str),
    G(&'static str),
    H(&'static str),
    I(&'static str),
    J(&'static str),
    K(&'static str),
    L(&'static str),
    M(&'static str),
    N(&'static str),
    O(&'static str),
    P(&'static str),
}

#[derive(Clone, Copy)]
pub struct Cellule {
    pub state: State,
}

impl Cellule {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    // //by default some are dead
    // pub fn new_dead() -> Self {
    //     Self { state: State::Dead }
    // }

    // //some are alive
    // pub fn set_alive(&mut self) {
    //     self.state = State::Alive;
    // }

    // //set those alive to dead
    // pub fn set_dead(&mut self) {
    //     self.state = State::Dead;
    // }

    // //checks those alive cells are still alive
    // pub fn is_alive(self) -> bool {
    //     self.state == State::Alive
    // }

    // //function that keeps game running
    // pub fn toggle(&mut self) {
    //     if self.is_alive() {
    //         self.set_dead()
    //     } else {
    //         self.set_alive()
    //     }
    // }

// TODO REPLACE FOLLOWING CODE WITH INPUTTABLE LAMBDA FUNCTIONS. SCRIPT RULES -> LAMBDA FUNCTIONS? replacement settings.
// check dynamic programming

//  following cringe rules

    // //counts alive neighbors around itself, contained inside an array which has a dynamic siae
    // //stored on the heap
    // pub fn count_alive_neighbors(neighbors: &[Self]) -> usize {
    //     neighbors.iter().filter(|n| n.is_alive()).count()
    // } 
    
    // //if alive neighbors less than 2 - it is "Alone" (True)
    // pub fn alone(neighbors: &[Self]) -> bool {
    //     Self::count_alive_neighbors(neighbors) < 2
    // }

    // //if alive neighbors greater than 3 - it is "Over-populated" (True)
    // pub fn overpopulated(neighbors: &[Self]) -> bool {
    //     Self::count_alive_neighbors(neighbors) > 3
    // }

    // //if alive neibors equal to 3 - it "can_be_revived" (True)
    // pub fn can_be_revived(neighbors: &[Self]) -> bool {
    //     Self::count_alive_neighbors(neighbors) == 3
    // }
}
