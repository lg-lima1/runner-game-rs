use rand::prelude::*;
use std::io;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn from(arg: &str) -> Result<Difficulty, &str> {
        let arg = arg.to_lowercase();
        match &arg[..] {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            _ => Err("not a difficulty"),
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Difficulty::Easy => 2,
            Difficulty::Medium => 3,
            Difficulty::Hard => 4,
        }
    }
}

pub struct Config {
    difficulty: Difficulty,
}

pub fn config(args: &[String]) -> Result<Config, &str> {
    fn print_args() {
        println!(
            "\
Usage:
\trunner_game [difficulty]
-----
Where:
\tdifficulty - Easy, Medium, Hard
"
        );
    }

    if args.len() < 2 {
        print_args();
        return Err("not enough arguments");
    }

    if args.len() > 2 {
        print_args();
        return Err("too much arguments");
    }

    let difficulty = Difficulty::from(&args[1])?;

    Ok(Config { difficulty })
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Divide(i32),
}

pub struct Engine {
    soldiers: i32,
    bad_soldiers: i32,
    difficulty: Difficulty,
}

impl Engine {
    pub fn new(difficulty: Difficulty) -> Engine {
        let mut rng = thread_rng();
        let soldiers = rng.gen_range(10..20);
        let bad_soldiers = rng.gen_range((soldiers + 1)..(soldiers * 2));

        println!("Your army has {} soldiers.", soldiers);
        println!(
            "Your army encountered {} bad soldiers. \u{1FA96}",
            bad_soldiers
        );

        Engine {
            soldiers,
            bad_soldiers,
            difficulty,
        }
    }

    fn add(&self, should_win: bool) -> Operation {
        let mut rng = thread_rng();
        let test = self.bad_soldiers - self.soldiers; // 10 = 20 - 10
        let value = if should_win {
            rng.gen_range((test + 1)..=(test * 5)) // 11 <= x <= 50
        } else {
            rng.gen_range((test * -5)..test) // -50 <= x < 10
        };
        Operation::Add(value)
    }

    fn subtract(&self, should_win: bool) -> Operation {
        let mut rng = thread_rng();
        let test = self.soldiers - self.bad_soldiers; // -10 =  10 - 20
        let value = if should_win {
            rng.gen_range((test * 5)..=(test - 1)) // -50 <= x <= -11
        } else {
            rng.gen_range(test..(test * -5)) // -10 <= x < 50
        };
        Operation::Subtract(value)
    }

    fn multiply(&self, should_win: bool) -> Operation {
        let mut rng = thread_rng();
        let test = self.bad_soldiers as f32 / self.soldiers as f32; // 2 = 20 / 10
        let test = test.ceil() as i32;
        let value = if should_win {
            rng.gen_range((test + 1)..=(test + 10)) // 3 <= x <= 12
        } else {
            rng.gen_range((test - 10)..test) // -8 <= x < 2
        };
        Operation::Multiply(value)
    }

    fn divide_to_lose(&self) -> Operation {
        let mut rng = thread_rng();
        let test = self.bad_soldiers as f32 / self.soldiers as f32; // 2 = 20 / 10
        let test = test.ceil() as i32;
        let value = rng.gen_range((test + 1)..=(test + 10)); // 3 <= x <= 12
        Operation::Divide(value)
    }

    fn operation(&self, should_win: bool) -> Operation {
        let mut rng = thread_rng();
        if rng.gen() {
            if rng.gen() {
                self.add(should_win)
            } else {
                self.subtract(should_win)
            }
        } else {
            if should_win {
                self.multiply(true)
            } else {
                if rng.gen() {
                    self.multiply(false)
                } else {
                    self.divide_to_lose()
                }
            }
        }
    }

    fn randomize_paths(&self) -> Vec<Operation> {
        let size = self.difficulty.to_usize();

        let mut paths = vec![self.operation(false); size - 1];

        let mut rng = thread_rng();
        let index = rng.gen_range(0..size);
        paths.insert(index, self.operation(true));
        paths
    }

