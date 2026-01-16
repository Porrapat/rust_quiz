#![allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Intro,
    Beginner,
    BeginnerPlus,
    Intermediate,
}

#[derive(Clone, Debug)]
pub struct Quiz {
    pub id: u32,
    pub title: &'static str,
    pub question: &'static str,
    pub code: Option<&'static str>,
    pub choices: Vec<&'static str>,
    pub correct: usize,
    pub explanation: &'static str,
    pub tags: Vec<&'static str>,
    pub level: Level,
}

pub fn quiz_bank() -> Vec<Quiz> {
    vec![
        Quiz {
            id: 1,
            title: "Rust source file extension",
            question: "Which file extension is used for a Rust source code file?",
            code: None,
            choices: vec![
                ".rt",
                ".rust",
                ".rst",
                ".rs",
            ],
            correct: 3,
            explanation: "In rust we use \".rs\" when creating a source code file (such as \"main.rs\").",
            tags: vec!["basics", "files"],
            level: Level::Intro,
        },

        Quiz {
            id: 2,
            title: "Mutable variables in Rust",
            question: "Which keyword is used to make a variable mutable in Rust?",
            code: None,
            choices: vec![
                "&",
                "let",
                "const",
                "mut",
            ],
            correct: 3,
            explanation: "In Rust, variables are immutable by default. To make a variable mutable, you must use the `mut` keyword, for example: `let mut x = 5;`.",
            tags: vec!["basics", "mutability"],
            level: Level::Intro,
        },
        Quiz {
            id: 3,
            title: "String Length and Shadowing",
            question: "What will the following code output in Rust?",
            code: Some(
                "let chars = \"I love Rust!\";\nlet chars = chars.len();\nprintln!(\"Total chars: {}\", chars);"
            ),
            choices: vec![
                "Total chars: 10",
                "Total chars: 12",
                "error[E0308]: mismatched types",
                "error: argument never used",
            ],
            correct: 1,
            explanation: "Thanks to variable name shadowing, we could use the exact same variable name with a different datatype, which can be very convenient in certain contexts.",
            tags: vec!["shadowing", "string", "len", "basics"],
            level: Level::Beginner,
        },
        Quiz {
            id: 4,
            title: "Integer Range Limits (i8)",
            question: "What will the following code output in Rust?",
            code: Some(
                "let n: i8 = 1_000;\nprintln!(\"n={}\", n);"
            ),
            choices: vec![
                "n=1000",
                "n=1_000",
                "error: literal out of range for `i8`", // Correct choice
                "None of the above",
            ],
            correct: 2, // Index of "error: literal out of range for `i8`"
            explanation: "Since we told Rust that we want \"n\" to be of only 8 bits, \"n\" can only contain a value between -128 to 127. If we want a bigger value we're going to have to use i16.",
            tags: vec!["integers", "i8", "data types", "range"],
            level: Level::Beginner,
        },
        Quiz {
            id: 5,
            title: "Default Integer Type Inference",
            question: "What is the default type of \"x\" in Rust for:\n\nlet x = 5;",
            code: Some("let x = 5;"),
            choices: vec![
                "i8",
                "u8",
                "i32", // Correct choice
                "i64",
            ],
            correct: 2, // Index of "i32"
            explanation: "If you don't specify a type, Rust defaults to \"i32\" for integers.",
            tags: vec!["integers", "data types", "type inference", "i32"],
            level: Level::Beginner,
        },
        Quiz {
            id: 6,
            title: "Centering with Formatting Specifiers",
            question: "What will the following code output in Rust?",
            code: Some(
                "let var = \"Rust\";\nprintln!(\"{var:*^10}\");",
            ),
            choices: vec![
                "^^^Rust^^^",
                "***Rust***", // Correct choice in the image
                "******Rust",
                "Rust******",
            ],
            correct: 1, // Index of "***Rust***"
            explanation: "Just like in Python, we can use this format specifier to tell Rust that we want to centre an element using \"^\" with the symbol \"*\" and that we want that to occupy a total of 10 spaces.",
            tags: vec!["formatting", "println", "macros", "specifiers"],
            level: Level::BeginnerPlus,
        },
        Quiz {
            id: 7,
            title: "Array Initialization and Debug Print",
            question: "What will the following code output in Rust?",
            code: Some(
                "let unknown = [\"Rust\"; 3];\nprintln!(\"{:?}\", unknown);",
            ),
            choices: vec![
                "\"t\"",
                "(\"Rust\", \"Rust\", \"Rust\")",
                "[\"Rust\", \"Rust\", \"Rust\"]", // Correct choice
                "error[E0277]",
            ],
            correct: 2, // Index of "[\"Rust\", \"Rust\", \"Rust\"]"
            explanation: "In Rust you can use the following syntax to create an array with repeating elements: \"let arr = [element; n];\". To print an array we need to use the \"{:?}\" debug format specifier.",
            tags: vec!["arrays", "debug", "formatting", "syntax"],
            level: Level::Beginner,
        }, 
        Quiz {
            id: 8,
            title: "If Expression Assignment",
            question: "What will the following code output in Rust?",
            code: Some(
                "let input = \"Rust\";\nlet text = if input.len() == 4 {\"YES\"} else {\"NO\"};\nprintln!(\"{}\", text)"
            ),
            choices: vec![
                "YES", // Correct choice
                "NO",
                "Rust",
                "error: could not compile",
            ],
            correct: 0, // Index of "YES"
            explanation: "In Rust we can create a \"let if\" expression using the following syntax: \"let var = if condition {value1} else {value2};\". If the condition evaluates to true, the first value will be assigned to the variable, else the second one will.",
            tags: vec!["if", "expression", "control flow", "string", "len"],
            level: Level::BeginnerPlus,
        },
        Quiz {
            id: 9,
            title: "Function Definition and Return Type",
            question: "What is the correct way to define a function that returns an integer in Rust?",
            code: None,
            choices: vec![
                "fn add() -> i32 { 5 }", // Correct choice
                "function add() { return 5; }",
                "def add() -> int { 5 }",
                "fn add { 5 }",
            ],
            correct: 0, 
            explanation: "Rust functions are defined with \"fn\", and \"-> i32\" means the function returns a 32-bit integer.",
            tags: vec!["function", "syntax", "i32", "return"],
            level: Level::Beginner,
        },
        Quiz {
            id: 10,
            title: "Ownership and Move Semantics",
            question: "What will the following code output in Rust?",
            code: Some(
                "let original = String::from(\"Bob\");\nlet copy = original;\nprintln!(\"1. {}, 2. {}\", original, copy);"
            ),
            choices: vec![
                "1. {original}, 2. {copy}",
                "1. Bob, 2. Bob",
                "1. original, 2. copy",
                "Error", // Correct choice
            ],
            correct: 3, 
            explanation: "Running this code will give you the following error: \"error[E0382]: borrow of moved value: `original`\". I will cover this in detail in a future lesson, but for now, know that what we did here is called a 'move'. We moved the data from the variable \"original\" to another variable, misleadingly named \"copy.\" Rust invalidates the \"original\" variable after this operation to ensure memory safety, so \"original\" is no longer valid, and \"copy\" essentially replaces it.",
            tags: vec!["ownership", "move", "string", "error"],
            level: Level::Intermediate,
        },
        Quiz {
            id: 11,
            title: "String Length (Bytes vs. Characters)",
            question: "What will the following code output in Rust?",
            code: Some("println!(\"{}\", \"äöå\".len());"),
            choices: vec![
                "3",
                "5",
                "6", // Correct choice
                "Error",
            ],
            correct: 2, 
            explanation: "\"len()\" in Rust returns the length in bytes, not of the total amount of characters. If you want the latter you'd have to use: \".chars().count()\". \"öäå\" are special characters that contain 2 bytes each, that's why we get 6 as a return.",
            tags: vec!["string", "len", "unicode", "bytes"],
            level: Level::BeginnerPlus,
        },
        Quiz {
            id: 12,
            title: "Loop Expression Return Value",
            question: "What will the following code output in Rust?",
            code: Some(
                "let mut count = 0;
let result = loop {
    count += 2;
    if count > 5 {
        break count * 2;
    }
};
println!(\"{}\", result);"
            ),
            choices: vec![
                "6",
                "8",
                "10",
                "12", // Correct choice
            ],
            correct: 3, 
            explanation: "We start the count at 0 and loop until \"count\" increments to 6. Once \"count\" reaches 6, it triggers our \"count > 5\" condition and breaks out of the loop. Since we included \"count * 2\" after the break statement, that value is assigned to \"result,\" which is why we end up with an output of 12.",
            tags: vec!["loop", "expression", "break", "control flow"],
            level: Level::Intermediate,
        },
        Quiz {
            id: 13,
            title: "Custom Fill Character Formatting",
            question: "What will the following code output in Rust?",
            code: Some("let name = \"Bob\";\nprintln!(\"{name:~^9}\");"),
            choices: vec![
                "^^^Bob^^^",
                "~~~Bob~~~", // Correct choice
                "Bob~~~~~~",
                "Bob^^^^^^",
            ],
            correct: 1, 
            explanation: "This is one of my favourite styling format specifiers that I use in Python (and I'm happy to see that Rust has it as well). Here's how it works: \"name\" is the variable we want to use. \"~\" is the fill character. \"^\" centres the variable; you can also left-align or right-align using \"<\" or \">\", respectively. \"9\" specifies the total width to reserve for the variable, including its own length.",
            tags: vec!["formatting", "println", "specifiers", "fill"],
            level: Level::BeginnerPlus,
        },
        Quiz {
            id: 14,
            title: "Ownership Move on Function Call",
            question: "What will the following code output in Rust?",
            code: Some(
                "fn main() {
\tlet name = String::from(\"Bob\");
\tgreet(name);
\tgreet(name);
}

fn greet(name: String) {
\tprintln!(\"Hello, {}!\", name);
}"
            ),
            choices: vec![
                "Hello, Bob! Hello, Bob!",
                "Hello, BobBob!",
                "Error", // Correct choice
                "None of the above",
            ],
            correct: 2, 
            explanation: "Running this code will give you the following error: \"error[E0382]: use of moved value: `name`\". The reason is because we created an owned string using the \"String::from\" syntax and that means that we need to be careful where we use it if we don't want to lose ownership. A better solution would be to define our function to accept a string slice: \"fn greet(name: &str)\". Then we would be forced to pass in \"&name\".",
            tags: vec!["ownership", "move", "function", "string"],
            level: Level::Intermediate,
        },
        Quiz {
            id: 15,
            title: "Struct Instantiation",
            question: "What would be the correct way to instantiate this struct in Rust?\n\nstruct Rectangle { width: u32, height: u32 }",
            code: Some(
                "struct Rectangle {
    width: u32,
    height: u32,
}"
            ),
            choices: vec![
                "Rectangle(10, 20);",
                "Rectangle { width: 10, height: 20 };", // Correct choice (assuming second option in image is the same as the third but selected)
                "Rectangle(width:10, height:20);",
                "Rectangle {10, 20};",
            ],
            correct: 1, // Index of the correct curly-brace syntax
            explanation: "To create an instance of \"Rectangle\" we must provide the values for both the width and the height inside curly brackets using the field names and colons.",
            tags: vec!["struct", "syntax", "instantiation"],
            level: Level::Beginner,
        },
        Quiz {
            id: 16,
            title: "Option::unwrap() on None",
            question: "What will happen if we try to run this code in Rust?",
            code: Some(
                "let value: Option<u8> = None;\nlet unwrapped = value.unwrap();"
            ),
            choices: vec![
                "The program will print '0' to the console.",
                "The program will print 'None' to the console.",
                "The program will panic at runtime.", // Correct choice
                "The program will compile and run successfully without output.",
            ],
            correct: 2, 
            explanation: "When you call \".unwrap()\" on an Option-type that is \"None\", Rust will panic and terminate the program. If you want to provide a default value when unwrapping an Option-type, I'd recommend using: \"value.unwrap_or(0);\". This will use the default value of \"0\" in the case where value ends up being \"None\".",
            tags: vec!["option", "panic", "unwrap", "error handling"],
            level: Level::BeginnerPlus,
        },
        Quiz {
            id: 17,
            title: "Advantage of Option<T>",
            question: "In Rust, what is the main advantage of using `Option<T>` over using null values?",
            code: None,
            choices: vec![
                "It makes your code run faster.",
                "It eliminates null reference errors.", // Correct choice
                "It automatically fills in missing values with zero.",
                "It allows you to save on memory.",
            ],
            correct: 1, 
            explanation: "In Rust, you can't use a value that might be \"null\" without explicitly handling the possibility first. This makes it much harder to accidentally use a missing value, reducing the risk of runtime errors.",
            tags: vec!["option", "null", "error handling", "safety"],
            level: Level::BeginnerPlus,
        },
        Quiz {
            id: 18,
            title: "Panic Macro Syntax",
            question: "Which of these is the correct way to trigger a panic in Rust?",
            code: None,
            choices: vec![
                "panic(\"Something went wrong!\")",
                "throw!(\"Something went wrong!\")",
                "raise!(\"Something went wrong!\")",
                "panic!(\"Something went wrong!\")", // Correct choice
            ],
            correct: 3, 
            explanation: "Remember, \"panic!\" is a macro and requires the \"!\" to run.",
            tags: vec!["panic", "macro", "syntax", "error handling"],
            level: Level::Beginner,
        },
        Quiz {
            id: 19,
            title: "String Slicing (Byte Indexing)",
            question: "Which of the following code snippets will allow you to access \"5\" from the following string in Rust?",
            code: Some("let s = \"Bob 5\";"),
            choices: vec![
                "&s[4..5]", // Correct choice
                "&s[5]",
                "&s[4]",
                "&s.char_at(4)",
            ],
            correct: 0, 
            explanation: "In Rust, you can't index a string directly by character because strings are UTF-8 encoded and characters can be multiple bytes. However, you can slice a string using byte ranges — as long as those byte boundaries align with valid UTF-8 characters.",
            tags: vec!["string", "slice", "indexing", "byte"],
            level: Level::Intermediate,
        },
        Quiz {
            id: 20,
            title: "Integer Copy Trait",
            question: "What will happen when we run this code in Rust?",
            code: Some(
                "let x = 42;
let y = x;
dbg!(x, y);"
            ),
            choices: vec![
                "Only 'y' is printed, since 'x' is moved.",
                "The program will panic.",
                "Both 'x' and 'y' are displayed correctly using 'dbg!'", // Correct choice
                "error[E0382]: use of moved value: 'x'",
            ],
            correct: 2, 
            explanation: "In Rust, integers implement the `Copy` trait. When you assign `x` to `y`, the value is copied, not moved, so both `x` and `y` remain usable. The `dbg!()` macro prints both values along with file and line number information.",
            tags: vec!["copy", "move", "ownership", "integer", "trait"],
            level: Level::Intermediate,
        }
            
    ]
}
