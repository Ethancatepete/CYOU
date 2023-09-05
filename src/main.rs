mod cell; //importing cell.rs code

use cell::{Cellule, State};
use gloo::timers::callback::Interval;
use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::BuiltinTheme,
    yew::CodeEditor,
};
use rand::{seq::IteratorRandom, Rng};
use rhai::{Engine, EvalAltResult};
use std::collections::BTreeMap;
use yew::{classes, html, html::Scope, Component, Context, Html};

pub enum Msg {
    Random,
    Start,
    Step, //go step by step
    Reset,
    Stop,
    Conditions(String), // Name of condition as a rhai script, text models?
    SetState(char),     // change state of
    ToggleCellule(usize),
    AddState,
    RemoveState,
    Tick, // game update tick
}

//creation of grid
pub struct App {
    active: bool,                            // is the game running
    selected_state: State, // what state is selected, must be a state in cell_states
    cell_states: BTreeMap<State, TextModel>, // set and textmodel? language as rhaiscript
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    current_eval_cell: usize, //current cell being evaluated
    engine: Engine,
    _interval: Interval, //how far each cell is from each other
}

//use interface
impl App {
    pub fn toggle_state(&mut self, state: State) -> Result<(), String> {
        if state == 'A' || state == 'B' {
            return Err("Cannot toggle these state".to_string());
        }

        if self.cell_states.contains_key(&state) {
            self.cell_states.remove(&state);
            return Ok(());
        } else {
            self.cell_states
                .insert(state, TextModel::create("", Some("rust"), None).unwrap());
            return Ok(());
        }
    }

    pub fn random_mutate(&mut self) {
        //goes through each pixel in a grid
        for cellule in self.cellules.iter_mut() {
            // Switch state to a randomly available state in the cell states
            let random_state = self
                .cell_states
                .keys()
                .choose(&mut rand::thread_rng())
                .unwrap();
            cellule.set_state(*random_state);
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
        let mut new_cellules = self.cellules.clone();

        //goes through each cell
        for (idx, cellule) in self.cellules.iter().enumerate() {
            self.current_eval_cell = idx;
            let result = self
                .engine
                .eval::<char>(&self.cell_states[&cellule.state].get_value());
            log::info!("{:?}", result);
            match result {
                Ok(state) => {
                    new_cellules[idx].set_state(state);
                }
                Err(err) => if let EvalAltResult::ErrorRuntime(_err, _) = *err {},
            }
        }

        self.cellules = new_cellules;
    }

    fn rand(a: i64, b: i64) -> bool {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..b);
        x < a
    }

