//Either two states
#[derive(Clone, Copy, PartialEq, Eq)] //Duplicate the same with partial equity(a!=b) and !(a==b), and equity (x==y)
pub enum State {
    //types of states for each cell
    A, // blank, 0
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
}

impl State {
    pub fn to_string(&self) -> String {
        match self {
            State::A => "A".to_string(),
            State::B => "B".to_string(),
            State::C => "C".to_string(),
            State::D => "D".to_string(),
            State::E => "E".to_string(),
            State::F => "F".to_string(),
            State::G => "G".to_string(),
            State::H => "H".to_string(),
            State::I => "I".to_string(),
            State::J => "J".to_string(),
            State::K => "K".to_string(),
            State::L => "L".to_string(),
            State::M => "M".to_string(),
            State::N => "N".to_string(),
            State::O => "O".to_string(),
            State::P => "P".to_string(),
        }
    }
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

    pub fn set_blank(&mut self) {
        self.state = State::A;
    }

    // //by default some are dead
    // pub fn new_dead() -> Self {
    //     Self { state: State::Dead }
    // }

    // //checks those alive cells are still alive
    // pub fn is_alive(self) -> bool {
    //     self.state == State::Alive
    // }

    //function that keeps game running
    pub fn toggle_cell(&mut self, state: State) {
        match self.state {
            State::A => self.state = state,
            _ => self.state = State::A,
        }
    }

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
