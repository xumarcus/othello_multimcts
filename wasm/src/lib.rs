// wasm_bindgen BEGIN

mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// wasm_bindgen END

use othello_multimcts::*;
use rand::thread_rng;

// glue code

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct JSRunAI {
    next_move: u64,
    count: usize,
    score: f32,
    board: JSBoard
}

#[wasm_bindgen]
impl JSRunAI {
    pub fn get_next_move(&self) -> u64 {
        self.next_move
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }

    pub fn get_board(&self) -> JSBoard {
        self.board
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct JSBoard(Board);

#[wasm_bindgen]
impl JSBoard {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        Self(Board::default())
    }

    pub fn make(me: u64, op: u64, is_black: bool) -> Self {
        utils::set_panic_hook();
        Self(unsafe { Board::make(me, op, if is_black
            { Side::Black } else { Side::White }) })
    }

    pub fn place(&self, next_move: u64) -> Self {
        Self(self.0.place(Moves(next_move)))
    }

    pub fn run_ai(&self, timeout: f64) -> JSRunAI {
        let date = js_sys::Date::new_0();
        let mut root = Node::new(self.0);
        let mut algo = Algo::new(AlgoType::Roxanne, 0.05, thread_rng());
        while js_sys::Date::new_0().get_time() - date.get_time() < timeout {
            for _ in 0..100 {
                MCTSRunner::new(&mut root)
                    .run_sim(&mut algo)
                    .run_update(&mut root);
            }
        }
        let count = root.place_best();
        JSRunAI {
            next_move: root.next_move().0,
            count,
            score: root.avg().raw(),
            board: JSBoard(*root.board())
        }
    }

    // wasm_bindgen getters

    pub fn get_blacks(&self) -> u64 {
        match *self.0.side() {
            Side::Black => *self.0.get_me(),
            Side::White => *self.0.get_op()
        }
    }

    pub fn get_whites(&self) -> u64 {
        match *self.0.side() {
            Side::Black => *self.0.get_op(),
            Side::White => *self.0.get_me()
        }
    }

    pub fn count_blacks(&self) -> u32 {
        self.get_blacks().count_ones()
    }

    pub fn count_whites(&self) -> u32 {
        self.get_whites().count_ones()
    }

    pub fn is_human_turn(&self) -> bool {
        *self.0.side() == Side::default()
    }

    pub fn get_moves(&self) -> u64 {
        self.0.moves().0
    }
}
