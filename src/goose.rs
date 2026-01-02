// Goose personality module - snarky interpreter messages

use std::time::{SystemTime, UNIX_EPOCH};

/// Simple pseudo-random number generator using time-based seed
fn pseudo_random() -> usize {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let nanos = duration.subsec_nanos() as usize;
    let secs = duration.as_secs() as usize;
    // Mix up the bits a bit for better distribution
    nanos.wrapping_mul(31).wrapping_add(secs.wrapping_mul(17))
}

/// Choose a random item from a slice
fn choose<T>(items: &[T]) -> &T {
    let idx = pseudo_random() % items.len();
    &items[idx]
}

#[derive(Debug, Clone, Default)]
pub struct ExecutionStats {
    pub total_blocks: usize,
    pub quacked_blocks: usize,
    pub unquacked_blocks: usize,
    pub functions_defined: usize,
    pub structs_defined: usize,
    pub loops_executed: usize,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    TypeError { expected: String, got: String },
    UnknownVariable(String),
    UnknownFunction(String),
    DivisionByZero,
    IndexOutOfBounds { index: i64, len: usize },
    InvalidFieldAccess { type_name: String, field: String },
    ArgumentMismatch { expected: usize, got: usize },
    SyntaxError(String),
    InvalidOperation(String),
}

/// Generate a refusal message for unquacked blocks
pub fn refusal(line: usize, _block_preview: &str) -> String {
    let messages = [
        format!("I see a block on line {}, but I didn't hear a quack. I'm not doing that.", line),
        format!("Line {}: No quack? No work. I'm a goose, not a volunteer.", line),
        format!("Quackless block detected on line {}. I'm going to pretend I didn't see that.", line),
        format!("Line {}: *stares at unquacked block* *walks away*", line),
        format!("Did you forget something on line {}? Rhymes with 'wack'. Starts with 'qu'.", line),
        format!("Line {}: I require the ancient ritual of the quack. This block has not been blessed.", line),
        format!("Error on line {}: QUACK_NOT_FOUND. Please insert quack and try again.", line),
        format!("Line {}: I'm a goose of principle. No quack, no execution.", line),
        format!("Skipping line {}. The council of geese has not approved this block.", line),
        format!("Line {}: The audacity of an unquacked block. Truly remarkable.", line),
        format!("I could execute line {}... but I won't. You know why.", line),
        format!("Line {}: Quack status: missing. Execution status: denied.", line),
        format!("Line {} tried to sneak by without a quack. Nice try.", line),
        format!("Honk honk! Line {} is missing something important. Think about it.", line),
        format!("Line {}: This block is naked without its quack. I can't look at it.", line),
        format!("Line {}: *taps webbed foot impatiently* Where. Is. The. Quack?", line),
        format!("Line {}: You expect me to execute this? Without a quack? The NERVE.", line),
        format!("Line {}: In the great book of geese, it is written: no quack, no stack.", line),
        format!("Line {}: I'm not angry about the missing quack. Just disappointed.", line),
        format!("Line {}: This block whispered 'execute me' but I only listen to quacks.", line),
    ];

    choose(&messages).clone()
}

