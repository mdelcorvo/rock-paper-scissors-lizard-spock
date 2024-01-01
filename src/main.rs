//
// Rock, Paper, Scissors, Lizard, Spock
//
// A game by mdelcorvo

use std::fmt;
use std::io;
use std::str;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use colored::*;

enum RockPaperScissorsLizardSpockGuess {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

enum RockPaperScissorsLizardSpockCompare {
    RockCrushesScissors,
    PaperCoversRock,
    ScissorsCutPaper,
    RockCrushesLizard,
    LizardPoisonsSpock,
    SpockSmaschesScissors,
    ScissorsDecapitatesLizard,
    SpockVaporizesRock,
    PaperDisprovesSpock,
    LizardEatsPaper,
}

enum RockPaperScissorsLizardSpockResult {
    Win(RockPaperScissorsLizardSpockCompare),
    Loss(RockPaperScissorsLizardSpockCompare),
    Tie(String),
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

impl Compare<RockPaperScissorsLizardSpockGuess, RockPaperScissorsLizardSpockResult> for RockPaperScissorsLizardSpockGuess{
    fn compare(&self, b: &RockPaperScissorsLizardSpockGuess) -> RockPaperScissorsLizardSpockResult {
        match self {
            RockPaperScissorsLizardSpockGuess::Rock => {
                match b {
                    RockPaperScissorsLizardSpockGuess::Rock    =>
                        RockPaperScissorsLizardSpockResult::Tie(self.to_string()),
                    RockPaperScissorsLizardSpockGuess::Paper   =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::PaperCoversRock),
                    RockPaperScissorsLizardSpockGuess::Scissors =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::RockCrushesScissors),
                    RockPaperScissorsLizardSpockGuess::Lizard =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::RockCrushesLizard),
                    RockPaperScissorsLizardSpockGuess::Spock =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::SpockVaporizesRock),
                }
            }
            RockPaperScissorsLizardSpockGuess::Paper => {
                match b {
                    RockPaperScissorsLizardSpockGuess::Rock    =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::PaperCoversRock),
                    RockPaperScissorsLizardSpockGuess::Paper   =>
                        RockPaperScissorsLizardSpockResult::Tie(self.to_string()),
                    RockPaperScissorsLizardSpockGuess::Scissors =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::ScissorsCutPaper),
                    RockPaperScissorsLizardSpockGuess::Lizard =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::LizardEatsPaper),
                    RockPaperScissorsLizardSpockGuess::Spock =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::PaperDisprovesSpock),
                }
            }
            RockPaperScissorsLizardSpockGuess::Scissors => {
                match b {
                    RockPaperScissorsLizardSpockGuess::Rock    =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::RockCrushesScissors),
                    RockPaperScissorsLizardSpockGuess::Paper   =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::ScissorsCutPaper),
                    RockPaperScissorsLizardSpockGuess::Lizard =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::ScissorsDecapitatesLizard),
                    RockPaperScissorsLizardSpockGuess::Spock =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::SpockSmaschesScissors),
                    RockPaperScissorsLizardSpockGuess::Scissors =>
                        RockPaperScissorsLizardSpockResult::Tie(self.to_string()),
                }
            }
            RockPaperScissorsLizardSpockGuess::Lizard => {
                match b {
                    RockPaperScissorsLizardSpockGuess::Rock    =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::RockCrushesLizard),
                    RockPaperScissorsLizardSpockGuess::Paper   =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::LizardEatsPaper),
                    RockPaperScissorsLizardSpockGuess::Scissors =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::ScissorsDecapitatesLizard),
                    RockPaperScissorsLizardSpockGuess::Spock =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::LizardPoisonsSpock),
                    RockPaperScissorsLizardSpockGuess::Lizard =>
                        RockPaperScissorsLizardSpockResult::Tie(self.to_string()),
                }
            }
            RockPaperScissorsLizardSpockGuess::Spock => {
                match b {
                    RockPaperScissorsLizardSpockGuess::Rock    =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::SpockVaporizesRock),
                    RockPaperScissorsLizardSpockGuess::Paper   =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::PaperDisprovesSpock),
                    RockPaperScissorsLizardSpockGuess::Lizard =>
                        RockPaperScissorsLizardSpockResult::Loss(RockPaperScissorsLizardSpockCompare::LizardPoisonsSpock),
                    RockPaperScissorsLizardSpockGuess::Scissors =>
                        RockPaperScissorsLizardSpockResult::Win(RockPaperScissorsLizardSpockCompare::SpockSmaschesScissors),
                    RockPaperScissorsLizardSpockGuess::Spock =>
                        RockPaperScissorsLizardSpockResult::Tie(self.to_string()),
                }
            }
        }
    }
}

