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

    fn step(&mut self, action: Action) -> (bool, f32, usize, usize, usize) {
        let previous_step = self.position;

        let next_action = match action {
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

        (self.position == 2, grade, previous_step as usize, next_action as usize, self.position as usize)
    }
}

impl Agent {
    fn choose_action(&self, pos: usize) -> Action {
        let gate = rand::random_range(0.0..1.0);
        let epsilon = 0.1; // Exploration (10% chance it picked random instead of favoring weighted table)

        let mut rng = rand::rng();
        let actions: [Action; 2] = [Action::Left, Action::Right];
        let random_actions = actions.choose(&mut rng).cloned().unwrap();

        let weighted = self.compute_weight(pos);

        if gate > epsilon {
            weighted.unwrap_or_else(|| {
                random_actions
            })
        } else {
            random_actions
        }
    }
    fn compute_weight(&self, pos: usize) -> Option<Action> {
        let left = self.brain[pos][0];
        let right = self.brain[pos][1];

        if left > right {
            Some(Action::Left)
        } else if right > left {
            Some(Action::Right)
        } else {
            None
        }
    }
    fn update_table(&mut self, prev_pos: usize, prev_actions: usize, reward: f32, new_pos: usize) {
        let left = self.brain[new_pos][0];
        let right = self.brain[new_pos][1];
        let new_max = left.max(right);

        self.brain[prev_pos][prev_actions] = self.brain[prev_pos][prev_actions] + 0.1 *
            (reward + 0.9 * new_max - self.brain[prev_pos][prev_actions]);
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
        let action = agent.choose_action(env.position as usize);
        let environment = env.step(action);
        steps += 1;
        println!("step {}: action={:?}, new state={}", steps, action, env.position);
        render(&env);

        agent.update_table(environment.2, environment.3, environment.1, environment.4);

        if environment.0 {
            n += 1;
            env.reset();
            println!("{:?}", agent.brain);

        }

        if n == 10000 {
            println!("Steps: {steps}");
            break
        }
    }
    println!("Episode Over.");
}