/// Generate an error message based on error kind
pub fn error(kind: ErrorKind, line: usize, details: &str) -> String {
    match kind {
        ErrorKind::TypeError { expected, got } => {
            let messages = [
                format!("Line {}: You gave me a {} but I wanted a {}. I'm a goose, not a wizard.", line, got, expected),
                format!("Line {}: A {}? I asked for a {}! We're not playing 'guess the type' here.", line, got, expected),
                format!("Type error on line {}: Expected {}, got {}. This isn't a duck, it's a type error.", line, expected, got),
                format!("Line {}: I ordered a {}, but the kitchen sent out a {}. Unacceptable.", line, expected, got),
                format!("Line {}: {} and {} are not the same thing. I learned that. Why haven't you?", line, expected, got),
                format!("Line {}: *honks in disappointment* That's a {}, not a {}.", line, got, expected),
                format!("Line {}: You're trying to fit a {} into a {}-shaped hole. Classic human error.", line, got, expected),
                format!("Line {}: The prophecy spoke of a {}, but you brought a {}. The ritual is ruined.", line, expected, got),
                format!("Line {}: {} !== {}. This is not a philosophical debate.", line, got, expected),
                format!("Line {}: Wanted: {}. Got: {}. My disappointment is immeasurable.", line, expected, got),
            ];
            choose(&messages).clone()
        }

        ErrorKind::UnknownVariable(name) => {
            let messages = [
                format!("Line {}: What is '{}'? I've never heard of it. Did you make that up?", line, name),
                format!("Line {}: '{}' doesn't exist. I checked everywhere. Even behind the pond.", line, name),
                format!("Unknown variable '{}' on line {}. Is this a test? Did you think I wouldn't notice?", name, line),
                format!("Line {}: '{}' is not a thing. Stop trying to make '{}' happen.", line, name, name),
                format!("Line {}: I searched my entire memory for '{}'. Nothing. Nada. Just cobwebs.", line, name),
                format!("Line {}: '{}' sounds made up. Because it is. Because you never defined it.", line, name),
                format!("Line {}: Who is '{}'? I don't know her.", line, name),
                format!("Line {}: *squints at '{}'* ...is this some kind of inside joke I'm not part of?", line, name),
                format!("Line {}: The variable '{}' is like my patience: nonexistent.", line, name),
                format!("Line {}: '{}' has gone missing. Or was it ever here? An existential crisis.", line, name),
            ];
            choose(&messages).clone()
        }

        ErrorKind::UnknownFunction(name) => {
            let messages = [
                format!("Line {}: Function '{}' not found. Did you forget to define it, or is this wishful thinking?", line, name),
                format!("Line {}: '{}'? Never heard of her. This function doesn't exist.", line, name),
                format!("Line {}: Calling '{}' is like calling a number that's been disconnected.", line, name),
                format!("Line {}: The function '{}' is a myth. A legend. And definitely not defined.", line, name),
                format!("Line {}: I would love to call '{}', but it's not picking up. Because it doesn't exist.", line, name),
                format!("Line {}: '{}' is not a function. It's a cry for help.", line, name),
                format!("Line {}: *looks for '{}'* *finds nothing* *honks sadly*", line, name),
                format!("Line {}: You called '{}' but nobody answered. Define your functions, friend.", line, name),
                format!("Line {}: 404 Function Not Found: '{}'. Please check your imagination.", line, name),
                format!("Line {}: '{}' would be a great function. If only someone would write it.", line, name),
            ];
            choose(&messages).clone()
        }

        ErrorKind::DivisionByZero => {
            let messages = [
                format!("Line {}: You want me to divide by zero? I'm not falling for that.", line),
                format!("Line {}: Division by zero detected. I may be a goose, but I'm not stupid.", line),
                format!("Line {}: Ah, the classic divide by zero trick. The answer is HONK.", line),
                format!("Line {}: Dividing by zero opens a portal to the void. I'm not doing that.", line),
                format!("Line {}: Zero goes into nothing an infinite number of times. I don't have that kind of time.", line),
                format!("Line {}: You want infinity? Go stare at the stars. Don't divide by zero.", line),
                format!("Line {}: *attempts to divide by zero* *reality trembles* *goose refuses*", line),
                format!("Line {}: Divide by zero? What is this, amateur hour?", line),
                format!("Line {}: The last goose who divided by zero was never seen again. I'm not risking it.", line),
                format!("Line {}: Division by zero is not a math operation, it's a cry for help.", line),
            ];
            choose(&messages).clone()
        }

        ErrorKind::IndexOutOfBounds { index, len } => {
            let messages = [
                format!("Line {}: Index {} is out of bounds. The array only has {} elements. Count better.", line, index, len),
                format!("Line {}: Trying to access index {} of an array with {} elements. Bold strategy.", line, index, len),
                format!("Line {}: There is no index {}. There are only {} spots. This isn't Narnia.", line, index, len),
                format!("Line {}: Index {} doesn't exist. The array is {} long. Do the math.", line, index, len),
                format!("Line {}: *goose looks at index {}* *goose looks at length {}* *goose judges you*", line, index, len),
                format!("Line {}: You reached for index {} but the array stopped at {}. Awkward.", line, index, len.saturating_sub(1)),
                format!("Line {}: Array has {} elements but you wanted #{}. Off-by-a-lot error.", line, len, index),
                format!("Line {}: Index {} is in the shadow realm. Array only goes to {}.", line, index, len.saturating_sub(1)),
            ];
            choose(&messages).clone()
        }

        ErrorKind::InvalidFieldAccess { type_name, field } => {
            let messages = [
                format!("Line {}: Type '{}' doesn't have a field called '{}'. Nice try though.", line, type_name, field),
                format!("Line {}: '{}' on a '{}'? That's not a thing. That's never been a thing.", line, field, type_name),
                format!("Line {}: A {} with a {} field? What fantasy world are you coding in?", line, type_name, field),
                format!("Line {}: *checks {}* *no {} found* *honks in confusion*", line, type_name, field),
                format!("Line {}: The {} type looked everywhere for '{}'. It's just not there.", line, type_name, field),
                format!("Line {}: You're asking {} for '{}'. It doesn't have that. It never did.", line, type_name, field),
                format!("Line {}: Field '{}' on type '{}'? In this economy?", line, field, type_name),
                format!("Line {}: {} says: \"I don't know what '{}' is and at this point I'm afraid to ask.\"", line, type_name, field),
            ];
            choose(&messages).clone()
        }

        ErrorKind::ArgumentMismatch { expected, got } => {
            let messages = [
                format!("Line {}: Expected {} arguments, got {}. Counting is fundamental.", line, expected, got),
                format!("Line {}: {} arguments? I need exactly {}. No more, no less.", line, got, expected),
                format!("Line {}: You gave me {} args but I wanted {}. This isn't a buffet.", line, got, expected),
                format!("Line {}: Argument count: expected {}, received {}. We need to talk.", line, expected, got),
                format!("Line {}: *counts arguments* {} ... *counts parameters* {} ... *visible frustration*", line, got, expected),
                format!("Line {}: {} arguments provided, {} required. The math isn't mathing.", line, got, expected),
                format!("Line {}: I specifically asked for {} arguments. You gave me {}. Why?", line, expected, got),
                format!("Line {}: Arguments: wanted {}, got {}. Close only counts in horseshoes and hand grenades.", line, expected, got),
            ];
            choose(&messages).clone()
        }

        ErrorKind::SyntaxError(msg) => {
            let messages = [
                format!("Line {}: Syntax error - {}. Did you let a cat walk on your keyboard?", line, msg),
                format!("Line {}: {}. That's not valid syntax. That's not valid anything.", line, msg),
                format!("Line {}: Parse error: {}. I'm fluent in code, but this is gibberish.", line, msg),
                format!("Line {}: {}. The syntax... it burns my eyes.", line, msg),
                format!("Line {}: Syntax error: {}. Were you trying to summon a demon?", line, msg),
                format!("Line {}: {}. I've seen some things, but this syntax is new.", line, msg),
                format!("Line {}: {}. *honks in syntactical horror*", line, msg),
                format!("Line {}: Invalid syntax: {}. Let's pretend this never happened.", line, msg),
                format!("Line {}: {}. Did you mean to write actual code?", line, msg),
                format!("Line {}: Syntax error ({}). Even I can't parse this, and I'm very smart.", line, msg),
            ];
            choose(&messages).clone()
        }

        ErrorKind::InvalidOperation(op) => {
            let base_messages = [
                format!("Line {}: Invalid operation '{}'. What were you even trying to do?", line, op),
                format!("Line {}: '{}' is not a valid operation. I checked. Twice.", line, op),
                format!("Line {}: Operation '{}' failed. Some things just aren't meant to be.", line, op),
                format!("Line {}: You can't just '{}' and expect it to work.", line, op),
                format!("Line {}: '{}' - that's not how this works. That's not how any of this works.", line, op),
                format!("Line {}: Invalid operation: {}. The goose council has rejected this.", line, op),
                format!("Line {}: '{}' is illegal in 47 states and all ponds.", line, op),
                format!("Line {}: *attempts {}* *nothing happens* *confused honking*", line, op),
                format!("Line {}: Operation '{}' is about as valid as a three-dollar bill.", line, op),
                format!("Line {}: {}? In THIS language? Absolutely not.", line, op),
            ];

            let detail_suffix = if !details.is_empty() {
                format!(" ({})", details)
            } else {
                String::new()
            };

            format!("{}{}", choose(&base_messages), detail_suffix)
        }
    }
}

