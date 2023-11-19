pub type State = char;

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
        self.state = 'A';
    }

    //function that keeps game running
    pub fn toggle_cell(&mut self, state: State) {
        match self.state {
            'A' => self.state = state,
            _ => self.state = 'A',
        }
    }
}
