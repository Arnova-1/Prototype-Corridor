use std::cmp::min;
use rand::prelude::IndexedRandom;

#[derive(Default, Debug)]
struct Corridor {
    position: u32,
}

impl Corridor {
    fn reset(&mut self) {
        self.position = 1
    }

    fn step(&mut self, action: Action) -> bool {
        match action {
            Action::Left => {
                self.position = self.position.saturating_sub(1);
            }
            Action::Right => {
                self.position = min(self.position + 1, 2)
            }
        }
        self.position == 2
    }
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Right,
    Left,
}

fn agent() -> Action {
    let mut rng = rand::rng();
    let actions : [Action; 2] = [Action::Right, Action::Left];
    actions.choose(&mut rng).cloned().unwrap()
}

// fn render(corridor: &Corridor) {
//     let w = 3;
//     let pos = corridor.position;
//
//     let row: String = (0..w)
//         .map(|i| if i == pos { "X" } else { "-" })
//         .collect();
//     let border = "*".repeat((w + 2) as usize);
//
//     println!("{border}\n*{row}*\n{border}")
// }

fn main() {
    let mut env = Corridor::default();
    env.reset();
    let mut steps = 0;
    let mut n = 0;

    loop {
        let action = agent();
        let is_done = env.step(action);
        steps += 1;
        //println!("step {}: action={:?}, new state={}", steps, action, env.position);
        //render(&env);

        if is_done {
            n += 1;
            env.reset();
        }

        if n == 10000 {
            println!("Steps: {steps}");
            break
        }
    }
    //println!("Episode Over.")
}
