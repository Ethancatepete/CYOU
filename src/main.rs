mod cell;

use cell::{Cellule, State};
use gloo::{timers::callback::Interval, storage::{LocalStorage, Storage}};
use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::BuiltinTheme,
    yew::CodeEditor,
};
use rand::{seq::IteratorRandom, Rng};
use rhai::{Engine, EvalAltResult};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
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
    SaveStates,
    IncrSize,
    DecrSize,
    Tick, // game update tick
}

pub struct App {
    active: bool,                            // is the game running
    selected_state: State, // what state is selected, must be a state in cell_states
    cell_states: BTreeMap<State, TextModel>, // set and textmodel? language as rhaiscript
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    current_eval_cell: usize, //current cell being evaluated
    engine: Engine,
    logbook: Arc<RwLock<Vec<String>>>,
    _interval: Interval,
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
        for cellule in self.cellules.iter_mut() {
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

    fn step(&mut self) {
        let mut new_cellules = self.cellules.clone();

        for (idx, cellule) in self.cellules.iter().enumerate() {
            self.current_eval_cell = idx;

            let neighbours: String = self
                 .neighbours()
                .iter()
                .map(|c| format!("'{}'", c))
                .collect::<Vec<String>>()
                .join(", "); // create formatting string so that the vector array of neighbours turns into a string of comma seperated values

            let script = format!(
                "const CURRENT = {cell_id};\nconst NEIGHBOURS = [{neighbours}];\n fn count_neighbours(a) {{let count = 0; for (state, index) in global::NEIGHBOURS {{if state == a {{ count += 1; }}}} return count; }};\n{code}", // code injection of variables cause the script doesnt let me define cool rust functions
                cell_id = idx,
                neighbours = neighbours,
                code = &self.cell_states[&cellule.state].get_value()
            );

            let log = self.logbook.clone();

            self.engine.on_print(move |s| {
                let entry = s.to_string();
                log.write().unwrap().push(entry);
            });

            let result = self.engine.eval::<char>(&script); // executes the user script for each cell, could be multithreaded?

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
        let mut neighbours: Vec<char> = Vec::new();
        let (x, y) = (idx % self.cellules_width, idx / self.cellules_width);

        //goes through each neighbour
        // 0 3 5
        // 1   6
        // 2 4 7
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

    
    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status: String = cellule.state.to_string();
        let mut cellule_size = 40.0;

        cellule_size = cellule_size / self.cellules_width as f32 ;

        html! {
            <div key={idx} class={classes!("cellule", cellule_status)} style={format!("width: {}rem; height: {}rem;", cellule_size, cellule_size)}
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

        let (cellules_width, cellules_height) = (10, 10); //grid is SQUARE

        let logbook = Arc::new(RwLock::new(Vec::<String>::new()));

        Self {
            active: false,       //does not start game
            selected_state: 'B', //default state
            cellules: vec![Cellule::new('A'); cellules_width * cellules_height], //everything set to dead
            cell_states: BTreeMap::from([
                (
                    'A',
                    TextModel::create("//dead \nlet live = count_neighbours('B'); \n\nif live == 3 {\n    return 'B'\n} else {\n    return 'A'}", Some("rust"), None).unwrap(),
                ),
                (
                    'B',
                    TextModel::create("//alive \nlet live = count_neighbours('B'); \n\nif live < 2 || live > 3 {\n    return 'A' \n} else {\n    return 'B'\n} ", Some("rust"), None).unwrap(),

                ),
                ]), //2 enabled states by default
            cellules_width,
            cellules_height,
            current_eval_cell: 0,
            engine: {
                let mut engine = Engine::new();
                engine.register_fn("rand", Self::rand);
                engine.register_fn("neighbours", Self::neighbours);
                engine.register_fn("rand_state", Self::rand_state);
                engine.set_optimization_level(rhai::OptimizationLevel::Full);
                engine
            },
            logbook: logbook.clone(),
            _interval: interval,
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
                log::info!("Step");
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

            Msg::SaveStates => {
                for (state, model) in self.cell_states.iter() {
                    LocalStorage::set(state.to_string(), model.get_value());
                }
                true
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
                    TextModel::create(&LocalStorage::get(new_state.to_string()).unwrap_or_else(|_|{format!("return \'{}\';", (((new_state as u8) + 1) as char))}), Some("rust"), None).unwrap(),
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
                true
            }

            Msg::IncrSize => {
                if self.cellules_width > 100 || self.cellules_height > 100 {
                    log::error!("Cannot increase size");
                    return false;
                }
                self.cellules_width += 1;
                self.cellules_height += 1;
                self.cellules = vec![Cellule::new('A'); self.cellules_width * self.cellules_height];
                log::info!("Increased size");
                true
            }

            Msg::DecrSize => {
                if self.cellules_width < 2 || self.cellules_height < 2 {
                    log::error!("Cannot decrease size");
                    return false;
                }
                self.cellules_width -= 1;
                self.cellules_height -= 1;
                self.cellules = vec![Cellule::new('A'); self.cellules_width * self.cellules_height];
                log::info!("Decreased size");
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
                html!{
                    <div key={y} class="cell-row">
                        { for cells }
                    </div>
                }
            });

        let available_states = self.cell_states.keys().cloned().collect::<Vec<char>>();

        html! {
            <div class="app">
                <div class="pane">
                    <h1>{ "Cellular Automata" }</h1>
                    <div class="cells">
                        {for cell_rows}
                    </div>
                    <div class="controls">
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "Random" }</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Step)}>{ "Step" }</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::IncrSize)}>{ "Incr" }</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::DecrSize)}>{ "Decr" }</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::SaveStates)}>{ "Save" }</button>
                    </div>
                    <div class="cheatsheet">
                        <script src="https://gist.github.com/wylited/b8d605326cf30fd54b34f9576378b843.js"></script>
                    </div>
                </div>
                <div class="pane">
                    <div class="editor">
                        <div class="nav">
                            <button class="nav-button" onclick={ctx.link().callback(|_| Msg::RemoveState)}>{"-"}</button>
                            {
                                available_states.into_iter().map(|state| {
                                    let mut class_string = format!("{} nav-button", state);
                                    if state == self.selected_state {
                                        class_string = format!("{} selected", class_string)
                                    }
                                    html!{ <button class={ class_string } onclick={ctx.link().callback(move |_| Msg::SetState(state))}>{ state }</button>}
                                }).collect::<Html>()
                            }
                            <button class="nav-button" onclick={ctx.link().callback(|_| Msg::AddState)}>{ "+" }</button>
                        </div>
                        <div class="code-editor">
                        <CodeEditor classes={"editor-height"} options={
                                CodeEditorOptions::default()
                                    .with_language("rust".to_owned())
                                    .with_model(self.cell_states[&self.selected_state].clone())
                                    .with_builtin_theme(BuiltinTheme::VsDark)
                                    .with_automatic_layout(true)
                                    .to_sys_options()} />
                        <br />
                        </div>
                        <div class="log">
                        <ul>
                        { for self.logbook.read().unwrap().iter().map(|entry| {
                            html! { <li>{ entry }</li> }
                        })}
                        </ul>
                        </div>
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
