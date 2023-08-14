mod cell; //importing cell.rs code

use cell::Cellule;
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
    ToggleCellule(usize), //use cell.rs code to configure states of alive
    Tick, //how fast calculations are carried out and displayed
}

//creation of grid
pub struct App {
    active: bool, //is the game running
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    _interval: Interval, //how far each cell is form each other
}

//use interface
impl App {

    //randomly changes cell around
    pub fn random_mutate(&mut self) {

        //goes through each pixel in a grid
        for cellule in self.cellules.iter_mut() {
           
            //if random generator is True (50/50)
            if rand::thread_rng().gen() {
                cellule.set_state(cell::State::A("alive")); //set the cell to alive
            } else {
                cellule.set_state(cell::State::B("dead")); //otherwise set it to dead
            }
        }
    }
    
    //makes all the pixels white - removes the cells
    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_state(cell::State::A("dead"));
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
        let cellule_status: String = {
            match cellule.state {
                cell::State::A(_) => "your mom".to_string(),
                cell::State::B(_) => todo!(), // ill finish this later itll prob just be colors anyways, might make it dynamically programmable
                cell::State::C(_) => todo!(),
                cell::State::D(_) => todo!(),
                cell::State::E(_) => todo!(),
                cell::State::F(_) => todo!(),
                cell::State::G(_) => todo!(),
                cell::State::H(_) => todo!(),
                cell::State::I(_) => todo!(),
                cell::State::J(_) => todo!(),
                cell::State::K(_) => todo!(),
                cell::State::L(_) => todo!(),
                cell::State::M(_) => todo!(),
                cell::State::N(_) => todo!(),
                cell::State::O(_) => todo!(),
                cell::State::P(_) => todo!(),
            }
        };

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

        let (cellules_width, cellules_height) = (53, 40); //grid is 53x40

        //runs the board as soon as the board is open - makes every cell dead
        Self {
            active: false, //does not start game
            cellules: vec![Cellule::new(cell::State::A("dead")); cellules_width * cellules_height], //everything set to dead
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
                // cellule.toggle(); fuck u no toggle function yet
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
        }
    }

//what displays the grid
    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows =
            self.cellules
                .chunks(self.cellules_width)
                .enumerate() //goes through each one
                .map(|(y, cellules)| { //mapping using y and cellules
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
                        <h1 class="app-title">{ "Game of Life" }</h1>
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
                        <h3>{"Dead cells"}</h3>
                        <h3>{"Alive cells"}</h3>
                        <h3>{"Spawn limit"}</h3>
                        <h3>{"Revive"}</h3>
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