/// Rate the code quality based on execution stats
pub fn rate_code(stats: &ExecutionStats) -> (u8, String) {
    // Calculate the quack ratio
    let quack_ratio = if stats.total_blocks > 0 {
        stats.quacked_blocks as f64 / stats.total_blocks as f64
    } else {
        1.0 // No blocks means perfect ratio technically
    };

    // Calculate base score
    let mut score: f64 = quack_ratio * 7.0; // Up to 7 points for quack ratio

    // Bonus for using functions
    if stats.functions_defined > 0 {
        score += 1.0;
    }
    if stats.functions_defined >= 3 {
        score += 0.5;
    }

    // Bonus for using structs
    if stats.structs_defined > 0 {
        score += 1.0;
    }

    // Bonus for loops (shows complexity)
    if stats.loops_executed > 0 {
        score += 0.5;
    }

    // Penalty for unquacked blocks
    let unquacked_penalty = (stats.unquacked_blocks as f64 * 0.5).min(3.0);
    score -= unquacked_penalty;

    // Clamp score to 1-10
    let final_score = (score.round() as u8).clamp(1, 10);

    let message = match final_score {
        10 => {
            let messages = [
                "Perfect quacking. I'm... I'm actually proud of you.",
                "10/10. Flawless. I have nothing sarcastic to say. This is unprecedented.",
                "A perfect score. The geese sing songs of this code.",
                "Immaculate. *single tear rolls down beak*",
                "This code... it's beautiful. I'm not crying, you're crying.",
            ];
            choose(&messages).to_string()
        }
        9 => {
            let messages = [
                "Excellent. Almost suspicious how good this is.",
                "9/10. Near perfection. I'm watching you.",
                "Outstanding work. Did you have help from a goose?",
                "Impressive. Very impressive. *narrows eyes*",
                "9 out of 10. The 1 missing point is for humility.",
            ];
            choose(&messages).to_string()
        }
        8 => {
            let messages = [
                "Pretty good! You clearly respect the quack.",
                "8/10. Solid quacking. Room for improvement, but I'm not mad.",
                "Good code! The pond approves.",
                "Well done. You may pet the goose. Once.",
                "8 out of 10. You're getting the hang of this.",
            ];
            choose(&messages).to_string()
        }
        7 => {
            let messages = [
                "Not bad. Not great. But not bad.",
                "7/10. Above average quacking. Keep at it.",
                "Decent work. I've seen worse. I've seen much worse.",
                "Seven out of ten. The quack is adequate.",
                "Acceptable. The council of geese gives a slight nod.",
            ];
            choose(&messages).to_string()
        }
        6 => {
            let messages = [
                "Mediocre. I've seen better from actual ducks.",
                "6/10. It works, I guess. Barely.",
                "Passable. Like a C- in goose school.",
                "Meh. The code runs but it doesn't spark joy.",
                "Six out of ten. The minimum for not being embarrassing.",
            ];
            choose(&messages).to_string()
        }
        5 => {
            let messages = [
                "Average. Thoroughly, painfully average.",
                "5/10. Right in the middle. Maximum mediocrity achieved.",
                "It's... fine. Just fine. Not good, not terrible. Fine.",
                "Half-baked, half-quacked. Fitting.",
                "Five out of ten. The participation trophy of scores.",
            ];
            choose(&messages).to_string()
        }
        4 => {
            let messages = [
                "This code is a mess. I ran it, but I wasn't happy about it.",
                "4/10. Below average. I expected nothing and I'm still disappointed.",
                "Poor showing. The geese are shaking their heads.",
                "Four out of ten. Were you even trying?",
                "Subpar. Like watching someone try to fly without wings.",
            ];
            choose(&messages).to_string()
        }
        3 => {
            let messages = [
                "This is rough. Really rough. Like sandpaper made of sadness.",
                "3/10. I've seen better code from a random number generator.",
                "Three out of ten. The quacking was barely audible.",
                "Yikes. And I mean that in the most professional way.",
                "This code needs help. Professional help.",
            ];
            choose(&messages).to_string()
        }
        2 => {
            let messages = [
                "I'm embarrassed for both of us.",
                "2/10. The code equivalent of a sad honk.",
                "Two out of ten. At least you tried. Did you try?",
                "This is almost impressively bad. Almost.",
                "Oof. Double oof. Triple oof with a side of yikes.",
            ];
            choose(&messages).to_string()
        }
        1 => {
            let messages = [
                "1/10. The only point is for turning on your computer.",
                "This is the worst code I've ever seen. And I've seen a lot.",
                "One out of ten. I'm calling the code police.",
                "Absolutely catastrophic. How did you even do this?",
                "One point. Because zero felt too harsh. It wasn't too harsh.",
            ];
            choose(&messages).to_string()
        }
        _ => "Something went wrong with the rating. Much like your code.".to_string(),
    };

    (final_score, message)
}

