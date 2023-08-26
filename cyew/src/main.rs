mod cell; //importing cell.rs code

use cell::{Cellule, State};
use gloo::timers::callback::Interval;
use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

//interface of Msg, which are the buttons to help control the game
pub enum Msg {
    Random,
    Start,
    Step, //go step by step
    Reset,
    Stop,
    SetState(String),
    ToggleCellule(usize), //use cell.rs code to configure states of alive
    Tick,                 //how fast calculations are carried out and displayed
}

//creation of grid
pub struct App {
    active: bool,          // is the game running
    selected_state: State, // what state is selected, must be a state in cell_states
    cell_states: Vec<State>,
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    _interval: Interval, //how far each cell is form each other
}

//use interface
impl App {
    pub fn toggle_state(&mut self, state: State) -> Result<(), String> {
        if state == State::A || state == State::B {
            return Err("Cannot toggle blank state".to_string());
        }

        if self.cell_states.contains(&state) {
            self.cell_states.retain(|s| *s != state);
            return Ok(());
        } else {
            self.cell_states.push(state);
            return Ok(());
        }
    }

    pub fn random_mutate(&mut self) {
        //goes through each pixel in a grid
        for cellule in self.cellules.iter_mut() {
            // Switch state to a randomly available state in the cell states
            let states = self.cell_states.len();
            let random_state = self.cell_states[rand::thread_rng().gen_range(0..states)];
            cellule.set_state(random_state);
        }
    }

    //makes all the pixels white - removes the cells
    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_blank();
        }
    }

    //step by step
    fn step(&mut self) {
        // edit step settings
        // let mut to_dead = Vec::new();
        // let mut to_live = Vec::new();

        // for row in 0..self.cellules_height {
        //     for col in 0..self.cellules_width {
        //         let neighbors = self.neighbors(row as isize, col as isize);

        //         let current_idx = self.row_col_as_idx(row as isize, col as isize);

        //         //if the cell being checked is alive and if it is alone (<2 cells around)
        //         //or if the cell is overpopulated(>3 cells around
        //         if self.cellules[current_idx].is_alive() {
        //             if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
        //                 to_dead.push(current_idx); //set the current cell to dead
        //             }

        //         //otherwise if the number of cells around is 3 then the dead cell is alive
        //         } else if Cellule::can_be_revived(&neighbors) {
        //             to_live.push(current_idx);
        //         }
        //     }
        // }
        // to_dead
        //     .iter()
        //     .for_each(|idx| self.cellules[*idx].set_dead());
        // to_live
        //     .iter()
        //     .for_each(|idx| self.cellules[*idx].set_alive());
    }

    //check all the surronding neibors around the cell - imagine your cell is in the center of a
    //3x3 grid
    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    //wrao prints each line. So for each row, print until the number of lines equivalent to height
    //of grid
    //vice versa for col
    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col // the grid?
    }

    //Rendering for HTMl - wasm
    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status: String = cellule.state.to_string();

        //?
        html! {
            <div key={idx} class={classes!("game-cellule", cellule_status)}
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>
            </div>
        }
    }
}

//using interface of app
impl Component for App {
    type Message = Msg;
    type Properties = ();

    //creates the grid, using the function above
    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick); //runs a callback for each tick
        let interval = Interval::new(200, move || callback.emit(())); //200ms between each moves -- runs above line

        let (cellules_width, cellules_height) = (60, 40); //grid is 53x40

        //runs the board as soon as the board is open - makes every cell dead
        Self {
            active: false,            //does not start game
            selected_state: State::B, //default state
            cellules: vec![Cellule::new(cell::State::A); cellules_width * cellules_height], //everything set to dead
            cell_states: vec![State::A, State::B, State::C, State::D, State::E], //5 enabled states by default
            cellules_width,
            cellules_height,
            _interval: interval, //tick speed basically
        }
    }

    //updates every 200ms
    //buttons displayed at the bottom
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            Msg::Step => {
                self.step();
                true
            }
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }

            //this is the function that happens when u click on a tile. (the wasm msg that sedss)
            Msg::ToggleCellule(idx) => {
                let cellule = self.cellules.get_mut(idx).unwrap();
                cellule.toggle_cell(self.selected_state);
                true
            }

            Msg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            }

            Msg::SetState(state) => {
                let state = State::from_string(&state);
                match state {
                    Ok(state) => {
                        self.selected_state = state;
                        true
                    }
                    Err(_) => false,
                }
            }
        }
    }

    //what displays the grid
    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self
            .cellules
            .chunks(self.cellules_width)
            .enumerate() //goes through each one
            .map(|(y, cellules)| {
                //mapping using y and cellules
                let idx_offset = y * self.cellules_width;

                let cells = cellules
                    .iter()
                    .enumerate()
                    .map(|(x, cell)| self.view_cellule(idx_offset + x, cell, ctx.link())); //map each x to grid
                html! {
                    <div key={y} class="game-row">
                        { for cells }
                    </div>
                }
            });

        html! {
               /*
                <section class="game-container">
                    <header class="app-header">
                       // <img alt="The app logo" src="favicon.ico" class="app-logo"/>
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">

                        <div class="game-of-life">
                            { for cell_rows }
                        </div>

                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "Random" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Step)}>{ "Step" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
                /*
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "Game of Life - a yew experiment " }
                    </strong>
                    <a href="https://github.com/yewstack/yew" target="_blank">{ "source" }</a>
                </footer>
                */
                */


            <div>
                //this will be on the left side
                <div class="split game-container">
                    <header class="app-header">
                       // <img alt="The app logo" src="favicon.ico" class="app-logo"/>
                        <h1 class="app-title">{ "Cellular Automata" }</h1>
                    </header>
                    <div class="game-area">

                        <div class="game-of-life">
                            { for cell_rows }
                        </div>

                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "Random" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Step)}>{ "Step" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                        </div>
                    </div>
                </div>


                <div class = "split right">
                    <div class = "txt">
                        <h3 id="h3_1">{"Dead cells: "}</h3>
                        <h3 id ="h3_2">{"Alive cells: "}</h3>
                        <h3 id ="h3_3">{"Spawn limit: "}</h3>
                        <h3 id ="h3_4">{"Revive: "}</h3>
                    </div>

                    <div class = "box">
                        //need to replace
                        //<button class="game-button menu" onclick={ctx.link().callback(|_| Msg:: left changer)}>{"<"}</button>
                        <div class = "menu">{"<"}</div>
                        <div>{"Counter"}</div>
                        <div class = "menu">{">"}</div>
                        //need to replace the arrow with the randomiser script
                        //<button class="game-button menu" onclick={ctx.link().callback(|_| Msg:: right changer)}>{">"}</button>
            
                    </div>


                </div>

            </div>
                /*
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "Game of Life - a yew experiment " }
                    </strong>
                    <a href="https://github.com/yewstack/yew" target="_blank">{ "source" }</a>
                </footer>
                */
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    //helps checks cells that are on the edge of the grid.
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

//rendering app with wasm
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