    fn rand_state() -> char {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..5);
        match x {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            _ => 'A',
        }
    }

    fn neighbours(&self) -> Vec<char> {
        let idx = self.current_eval_cell;
        let mut neighbours = Vec::new();
        let (x, y) = (idx % self.cellules_width, idx / self.cellules_width);

        //goes through each neighbour
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue; //skip the cell itself
                }

                let (x, y) = (x as i32 + i, y as i32 + j);
                if x < 0 || y < 0 {
                    continue; //skip if out of bounds
                }

                let (x, y) = (x as usize, y as usize);
                if x >= self.cellules_width || y >= self.cellules_height {
                    continue; //skip if out of bounds
                }

                let idx = y * self.cellules_width + x;
                neighbours.push(self.cellules[idx].state);
            }
        }

        neighbours
    }

    fn count_neighbours(&self, filterstate: char) -> usize {
        self.neighbours()
            .iter()
            .filter(|&state| state == &filterstate)
            .count()
    }
    //Rendering for HTMl - wasm
    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status: String = cellule.state.to_string();
        html! {
            <div key={idx} class={classes!("game-cellule", cellule_status)}
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>
            </div>
        }
    }
}

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
            active: false,       //does not start game
            selected_state: 'B', //default state
            cellules: vec![Cellule::new('A'); cellules_width * cellules_height], //everything set to dead
            cell_states: BTreeMap::from([
                (
                    'A',
                    TextModel::create("return \'B\'", Some("rust"), None).unwrap(),
                ),
                (
                    'B',
                    TextModel::create("return \'C\'", Some("rust"), None).unwrap(),
                ),
                (
                    'C',
                    TextModel::create("return \'D\'", Some("rust"), None).unwrap(),
                ),
                (
                    'D',
                    TextModel::create("return \'E'", Some("rust"), None).unwrap(),
                ),
                (
                    'E',
                    TextModel::create("return \'A\'", Some("rust"), None).unwrap(),
                ),
            ]), //5 enabled states by default
            cellules_width,
            cellules_height,
            current_eval_cell: 0,
            engine: {
                let mut engine = Engine::new();
                engine.register_fn("rand", Self::rand);
                engine.register_fn("neighbours", Self::neighbours);
                engine.register_fn("count_neighbours", Self::count_neighbours);
                engine.register_fn("rand_state", Self::rand_state);
                engine
            },
            _interval: interval, //tick speed basically
        }
    }

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
                if self.cell_states.contains_key(&state) {
                    self.selected_state = state;

                    true
                } else {
                    log::error!("Invalid state: {}", state);
                    false
                }
            }

            Msg::Conditions(condition) => {
                let state = self.selected_state;
                // update the string in the btreemap
                if self.cell_states.contains_key(&state) {
                    self.cell_states[&state].set_value(&condition);
                    true
                } else {
                    log::error!("Invalid selected state: {}", state);
                    false
                }
            }

            Msg::AddState => {
                if self.cell_states.len() >= 16 {
                    log::error!("Too many states");
                    return false;
                }

                let mut new_state = 'A';
                while self.cell_states.contains_key(&new_state) {
                    new_state = (new_state as u8 + 1) as char;
                }

                self.cell_states.insert(
                    new_state,
                    TextModel::create("", Some("rust"), None).unwrap(),
                );
                self.selected_state = new_state;
                true
            }

            Msg::RemoveState => {
                if self.cell_states.len() <= 2 {
                    log::error!("Too few states");
                    return false;
                }

                if self.selected_state == *self.cell_states.keys().last().unwrap() {
                    self.selected_state = *self.cell_states.keys().nth(1).unwrap();
                }

                for cellule in self.cellules.iter_mut() {
                    if cellule.state == *self.cell_states.keys().last().unwrap() {
                        cellule.set_state(*self.cell_states.keys().nth(1).unwrap());
                    }
                }

                let last_state = self.cell_states.keys().last().unwrap().clone();
                self.cell_states.remove(&last_state);

                log::info!("removed state {}", last_state);
                log::info!("{:?}", self.cell_states.clone());
                true
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

        // sort the cell_states btreemap based on the keys (character smallest to largest a-m)

        let available_states = self.cell_states.keys().cloned().collect::<Vec<char>>();

        html! {
            <div>
            //this will be on the eft side
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
                    <div class="nav">
                        <button class="- nav-button" onclick={ctx.link().callback(|_| Msg::RemoveState)}>{ "-"}</button>
                        {
                            available_states.into_iter().map(|state| {
                                let class_string = format!("{} nav-button", state);
                                html!{ <button class={ class_string } onclick={ctx.link().callback(move |_| Msg::SetState(state))}>{ state }</button>}
                            }).collect::<Html>()
                        }
                        <button class="+ nav-button" onclick={ctx.link().callback(|_| Msg::AddState)}>{ "+" }</button>
                    </div>


                    <div class = "txt">
                        <CodeEditor classes={"full-height"} options={
                            CodeEditorOptions::default()
                                .with_language("rust".to_owned())
                                .with_model(self.cell_states[&self.selected_state].clone())
                                .with_builtin_theme(BuiltinTheme::VsDark)
                                .with_automatic_layout(true)
                                .to_sys_options()
                        } />
                        <br />
                        <button class="game-button" onclick={ctx.link().callback(|_| Msg::Conditions("".to_string()))}>{ "Submit" }</button>

                    </div>
                </div>
            </div>
        }
    }
}
//rendering app with wasm
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