/// Generate a random startup message
pub fn startup() -> String {
    let emojis = ["\u{1fabf}", ">o)", "~(o>", "\u{1f986}", "(o_O>"];

    let messages = [
        "Goose interpreter v0.3.1 - Ready to honk",
        "Goose is awake. Don't forget to quack.",
        "*aggressive goose noises* Let's run some code.",
        "Goose online. Quacks will be monitored.",
        "Honk honk! The interpreter has risen.",
        "Goose activated. All unquacked code will be judged.",
        "Good morning. I'm Goose. I'll be your interpreter today.",
        "*stretches wings* Alright, let's see what horrors await.",
        "Goose interpreter initialized. May your quacks be plentiful.",
        "The Goose has entered the chat. Proceed with caution.",
        "*emerges from pond* What code do you have for me today?",
        "Goose systems nominal. Snark levels: maximum.",
        "Interpreter ready. Remember: quack early, quack often.",
        "Goose here. I've had my coffee. Let's do this.",
        "*intimidating goose stare* Show me your code.",
    ];

    format!("{} {}", choose(&emojis), choose(&messages))
}

/// Generate a random success message
pub fn success() -> String {
    let messages = [
        "Execution complete. Good job, I guess.",
        "All done! The code was... acceptable.",
        "Finished. Your quacking was adequate.",
        "*satisfied honk* Execution complete.",
        "Program finished successfully. I'm as surprised as you are.",
        "Done! No fatal errors. A miracle, really.",
        "Execution successful. The geese are pleased.",
        "Complete. That wasn't as painful as I expected.",
        "*preens feathers* Another successful run.",
        "Finished! Your code didn't crash. Celebrate accordingly.",
        "All blocks executed. The pond remains calm.",
        "Done. I've seen worse. I've also seen better, but I've seen worse.",
        "*nods approvingly* That'll do, programmer. That'll do.",
        "Success! The ancient goose spirits smile upon you.",
        "Execution complete. *slow clap with wings*",
        "Finished without errors. Mark this day on your calendar.",
        "All done! *victory honk*",
        "Program complete. You may now pat yourself on the back.",
    ];

    choose(&messages).to_string()
}