    pub fn select_operation(&self) -> Result<Operation, &str> {
        let paths = self.randomize_paths();

        println!("Which path will you take?");
        println!("1:\t{:?}", paths[0]);
        println!("2:\t{:?}", paths[1]);

        if paths.len() == Difficulty::Medium.to_usize() {
            println!("3:\t{:?}", paths[2]);
        }
        if paths.len() == Difficulty::Hard.to_usize() {
            println!("3:\t{:?}", paths[2]);
            println!("4:\t{:?}", paths[3]);
        }

        let mut option = String::new();
        if let Err(_) = io::stdin().read_line(&mut option) {
            return Err("Failed to read line.");
        }

        let option = option.trim_end().parse::<usize>();
        let option = match option {
            Ok(x) => x,
            Err(_) => return Err("Failed to parse input number."),
        };

        let chosen_path = paths.get(option - 1);
        let chosen_path = match chosen_path {
            Some(x) => x,
            None => return Err("Index out of range"),
        };

        Ok(chosen_path.clone())
    }

    pub fn apply_operation(&mut self, operation: Operation) {
        match operation {
            Operation::Add(val) => self.soldiers += val,
            Operation::Subtract(val) => self.soldiers -= val,
            Operation::Multiply(val) => self.soldiers *= val,
            Operation::Divide(val) => self.soldiers /= val,
        }
    }

    pub fn fight_war(&mut self) -> bool {
        self.soldiers -= self.bad_soldiers;
        if self.soldiers > 0 {
            println!(
                "You won! Now your army have {} soldiers. \u{1F973}",
                self.soldiers
            );
        } else {
            println!("You lost! You should have taken another path. \u{2620}");
        }
        self.soldiers <= 0
    }

    pub fn new_encounter(&mut self) {
        let mut rng = thread_rng();
        self.bad_soldiers = rng.gen_range((self.soldiers + 1)..((self.soldiers + 1) * 2));

        println!("As you walk through the field, another enemy group approaches! \u{1F630} \u{1F630} \u{1F630}");
        println!(
            "Your army encountered {} bad soldiers. \u{1FA96}",
            self.bad_soldiers
        );
    }
}

pub fn run(config: Config) -> Result<(), String> {
    let mut game = Engine::new(config.difficulty);
    loop {
        let operation = game.select_operation()?;
        game.apply_operation(operation);
        let did_lose = game.fight_war();
        if did_lose {
            return Err("Game lost! Try again.".to_string());
        }
        game.new_encounter();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_game() {
        let game = Engine::new(Difficulty::Easy);
        assert!(game.soldiers >= 10 && game.soldiers < 20);
        assert!(game.bad_soldiers > game.soldiers && game.bad_soldiers < game.soldiers * 2);
    }

    #[test]
    fn new_game_easy() {
        let game = Engine::new(Difficulty::Easy);
        let paths = game.randomize_paths();
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn new_game_medium() {
        let game = Engine::new(Difficulty::Medium);
        let paths = game.randomize_paths();
        assert_eq!(paths.len(), 3);
    }

    #[test]
    fn new_game_hard() {
        let game = Engine::new(Difficulty::Hard);
        let paths = game.randomize_paths();
        assert_eq!(paths.len(), 4);
    }

    #[test]
    fn winable_path() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.operation(true);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers > 0);
    }

    #[test]
    fn losable_path() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.operation(false);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers <= 0);
    }

    #[test]
    fn add_to_win() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.add(true);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers > 0);
    }

    #[test]
    fn add_to_lose() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.add(false);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers <= 0);
    }

    #[test]
    fn subtract_to_win() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.subtract(true);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers > 0);
    }

    #[test]
    fn subtract_to_lose() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.subtract(false);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers <= 0);
    }

    #[test]
    fn multiply_to_win() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.multiply(true);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers > 0);
    }

    #[test]
    fn multiply_to_lose() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.multiply(false);
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers <= 0);
    }

    #[test]
    fn divide_to_lose() {
        let mut game = Engine::new(Difficulty::Easy);
        let operation = game.divide_to_lose();
        println!("operation: {:?}", operation);
        game.apply_operation(operation);
        println!("soldiers: {}", game.soldiers);
        assert!(game.soldiers - game.bad_soldiers <= 0);
    }
}