impl fmt::Display for RockPaperScissorsLizardSpockResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsLizardSpockResult::Win(result) => {
                match result {
                    RockPaperScissorsLizardSpockCompare::RockCrushesScissors => write!(f, "Rock crushes scissors"),
                    RockPaperScissorsLizardSpockCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperScissorsLizardSpockCompare::ScissorsCutPaper => write!(f, "Scissors cut paper"),
                    RockPaperScissorsLizardSpockCompare::RockCrushesLizard => write!(f, "Rock crushes lizard!"),
                    RockPaperScissorsLizardSpockCompare::LizardPoisonsSpock => write!(f, "Lizard poisons Spock!"),
                    RockPaperScissorsLizardSpockCompare::SpockSmaschesScissors => write!(f, "Spock smasches scissors!"),
                    RockPaperScissorsLizardSpockCompare::ScissorsDecapitatesLizard => write!(f, "Scissors decapitates lizard!"),
                    RockPaperScissorsLizardSpockCompare::SpockVaporizesRock => write!(f, "Spock vaporizes rock!"),
                    RockPaperScissorsLizardSpockCompare::PaperDisprovesSpock => write!(f, "Paper disproves Spock!"),
                    RockPaperScissorsLizardSpockCompare::LizardEatsPaper => write!(f, "Lizard eats paper!"),
                }
            },
            RockPaperScissorsLizardSpockResult::Loss(result) => {
                match result {
                    RockPaperScissorsLizardSpockCompare::RockCrushesScissors => write!(f, "Rock crushes scissors"),
                    RockPaperScissorsLizardSpockCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperScissorsLizardSpockCompare::ScissorsCutPaper => write!(f, "Scissors cut paper"),
                    RockPaperScissorsLizardSpockCompare::RockCrushesLizard => write!(f, "Rock crushes lizard!"),
                    RockPaperScissorsLizardSpockCompare::LizardPoisonsSpock => write!(f, "Lizard poisons Spock!"),
                    RockPaperScissorsLizardSpockCompare::SpockSmaschesScissors => write!(f, "Spock smasches scissors!"),
                    RockPaperScissorsLizardSpockCompare::ScissorsDecapitatesLizard => write!(f, "Scissors decapitates lizard!"),
                    RockPaperScissorsLizardSpockCompare::SpockVaporizesRock => write!(f, "Spock vaporizes rock!"),
                    RockPaperScissorsLizardSpockCompare::PaperDisprovesSpock => write!(f, "Paper disproves Spock!"),
                    RockPaperScissorsLizardSpockCompare::LizardEatsPaper => write!(f, "Lizard eats paper!"),
                }
            },
            RockPaperScissorsLizardSpockResult::Tie(result) => write!(f, "{result}"),
        }
    }
}

#[derive(Debug)]
enum ParseRockPaperScissorsGuessError {
    Unknown(String),
}

impl str::FromStr for RockPaperScissorsLizardSpockGuess {
    type Err = ParseRockPaperScissorsGuessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r"  | "rock"    => Ok(RockPaperScissorsLizardSpockGuess::Rock),
            "p"  | "paper"   => Ok(RockPaperScissorsLizardSpockGuess::Paper),
            "sc" | "scissors" => Ok(RockPaperScissorsLizardSpockGuess::Scissors),
            "l"  | "lizard"    => Ok(RockPaperScissorsLizardSpockGuess::Lizard),
            "sp" | "spock"   => Ok(RockPaperScissorsLizardSpockGuess::Spock),
            _   => Err(ParseRockPaperScissorsGuessError::Unknown(s.to_string())),
        }
    }
}