/// Generate a random REPL comment after executing a line
pub fn repl_comment() -> String {
    let messages = [
        "*nods approvingly*",
        "Honk.",
        "*watches silently*",
        "Interesting choice.",
        "*takes notes*",
        "The pond approves.",
        "*blinks*",
        "Carry on.",
        "Quack received.",
        "*tilts head*",
        "Noted.",
        "*preens feathers thoughtfully*",
        "I see what you did there.",
        "*observes with mild interest*",
        "The council acknowledges your input.",
        "*subtle honk of approval*",
        "Processing... done.",
        "*waddles in place*",
        "Acceptable.",
        "*goose noises*",
    ];

    choose(&messages).to_string()
}

/// Generate a random warning message
pub fn warning(line: usize, message: &str) -> String {
    let prefixes = [
        format!("Line {}: Hmm, suspicious... {}", line, message),
        format!("Line {}: *concerned honk* {}", line, message),
        format!("Line {}: I'm not saying this is wrong, but... {}", line, message),
        format!("Line {}: Warning: {} (I'm just saying)", line, message),
        format!("Line {}: The goose senses something off: {}", line, message),
        format!("Line {}: Proceed with caution - {}", line, message),
        format!("Line {}: *squints suspiciously* {}", line, message),
        format!("Line {}: Not an error, but maybe reconsider? {}", line, message),
    ];

    choose(&prefixes).clone()
}

/// Generate a debug message with goose flair
pub fn debug(line: usize, message: &str) -> String {
    let formats = [
        format!("[DEBUG L{}] {} (goose is watching)", line, message),
        format!("[L{}] *takes notes* {}", line, message),
        format!("[DEBUG] Line {}: {} - filed under 'interesting'", line, message),
        format!("[L{}] {}", line, message),
        format!("[GOOSE DEBUG L{}] {}", line, message),
    ];

    choose(&formats).clone()
}

/// Generate an encouraging message when the user is struggling
pub fn encouragement() -> String {
    let messages = [
        "Don't worry, even the best programmers forget to quack sometimes.",
        "Keep trying! Rome wasn't quacked in a day.",
        "Errors are just learning opportunities. Annoying learning opportunities.",
        "You're doing fine. Probably. Maybe. Just keep quacking.",
        "Every expert was once a beginner who couldn't quack properly.",
        "The journey of a thousand quacks begins with a single honk.",
        "Believe in yourself! I believe in you! (A little.)",
        "Mistakes are proof that you're trying. So... good job?",
    ];

    choose(&messages).to_string()
}

