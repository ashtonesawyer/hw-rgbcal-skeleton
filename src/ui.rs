use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    ///Print State to terminal
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    ///Set defaults for UiState
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    button_a: Button,
    button_b: Button,
    state: UiState,
}

impl Ui {
    pub fn new(knob: Knob, button_a: Button, button_b: Button) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    ///Update global LED levels to match local
    async fn update_led(&mut self, level: u32, led: usize) {
        if level !=  self.state.levels[led] {
            self.state.levels[led] = level;
            self.state.show();
            set_rgb_levels(|rgb| {
                *rgb = self.state.levels;
            })
            .await;
        }
    }

    ///Update global frame rate to match local
    async fn update_fr(&mut self, level: u64) {
        let lvl = (level + 1) * 10;
        if lvl != self.state.frame_rate {
            self.state.frame_rate = lvl;
            self.state.show();
            set_frame_rate(|fr| {
                *fr = self.state.frame_rate;
            })
            .await;
        }
    }

    ///Set levels from knob and show
    pub async fn run(&mut self) -> ! {
        let lvl = self.knob.measure().await as u64;
        self.state.frame_rate = (lvl + 1) * 10;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            let level = self.knob.measure().await;
            match (self.button_a.is_low(), self.button_b.is_low()) {
                (true, true) => self.update_led(level, 0).await,     // red
                (true, false) => self.update_led(level, 1).await,    // green
                (false, true) => self.update_led(level, 2).await,    // blue
                (false, false) => self.update_fr(level as u64).await,
            }
            Timer::after_millis(50).await;
        }
    }
}
