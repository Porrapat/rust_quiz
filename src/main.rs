mod quiz;
mod engine;

use std::io::{self, Write};

use quiz::quiz_bank;
use engine::{QuizState, AnswerResult};

use rand::seq::SliceRandom;
use rand::rng;

fn pick_random_quizzes(mut quizzes: Vec<quiz::Quiz>, count: usize) -> Vec<quiz::Quiz> {
    let mut rng = rng();
    quizzes.shuffle(&mut rng);
    quizzes.into_iter().take(count).collect()
}

fn select_mode() -> u8 {
    loop {
        println!();
        println!("Select mode:");
        println!("1. Random 5 questions");
        println!("2. Play all questions (in order)");
        print!("Your choice (1 or 2): ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return 1,
            "2" => return 2,
            _ => {
                println!("❌ Invalid choice, please enter 1 or 2.");
            }
        }
    }
}

fn main() {
    println!("🦀 Welcome to RustQuiz!");
    println!("-----------------------");

    let mode = select_mode();

    let all_quizzes = quiz_bank();

    let quizzes = match mode {
        1 => {
            let count = 5.min(all_quizzes.len());
            pick_random_quizzes(all_quizzes, count)
        }
        2 => all_quizzes,
        _ => unreachable!(),
    };

    let mut state = QuizState::new();

    while let Some(quiz) = state.current_quiz(&quizzes) {
        println!("\n[{}] {}", quiz.id, quiz.title);
        println!("{}", quiz.question);

        if let Some(code) = quiz.code {
            println!("\n--- code ---");
            println!("{}", code);
            println!("------------");
        }

        for (i, choice) in quiz.choices.iter().enumerate() {
            println!("  {}. {}", i + 1, choice);
        }

        print!("\nYour answer (1-{}): ", quiz.choices.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = match input.trim().parse::<usize>() {
            Ok(n) if n > 0 && n <= quiz.choices.len() => n - 1,
            _ => {
                println!("❌ Invalid input, try again.");
                continue;
            }
        };

        match state.answer(&quizzes, choice) {
            AnswerResult::Correct => {
                println!("✅ Correct!");
            }
            AnswerResult::Wrong => {
                println!("❌ Wrong!");
                println!("👉 Explanation: {}", quiz.explanation);
            }
            AnswerResult::Finished => break,
        }
    }

    println!("\n🎉 Quiz finished!");
    println!("Score: {}/{}", state.score, quizzes.len());
}
