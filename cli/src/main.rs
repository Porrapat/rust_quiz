use dialoguer::Select;

use colored::*;
use rust_quiz::engine::{AnswerResult, QuizState};
use rust_quiz::quiz;
use rust_quiz::quiz::quiz_bank;

use rand::rng;
use rand::seq::SliceRandom;

enum GameMode {
    Rand5, // Random 5 questions
    All,   // Play all questions
}

fn pick_random_quizzes(mut quizzes: Vec<quiz::Quiz>, count: usize) -> Vec<quiz::Quiz> {
    let mut rng = rng();
    quizzes.shuffle(&mut rng);
    quizzes.into_iter().take(count).collect()
}

fn select_mode() -> GameMode {
    let items = vec!["Random 5 questions", "Play all questions (in order)"];

    let selection = Select::new()
        .with_prompt("Select mode")
        .items(&items)
        .interact()
        .unwrap();

    match selection {
        0 => GameMode::Rand5,
        1 => GameMode::All,
        _ => unreachable!(),
    }
}

fn main() {
    println!(
        r#"
               🦀 Welcome to
    ____             __                 _    
   / __ \__  _______/ /_   ____ ___  __(_)___
  / /_/ / / / / ___/ __/  / __ `/ / / / /_  /
 / _, _/ /_/ (__  ) /_   / /_/ / /_/ / / / /_
/_/ |_|\__,_/____/\__/   \__, /\__,_/_/ /___/
                           /_/               
    "#
    );
    println!(
        "{}\n",
        "Use arrow keys to navigate between choices"
            .yellow()
            .bold()
            .dimmed()
    );

    let mode = select_mode();

    let all_quizzes = quiz_bank();

    let quizzes = match mode {
        GameMode::Rand5 => {
            let count = 5.min(all_quizzes.len());
            pick_random_quizzes(all_quizzes, count)
        }
        GameMode::All => all_quizzes,
    };

    let mut state = QuizState::new();

    clearscreen::clear().unwrap_or_else(|_| {
        print!("\x1B[2J\x1B[1;1H");
    });

    while let Some(quiz) = state.current_quiz(&quizzes) {
        println!(
            "{} [{}/{}]",
            "Question".bright_black(),
            quiz.id,
            quizzes.len()
        );
        println!("{} {}", "Title:".green().bold(), quiz.title);
        println!("{} {}\n", "Question:".yellow().bold(), quiz.question);

        if let Some(code) = quiz.code {
            println!("{}\n", code.dimmed().italic());
        }

        let choice = Select::new().items(&quiz.choices).interact().unwrap();

        match state.answer(&quizzes, choice) {
            AnswerResult::Correct => {
                println!("{}", "Correct answer".green());
            }
            AnswerResult::Wrong => {
                println!("{}", "Wrong answer".red());
                println!(
                    "{} \"{}\"\n",
                    "Correct answer is:".cyan(),
                    quiz.choices[quiz.correct]
                );
            }
            AnswerResult::Finished => break,
        }

        println!("{} {}", "Explanation:".cyan().bold(), quiz.explanation);

        println!("{}", "\nPress Enter to continue...".dimmed());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        clearscreen::clear().unwrap_or_else(|_| {
            print!("\x1B[2J\x1B[1;1H");
        });
    }

    println!("\n🎉 Quiz finished!");
    println!("Score: {}/{}", state.score, quizzes.len());
    if state.score > quizzes.len() / 2 {
        println!("{}", "Great job!".green());
    } else {
        println!("{}", "Better luck next time.".yellow());
    }
}