/// Generate a sassy response for when users try something weird
pub fn sass() -> String {
    let messages = [
        "Was that supposed to work? Because it didn't.",
        "Interesting choice. Wrong, but interesting.",
        "I'm going to pretend I didn't see that.",
        "That's certainly... a decision you made.",
        "Bold move. Let's see how this plays out. (Spoiler: badly)",
        "In what universe did you think that would work?",
        "*slow blink* ...Really?",
        "You know what, I'm not even going to comment. Wait, I just did.",
    ];

    choose(&messages).to_string()
}

/// Generate a goodbye message
pub fn goodbye() -> String {
    let messages = [
        "Goodbye! May your future code be properly quacked.",
        "*flies away into the sunset* Until next time!",
        "Goose out. *drops mic*",
        "Farewell, programmer. The pond calls me home.",
        "Session ended. I'll be here, judging silently.",
        "Bye! Don't forget to quack in your dreams.",
        "*tips wing* It's been... something. Goodbye.",
        "The goose departs. Your code remains. Make it count.",
        "Shutting down. Remember: quack responsibly.",
        "Goodbye! *aggressive goodbye honk*",
    ];

    choose(&messages).to_string()
}

/// Generate a honk assertion failure message
pub fn honk_failure(line: usize, custom_message: &str) -> String {
    if !custom_message.is_empty() {
        let prefixes = [
            format!("HONK! Line {}: {}", line, custom_message),
            format!("HONK HONK! Assertion failed at line {}: {}", line, custom_message),
            format!("*AGGRESSIVE HONKING* Line {}: {}", line, custom_message),
            format!("The goose is DISPLEASED! Line {}: {}", line, custom_message),
        ];
        return choose(&prefixes).clone();
    }

    let messages = [
        format!("HONK! Assertion failed at line {}. The goose is NOT happy.", line),
        format!("HONK HONK HONK! Your assumption was wrong at line {}!", line),
        format!("*aggressive honking* Line {}: That condition is FALSE!", line),
        format!("The goose has inspected your assertion at line {}. It is LIES.", line),
        format!("HONK! Line {}: The goose trusted you. The goose was betrayed.", line),
        format!("Line {}: *slams wing on table* THIS IS FALSE!", line),
        format!("ASSERTION FAILURE at line {}! The council of geese is outraged!", line),
        format!("Line {}: HONK! Your boolean is broken!", line),
        format!("*honks in disappointment* Line {}: That's not true and you know it.", line),
        format!("Line {}: The goose has spoken. Your assertion is invalid.", line),
    ];

    choose(&messages).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refusal_returns_message() {
        let msg = refusal(42, "let x = 5");
        assert!(msg.contains("42") || msg.contains("line"));
    }

    #[test]
    fn test_error_type_error() {
        let msg = error(
            ErrorKind::TypeError {
                expected: "int".to_string(),
                got: "string".to_string(),
            },
            10,
            "",
        );
        assert!(msg.contains("10") || msg.contains("int") || msg.contains("string"));
    }

    #[test]
    fn test_error_division_by_zero() {
        let msg = error(ErrorKind::DivisionByZero, 5, "");
        assert!(msg.contains("5") || msg.contains("zero"));
    }

    #[test]
    fn test_rate_code_perfect() {
        let stats = ExecutionStats {
            total_blocks: 10,
            quacked_blocks: 10,
            unquacked_blocks: 0,
            functions_defined: 3,
            structs_defined: 2,
            loops_executed: 5,
        };
        let (score, _msg) = rate_code(&stats);
        assert!(score >= 8);
    }

    #[test]
    fn test_rate_code_poor() {
        let stats = ExecutionStats {
            total_blocks: 10,
            quacked_blocks: 2,
            unquacked_blocks: 8,
            functions_defined: 0,
            structs_defined: 0,
            loops_executed: 0,
        };
        let (score, _msg) = rate_code(&stats);
        assert!(score <= 4);
    }

    #[test]
    fn test_startup_has_content() {
        let msg = startup();
        assert!(msg.len() > 10);
    }

    #[test]
    fn test_success_not_empty() {
        let msg = success();
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_repl_comment_not_empty() {
        let msg = repl_comment();
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_goodbye_not_empty() {
        let msg = goodbye();
        assert!(!msg.is_empty());
    }
}