impl fmt::Display for RockPaperScissorsLizardSpockGuess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsLizardSpockGuess::Rock    => write!(f, "Rock"),
            RockPaperScissorsLizardSpockGuess::Paper   => write!(f, "Paper"),
            RockPaperScissorsLizardSpockGuess::Scissors => write!(f, "Scissors"),
            RockPaperScissorsLizardSpockGuess::Lizard    => write!(f, "Lizard"),
            RockPaperScissorsLizardSpockGuess::Spock   => write!(f, "Spock"),
        }
    }
}

impl Distribution<RockPaperScissorsLizardSpockGuess> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockPaperScissorsLizardSpockGuess {
        let index: u8 = rng.gen_range(0..5);
        match index {
            0 => RockPaperScissorsLizardSpockGuess::Rock,
            1 => RockPaperScissorsLizardSpockGuess::Paper,
            2 => RockPaperScissorsLizardSpockGuess::Scissors,
            3 => RockPaperScissorsLizardSpockGuess::Lizard,
            4 => RockPaperScissorsLizardSpockGuess::Spock,
            _ => unreachable!(),
        }
    }
}

fn main() {
    //
    // Game Intro
    //
    println!("\n");
    println!("Hello, I'm Dr. Sheldon Cooper, Lets play Rock, Paper, Scissors, Lizard and Spock! \n");
    println!("How about best 3 out of 5 rounds? \n");
    println!("{}", "Are you able to beat a genius?\n".italic().blue());

    let mut player_wins = 0;
    let mut comp_wins = 0;
    let mut quit = false;

    'game: loop { // game

        //
        // 'game' loop
        //

        let comp_move: RockPaperScissorsLizardSpockGuess = rand::thread_rng().gen();

        println!("Please select (r)ock, (p)aper, (sc)issors, (l)izard or (sp)ock :");

        loop { // round

            //
            // 'round' loop
            //

            let mut player_move = String::new();

            io::stdin()
                .read_line(&mut player_move)
                .expect("Failed to read move");

            let player_move: Result<RockPaperScissorsLizardSpockGuess, ParseRockPaperScissorsGuessError>
                = player_move.trim().parse();

            let player_move = match player_move {
                Ok(player_move_val) => {
                    println!("");
                    println!("You chose {}", player_move_val);
                    println!("I chose {}", comp_move);
                    player_move_val
                },
                Err(ParseRockPaperScissorsGuessError::Unknown(s)) => {
                    match &s[..] {
                        "q" | "quit" => {
                            println!("Quit? Ahah.");
                            quit = true;
                            break 'game;
                        },
                        _            => {
                            println!("\"{}\" is not a valid guess, try again.\n",s); // TODO Figure out how to name the character here.
                            continue
                        },
                    }
                },
            };

            //
            // Report Round Results
            //

            let result: RockPaperScissorsLizardSpockResult = player_move.compare(&comp_move);
            match result {
                RockPaperScissorsLizardSpockResult::Win(_) => {
                    player_wins += 1;
                    println!("{}", result);
                    println!("You won this round.");
                },
                RockPaperScissorsLizardSpockResult::Tie(_) => println!("Tie..."),
                RockPaperScissorsLizardSpockResult::Loss(_) => {
                    comp_wins += 1;
                    println!("{}", result);
                    println!("You lost this round.");
                },
            }

            break;
        }

        //
        // Win Check
        //
        println!("");
        if player_wins == 3 {
            println!("{}","Congratulations, You won the game!\n".italic().green());
            break;
        } else if comp_wins == 3 {
            println!("Congratulations, You won.... \n");
            println!("{}", "Bazinga!! You lost the game! \n".italic().yellow());
            break;
        } else {
            println!("You have {} wins, and I have {} wins.\n", player_wins, comp_wins);
        }
    }

    if quit == true {
        println!("Well... thanks for playing.  Sorry you had to leave so soon.\n");
        println!("{}", "Hey!! That's My Spot".red());
    }
}