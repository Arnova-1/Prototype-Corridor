use std::cmp::min;
use rand::prelude::IndexedRandom;

#[derive(Default, Debug)]
struct Corridor {
    position: u32,
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Right,
    Left,
}

#[derive(Default)]
struct Agent {
    brain: [[f32; 2];3]
}

impl Corridor {
    fn reset(&mut self) {
        self.position = 1
    }

    fn step(&mut self, action: Action) -> (bool, f32, usize, usize) {
        let previous_step = self.position;

        let index = match action {
            Action::Left => {
                self.position = self.position.saturating_sub(1);
                0
            }
            Action::Right => {
                self.position = min(self.position + 1, 2);
                1
            }
        };

        let grade = if self.position == 2 { 1.0 } else { 0.0 }; // Separate this later

        (self.position == 2, grade, previous_step as usize, index as usize)
    }
}

impl Agent {
    fn choose_action(&self) -> Action {
        let mut rng = rand::rng();
        let actions : [Action; 2] = [Action::Left, Action::Right];
        actions.choose(&mut rng).cloned().unwrap()
    }
    fn update_table(&mut self, grade: f32, pos: usize, actions: usize) {
        self.brain[pos][actions] += grade;
    }
}

fn render(corridor: &Corridor) {
    let w = 3;
    let pos = corridor.position;

    let row: String = (0..w)
        .map(|i| if i == pos { "X" } else { "-" })
        .collect();
    let border = "*".repeat((w + 2) as usize);

    println!("{border}\n*{row}*\n{border}")
}

fn main() {
    let mut env = Corridor::default();
    let mut agent = Agent::default();
    env.reset();
    let mut steps = 0;
    let mut n = 0;

    loop {
        let action = agent.choose_action();
        let is_done = env.step(action);
        steps += 1;
        println!("step {}: action={:?}, new state={}", steps, action, env.position);
        render(&env);

        agent.update_table(is_done.1, is_done.2, is_done.3);

        if is_done.0 {
            n += 1;
            env.reset();
            println!("Episode Over.");
        }

        if n == 10 {
            println!("Steps: {steps}");
            break
        }
    }
    println!("{:?}", agent.brain)
}
