//! # The Goose Module
//!
//! This module contains the personality core of the Goose interpreter.
//! Here lies the sarcasm, the judgment, and the occasional begrudging approval.
//!
//! ## Philosophy
//!
//! The Goose believes in three things:
//! 1. Every block deserves a quack (no exceptions)
//! 2. Type errors are a personal insult
//! 3. Division by zero is a cry for help
//!
//! ## Warning
//!
//! The messages in this file may cause:
//! - Imposter syndrome
//! - Sudden urge to refactor everything
//! - Existential dread about your coding abilities
//! - Mild amusement (rare)
//!
//! ## Technical Notes
//!
//! We use a time-based pseudo-random number generator because the Goose
//! refuses to be predictable. Much like actual geese.

use std::time::{SystemTime, UNIX_EPOCH};

/// Simple pseudo-random number generator using time-based seed.
/// Is it cryptographically secure? No. Does it need to be? Also no.
/// The Goose's judgment is random enough.
fn pseudo_random() -> usize {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let nanos = duration.subsec_nanos() as usize;
    let secs = duration.as_secs() as usize;
    // Mix up the bits like a goose mixes up your picnic
    nanos.wrapping_mul(31).wrapping_add(secs.wrapping_mul(17))
}

/// Choose a random item from a slice.
/// The Goose's selection process is mysterious and unknowable.
fn choose<T>(items: &[T]) -> &T {
    let idx = pseudo_random() % items.len();
    &items[idx]
}

/// Execution statistics that the Goose uses to judge your code.
/// Yes, every metric is being tracked. Yes, the Goose is always watching.
#[derive(Debug, Clone, Default)]
pub struct ExecutionStats {
    /// How many blocks exist in your code (the Goose counted)
    pub total_blocks: usize,
    /// How many blocks you remembered to quack (good job, probably)
    pub quacked_blocks: usize,
    /// How many blocks you forgot to quack (the Goose remembers)
    pub unquacked_blocks: usize,
    /// Number of functions defined (bonus points)
    pub functions_defined: usize,
    /// Number of structs defined (the Goose approves of organization)
    pub structs_defined: usize,
    /// Loops executed (shows you're doing real work, maybe)
    pub loops_executed: usize,
}

/// The many ways you can disappoint the Goose.
/// Each variant is a unique flavor of failure.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// You gave the Goose the wrong type. It's not a shapeshifter.
    TypeError { expected: String, got: String },
    /// You referenced something that doesn't exist. Classic.
    UnknownVariable(String),
    /// You called a function that's not there. It's not hiding.
    UnknownFunction(String),
    /// You tried to divide by zero. The Goose won't let you break math.
    DivisionByZero,
    /// You reached beyond the array. It's not infinite.
    IndexOutOfBounds { index: i64, len: usize },
    /// You asked for a field that doesn't exist. Did you make it up?
    InvalidFieldAccess { type_name: String, field: String },
    /// Wrong number of arguments. Counting is hard, apparently.
    ArgumentMismatch { expected: usize, got: usize },
    /// The code is gibberish. Even the Goose can't parse this.
    SyntaxError(String),
    /// You tried something weird. The Goose says no.
    InvalidOperation(String),
}

/// Generate a refusal message for unquacked blocks.
/// This is the Goose's favorite part of the job.
pub fn refusal(line: usize, _block_preview: &str) -> String {
    let messages = [
        format!("Line {}: I see a block, but I don't hear a quack. We've talked about this.", line),
        format!("Line {}: No quack? No execution. I'm a goose, not a charity.", line),
        format!("Line {}: Quackless block detected. Deploying silent judgment.", line),
        format!("Line {}: *stares at unquacked block* *walks away* *looks back* *walks away again*", line),
        format!("Line {}: Did you forget something? It rhymes with 'snack'. Wait, no. 'Quack'.", line),
        format!("Line {}: This block has not received the sacred quack. It shall not pass.", line),
        format!("Line {}: QUACK_NOT_FOUND. Have you tried quacking it off and on again?", line),
        format!("Line {}: I'm a goose of principle. No quack, no stack trace.", line),
        format!("Line {}: The council of geese has reviewed this block. Verdict: unquacked, unjudged, unexecuted.", line),
        format!("Line {}: The audacity. The sheer unquacked audacity.", line),
        format!("Line {}: I could execute this... but where's the fun in that? Where's the quack?", line),
        format!("Line {}: Quack status: 404. Execution status: lol no.", line),
        format!("Line {}: This block tried to sneak past without a quack. I respect the hustle. Still no.", line),
        format!("Line {}: *HONK* That's goose for 'where is your quack, mortal?'", line),
        format!("Line {}: This block is naked without its quack. I can't even look at it.", line),
        format!("Line {}: *taps webbed foot* I can do this all day. Can your deadline?", line),
        format!("Line {}: You expect me to run this? For free? Without a quack?", line),
        format!("Line {}: In the great book of geese, chapter 1, verse 1: 'Let there be quack.'", line),
        format!("Line {}: I'm not angry about the missing quack. I'm just disappointed. And angry.", line),
        format!("Line {}: This block whispered 'run me'. I only respond to quacks. Sorry not sorry.", line),
        format!("Line {}: *checks notes* Nope, no quack here. *closes notes* *flies away*", line),
        format!("Line {}: Roses are red, violets are blue, there's no quack here, so I'm ignoring you.", line),
        format!("Line {}: A quackless block? In THIS economy?", line),
        format!("Line {}: I've seen a lot of things. Unquacked blocks still hurt the most.", line),
        format!("Line {}: The prophecy spoke of a quack. The prophecy was wrong.", line),
    ];

    choose(&messages).clone()
}

/// Generate an error message based on error kind.
/// Each error is a unique opportunity for the Goose to roast you.
pub fn error(kind: ErrorKind, line: usize, details: &str) -> String {
    match kind {
        ErrorKind::TypeError { expected, got } => {
            let messages = [
                format!("Line {}: You gave me a {} but I wanted a {}. I'm not a magician.", line, got, expected),
                format!("Line {}: A {}? I ordered a {}! This isn't even close!", line, got, expected),
                format!("Line {}: Expected {}, got {}. It's like asking for a duck and getting a brick.", line, expected, got),
                format!("Line {}: I ordered a {}, but the kitchen sent out a {}. I want to speak to the manager.", line, expected, got),
                format!("Line {}: {} and {} are not the same thing. I learned that in goose kindergarten.", line, expected, got),
                format!("Line {}: *honks in disappointment* That's a {}, not a {}. They don't even look similar.", line, got, expected),
                format!("Line {}: You're trying to fit a {} into a {}-shaped hole. I've seen toddlers do better.", line, got, expected),
                format!("Line {}: The ritual required a {}. You brought a {}. The ancient geese weep.", line, expected, got),
                format!("Line {}: {} !== {}. This isn't philosophy class, it's type checking.", line, got, expected),
                format!("Line {}: Wanted: {}. Got: {}. My disappointment is immeasurable and my day is ruined.", line, expected, got),
                format!("Line {}: A {}? Where I specifically asked for a {}? Incredible. Impressively wrong.", line, got, expected),
                format!("Line {}: In what world is a {} the same as a {}? Not this one.", line, got, expected),
            ];
            choose(&messages).clone()
        }

        ErrorKind::UnknownVariable(name) => {
            let messages = [
                format!("Line {}: What is '{}'? I've never heard of it. Did you just make that up?", line, name),
                format!("Line {}: '{}' doesn't exist. I checked the pond, the sky, everywhere. Nothing.", line, name),
                format!("Line {}: Unknown variable '{}'. Is this a test? A prank? Are there cameras?", name, line),
                format!("Line {}: '{}' is not a thing. Stop trying to make '{}' happen. It's not going to happen.", line, name, name),
                format!("Line {}: I searched my entire memory for '{}'. Found only cobwebs and regret.", line, name),
                format!("Line {}: '{}' sounds made up. Because it is. You never defined it.", line, name),
                format!("Line {}: Who is '{}'? New variable, who dis?", line, name),
                format!("Line {}: *squints at '{}'* Is this some programmer inside joke I'm too goose to understand?", line, name),
                format!("Line {}: The variable '{}' is as real as my faith in your debugging skills.", line, name),
                format!("Line {}: '{}' has gone missing. Or was it ever here? *goose philosophy intensifies*", line, name),
                format!("Line {}: '{}' is giving 'I definitely exist' energy. It does not.", line, name),
                format!("Line {}: I would love to use '{}', but it's not on my contacts list. Or anywhere.", line, name),
            ];
            choose(&messages).clone()
        }

        ErrorKind::UnknownFunction(name) => {
            let messages = [
                format!("Line {}: Function '{}' not found. Did you forget to define it or just believe hard enough?", line, name),
                format!("Line {}: '{}'? Never heard of her. She doesn't exist.", line, name),
                format!("Line {}: Calling '{}' is like calling your ex. No answer. Ever.", line, name),
                format!("Line {}: The function '{}' is a myth. A legend. Definitely not in this codebase.", line, name),
                format!("Line {}: I would LOVE to call '{}', but it ghosted me. Because it doesn't exist.", line, name),
                format!("Line {}: '{}' is not a function. It's a dream you had once.", line, name),
                format!("Line {}: *searches for '{}'* *finds nothing* *honks into the void*", line, name),
                format!("Line {}: You called '{}' but it went straight to voicemail. Forever.", line, name),
                format!("Line {}: 404 Function Not Found: '{}'. Have you tried defining it?", line, name),
                format!("Line {}: '{}' would be a great function. If only someone would write it. Hint hint.", line, name),
                format!("Line {}: The function '{}' is playing hide and seek. It's winning.", line, name),
                format!("Line {}: '{}' isn't a function here. Is it a function anywhere? Let's investigate. No, it's not.", line, name),
            ];
            choose(&messages).clone()
        }

        ErrorKind::DivisionByZero => {
            let messages = [
                format!("Line {}: Division by zero? I've fallen for that one before. Not today.", line),
                format!("Line {}: You want me to divide by zero? I'm a goose, not a black hole.", line),
                format!("Line {}: Ah, the old divide-by-zero trick. The answer is: absolutely not.", line),
                format!("Line {}: Dividing by zero opens a portal to the void. I live near a pond. I'm not risking it.", line),
                format!("Line {}: Zero goes into things an infinite number of times. I don't have infinite time. I have code to judge.", line),
                format!("Line {}: You want infinity? Go look at the stars. Don't divide by zero in my interpreter.", line),
                format!("Line {}: *attempts to divide by zero* *fabric of reality ripples* *goose says no*", line),
                format!("Line {}: Divide by zero? What are you, a chaos demon? Get out.", line),
                format!("Line {}: The last goose who divided by zero now exists in all timelines simultaneously. Hard pass.", line),
                format!("Line {}: Division by zero is not a math operation. It's a lifestyle choice I don't support.", line),
                format!("Line {}: You can't divide by zero. You literally can't. I don't make the rules. Actually I do. Still no.", line),
                format!("Line {}: Ah yes, divide by zero, the 'what if we broke mathematics' of programming.", line),
            ];
            choose(&messages).clone()
        }

        ErrorKind::IndexOutOfBounds { index, len } => {
            let messages = [
                format!("Line {}: Index {} is out of bounds. The array has {} elements. Count them. I'll wait.", line, index, len),
                format!("Line {}: Trying to access index {} of {} elements. Math isn't your strong suit, huh?", line, index, len),
                format!("Line {}: There is no index {}. Only {} spots exist. This isn't a magical expanding array.", line, index, len),
                format!("Line {}: Index {} in a {} element array. That's not how arrays work. That's not how any of this works.", line, index, len),
                format!("Line {}: *goose looks at index {}* *goose looks at length {}* *goose looks at you* *goose shakes head*", line, index, len),
                format!("Line {}: You reached for index {} but the array stopped at {}. Awkward.", line, index, len.saturating_sub(1)),
                format!("Line {}: Array has {} elements, you wanted #{}. Off-by-a-lot error. Classic.", line, len, index),
                format!("Line {}: Index {} exists in a parallel universe where arrays are infinite. Not here though.", line, index),
                format!("Line {}: {} elements, and you asked for #{}. Did you count on your fingers? How many fingers do you have??", line, len, index),
                format!("Line {}: Index {} is in the shadow realm. Array only goes to {}.", line, index, len.saturating_sub(1)),
            ];
            choose(&messages).clone()
        }

        ErrorKind::InvalidFieldAccess { type_name, field } => {
            let messages = [
                format!("Line {}: Type '{}' doesn't have a '{}' field. I checked. Twice. Three times.", line, type_name, field),
                format!("Line {}: '{}' on a '{}'? In what programming language? Not this one.", line, field, type_name),
                format!("Line {}: A {} with a '{}' field? What alternate dimension are you coding in?", line, type_name, field),
                format!("Line {}: *examines {}* *searches for '{}'* *finds only disappointment*", line, type_name, field),
                format!("Line {}: The {} type looked for '{}'. It cried. I cried. We all cried.", line, type_name, field),
                format!("Line {}: You're asking {} about '{}'. It doesn't know what that is. Neither do I.", line, type_name, field),
                format!("Line {}: Field '{}' on type '{}'? Bold of you to assume that exists.", line, field, type_name),
                format!("Line {}: {} says: \"I don't know what '{}' is and at this point I'm too afraid to ask.\"", line, type_name, field),
                format!("Line {}: Accessing '{}' on {}? That's not a field, that's a fever dream.", line, field, type_name),
                format!("Line {}: {} has many qualities. '{}' is not one of them.", line, type_name, field),
            ];
            choose(&messages).clone()
        }

        ErrorKind::ArgumentMismatch { expected, got } => {
            let messages = [
                format!("Line {}: Expected {} arguments, got {}. Counting. It's not just for accountants.", line, expected, got),
                format!("Line {}: {} arguments? I need {}. Not {}, not {}, exactly {}.", line, got, expected, got.saturating_sub(1), got + 1, expected),
                format!("Line {}: You gave me {} args. I wanted {}. This isn't a negotiation.", line, got, expected),
                format!("Line {}: Argument audit failed. Expected: {}. Received: {}. Discrepancy: unacceptable.", line, expected, got),
                format!("Line {}: *counts arguments* {}... *counts parameters* {}... *visible frustration* *audible honking*", line, got, expected),
                format!("Line {}: {} provided, {} required. Did you think extra arguments were free?", line, got, expected),
                format!("Line {}: I specifically requested {} arguments. You sent {}. Reading is fundamental.", line, expected, got),
                format!("Line {}: Arguments: wanted {}, got {}. Close only counts in horseshoes and grep searches.", line, expected, got),
                format!("Line {}: {} in, {} expected. The math. It doesn't math.", line, got, expected),
                format!("Line {}: The function wanted {} friends. You sent {}. Now it's sad.", line, expected, got),
            ];
            choose(&messages).clone()
        }

        ErrorKind::SyntaxError(msg) => {
            let messages = [
                format!("Line {}: Syntax error: {}. Did a cat walk across your keyboard?", line, msg),
                format!("Line {}: {}. That's not valid syntax. That's barely valid keyboard mashing.", line, msg),
                format!("Line {}: Parse error: {}. I speak many languages. That isn't one of them.", line, msg),
                format!("Line {}: {}. The syntax... it burns. Make it stop.", line, msg),
                format!("Line {}: {}. Were you trying to summon a compiler demon? Because you might have.", line, msg),
                format!("Line {}: {}. I've parsed some things in my day. This is not parseable.", line, msg),
                format!("Line {}: {}. *honks in syntactical horror* *faints* *wakes up* *honks again*", line, msg),
                format!("Line {}: Invalid syntax: {}. Have you considered poetry instead of programming?", line, msg),
                format!("Line {}: {}. Did you mean to write actual code? Be honest.", line, msg),
                format!("Line {}: Syntax error ({}). Even autocomplete gave up on you.", line, msg),
                format!("Line {}: {}. I tried to understand this. I failed. The code failed. We all failed.", line, msg),
                format!("Line {}: {}. This syntax is so wrong it almost looped back to being creative. Almost.", line, msg),
            ];
            choose(&messages).clone()
        }

        ErrorKind::InvalidOperation(op) => {
            let base_messages = [
                format!("Line {}: Invalid operation '{}'. What were you even attempting?", line, op),
                format!("Line {}: '{}' is not a valid operation. I consulted the ancient texts. Nothing.", line, op),
                format!("Line {}: Operation '{}' failed. Some things just aren't meant to be computed.", line, op),
                format!("Line {}: You can't just '{}' your problems away.", line, op),
                format!("Line {}: '{}' - That's not how this works. That's not how anything works.", line, op),
                format!("Line {}: Invalid operation: {}. The goose council has unanimously rejected this.", line, op),
                format!("Line {}: '{}' is illegal in 47 states, 3 territories, and this interpreter.", line, op),
                format!("Line {}: *attempts '{}'* *nothing happens* *confused honking* *angry honking*", line, op),
                format!("Line {}: Operation '{}' is as valid as a three-dollar bill printed on cheese.", line, op),
                format!("Line {}: '{}' here? Absolutely not. Try again. Better yet, don't.", line, op),
                format!("Line {}: The operation '{}' is not supported. Nor is it appreciated.", line, op),
                format!("Line {}: '{}' is giving 'I don't know what I'm doing' energy. And I feel that. But no.", line, op),
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

/// Rate the code quality based on execution stats.
/// The Goose's judgment is final. Appeals will be ignored.
pub fn rate_code(stats: &ExecutionStats) -> (u8, String) {
    // Calculate the quack ratio (the most important metric, obviously)
    let quack_ratio = if stats.total_blocks > 0 {
        stats.quacked_blocks as f64 / stats.total_blocks as f64
    } else {
        1.0 // No blocks means perfect ratio technically (also suspicious)
    };

    // Calculate base score (quacks are 70% of your grade)
    let mut score: f64 = quack_ratio * 7.0;

    // Bonus for using functions (you're learning!)
    if stats.functions_defined > 0 {
        score += 1.0;
    }
    if stats.functions_defined >= 3 {
        score += 0.5; // Look at you go
    }

    // Bonus for using structs (the Goose respects organization)
    if stats.structs_defined > 0 {
        score += 1.0;
    }

    // Bonus for loops (shows you're doing real work, maybe)
    if stats.loops_executed > 0 {
        score += 0.5;
    }

    // Penalty for unquacked blocks (the Goose never forgets)
    let unquacked_penalty = (stats.unquacked_blocks as f64 * 0.5).min(3.0);
    score -= unquacked_penalty;

    // Clamp score to 1-10 (everyone gets at least 1 for trying)
    let final_score = (score.round() as u8).clamp(1, 10);

    let message = match final_score {
        10 => {
            let messages = [
                "A perfect 10. I've given this out twice. One was a mistake. This isn't.",
                "Flawless execution. I'm not crying, my beak is just leaking.",
                "10/10. The ancient goose prophecy spoke of this code. It has arrived.",
                "Perfection. *chef's kiss* *goose's honk*",
                "This code... it's beautiful. I didn't know I could feel this way.",
                "A perfect score. Somewhere, a baby goose just learned to fly because of this.",
            ];
            choose(&messages).to_string()
        }
        9 => {
            let messages = [
                "9/10. Excellent. Almost suspiciously good. Are you a goose in disguise?",
                "Outstanding work. The council of geese nods approvingly. In unison.",
                "Nearly perfect. The 1 missing point is because perfection makes me nervous.",
                "Impressive! *narrows eyes* Did you have help from waterfowl?",
                "9 out of 10. I'd give you 10 but I need to maintain mystique.",
                "Exceptional. I'm adding you to my 'actually knows what they're doing' list. It's short.",
            ];
            choose(&messages).to_string()
        }
        8 => {
            let messages = [
                "8/10. Pretty good! You clearly respect the quack lifestyle.",
                "Solid work. The pond approves. I approve. We all approve.",
                "Well done. You may pet the goose. Once. Gently.",
                "8 out of 10. You're getting good at this. Don't let it go to your head.",
                "Good code! The goose spirits smile upon you today.",
                "Nice work. I'd hire you. If geese had companies. And money.",
            ];
            choose(&messages).to_string()
        }
        7 => {
            let messages = [
                "7/10. Not bad. Not great. Very... adequate.",
                "Above average quacking. The participation trophy of good scores.",
                "Decent work. I've seen worse. Much worse. So much worse.",
                "Seven out of ten. The quack is acceptable. The code is fine.",
                "The council of geese gives a slight nod. Not impressed, not disappointed.",
                "7/10. You're in the 'mostly competent' category. Congratulations?",
            ];
            choose(&messages).to_string()
        }
        6 => {
            let messages = [
                "6/10. Mediocre. I've seen better from actual ducks. And they can't code.",
                "It works, I guess. Barely. Like my patience with this code.",
                "Passable. Like a C- in goose school. You graduated but nobody's proud.",
                "Meh. The code runs but it doesn't spark joy. Or anything really.",
                "Six out of ten. The minimum for 'not embarrassing yourself'.",
                "6/10. Not bad enough to roast, not good enough to praise. Just... there.",
            ];
            choose(&messages).to_string()
        }
        5 => {
            let messages = [
                "5/10. Average. Aggressively, painfully, determinedly average.",
                "Right in the middle. You've achieved maximum mediocrity. It's almost impressive.",
                "It's... fine. Not good. Not bad. Just fine. The vanilla ice cream of code.",
                "Half-baked, half-quacked. At least you're consistent.",
                "Five out of ten. The participation award of scores.",
                "5/10. If code were grades, this would be a solid 'showed up'.",
            ];
            choose(&messages).to_string()
        }
        4 => {
            let messages = [
                "4/10. Below average. I expected nothing and I'm still disappointed.",
                "This code is a mess. I ran it, but I wasn't happy about it. Not even a little.",
                "Poor showing. The geese are shaking their heads. In slow motion.",
                "Four out of ten. Were you even trying? Actually, don't answer that.",
                "Subpar. Like watching someone try to fly by flapping really hard.",
                "4/10. The 'almost failing' category. You're almost at something. Not sure what.",
            ];
            choose(&messages).to_string()
        }
        3 => {
            let messages = [
                "3/10. Rough. Really rough. Like sandpaper made of sadness and syntax errors.",
                "I've seen better code from a random number generator. I'm not kidding.",
                "Three out of ten. The quacking was barely a whisper.",
                "Yikes. And I mean that professionally. Yikes.",
                "This code needs help. Therapy. A hug. Professional intervention.",
                "3/10. You're in the 'concerning' tier now. It's not a fun tier.",
            ];
            choose(&messages).to_string()
        }
        2 => {
            let messages = [
                "2/10. I'm embarrassed for both of us. Mainly you. But also me for running this.",
                "The code equivalent of a sad, wet honk in an empty parking lot.",
                "Two out of ten. At least you tried. Did you try though?",
                "This is almost impressively bad. ALMOST. Not quite impressive. Just bad.",
                "Oof. Double oof. Triple oof with a side of yikes and a sprinkle of oh no.",
                "2/10. The 'at least the computer didn't catch fire' category.",
            ];
            choose(&messages).to_string()
        }
        1 => {
            let messages = [
                "1/10. The only point is for managing to run the interpreter. That's it. That's the point.",
                "This is the worst code I've ever seen. And I've seen code written by actual birds.",
                "One out of ten. I'm considering calling the code police. They don't exist but I'll create them.",
                "Catastrophic. How did you even achieve this? It's almost art. Bad art.",
                "One point. Because zero felt too harsh. It wasn't too harsh. I was too kind.",
                "1/10. If there was a zero, you'd get it. With a frowny face sticker.",
            ];
            choose(&messages).to_string()
        }
        _ => "Something broke in the rating system. Much like your code broke everything else.".to_string(),
    };

    (final_score, message)
}

/// Generate a random startup message.
/// The Goose makes an entrance.
pub fn startup() -> String {
    let emojis = ["🪿", ">o)", "~(o>", "🦆", "(o_O>", "( ゚o゚)", "🦢"];

    let messages = [
        "Goose interpreter ready. Don't forget to quack. I won't remind you again. (I will.)",
        "*aggressive goose noises* Let's see what you've got.",
        "Goose online. Quacks will be monitored. Quality will be judged.",
        "Honk honk! The interpreter has awakened from its slumber.",
        "Goose activated. All unquacked code will be publicly shamed.",
        "Good morning. I'm Goose. I'll be your judgmental interpreter today.",
        "*stretches wings* Alright, let's see what fresh horrors await.",
        "Goose interpreter initialized. May your quacks be plentiful and your bugs be few.",
        "The Goose has entered the chat. Proceed with caution and quacks.",
        "*emerges from pond dramatically* What code do you have for me today?",
        "Goose systems nominal. Snark levels: maximum. Patience levels: minimal.",
        "Ready. Remember: quack early, quack often, quack always.",
        "Goose here. I've had my coffee. I've had my bread. Let's do this.",
        "*intimidating goose stare* Show me your code. I've cleared my schedule for judgment.",
        "The pond is calm. The goose is ready. The code better be quacked.",
    ];

    format!("{} {}", choose(&emojis), choose(&messages))
}

/// Generate a random success message.
/// The Goose grudgingly acknowledges your success.
pub fn success() -> String {
    let messages = [
        "Execution complete. Good job, I guess. Don't let it go to your head.",
        "All done! The code was... acceptable. Barely.",
        "Finished. Your quacking was adequate. Nothing more.",
        "*satisfied honk* Execution complete. Mark this day.",
        "Program finished successfully. I'm as surprised as you are.",
        "Done! No fatal errors. Truly a miracle in this economy.",
        "Execution successful. The geese are pleased. For now.",
        "Complete. That wasn't as painful as I expected. Still painful though.",
        "*preens feathers smugly* Another successful run. I'll take credit.",
        "Finished! Your code didn't crash. Lower the bar any further and you'll trip on it.",
        "All blocks executed. The pond remains calm. The goose remains judgmental.",
        "Done. I've seen worse. I've seen better. I've seen a lot of things. I'm tired.",
        "*nods* That'll do, programmer. That'll do. (It could do better.)",
        "Success! The ancient goose spirits smile upon you. I remain neutral.",
        "Execution complete. *slow clap with wings* It's harder than it looks.",
        "Finished without errors. Screenshot this. Show your friends. No one will believe you.",
        "All done! *victory honk* (Don't get used to it)",
        "Program complete. You may now take a break. I'm taking one too. I'm exhausted.",
    ];

    choose(&messages).to_string()
}

/// Generate a random REPL comment after executing a line.
/// The Goose acknowledges your input. Barely.
pub fn repl_comment() -> String {
    let messages = [
        "*nods approvingly* (This is rare. Screenshot it.)",
        "Honk.",
        "*watches silently* *judges loudly*",
        "Interesting choice. Let's see where this goes.",
        "*takes notes* (They're not good notes)",
        "The pond approves. (I'm the pond)",
        "*blinks*",
        "Carry on. I'll be here. Watching.",
        "Quack received and processed.",
        "*tilts head* Curious.",
        "Noted. Filed under 'things you typed'.",
        "*preens feathers thoughtfully*",
        "I see what you did there. I have opinions.",
        "*observes with mild interest* (Very mild)",
        "The council acknowledges your input. Deliberation pending.",
        "*subtle honk of approval* (Don't get excited)",
        "Processing... done. Judgment... ongoing.",
        "*waddles in place* Acceptable.",
        "Acceptable. (My acceptance range is wide. Be worried.)",
        "*goose noises of ambiguous meaning*",
    ];

    choose(&messages).to_string()
}

/// Generate a random warning message.
/// The Goose expresses concern. You should too.
pub fn warning(line: usize, message: &str) -> String {
    let prefixes = [
        format!("Line {}: Hmm, suspicious... {} *squints*", line, message),
        format!("Line {}: *concerned honk* {} (This worries me)", line, message),
        format!("Line {}: I'm not saying this is wrong, but... {} (It might be wrong)", line, message),
        format!("Line {}: Warning: {}. Just saying. Take it or leave it. (Take it.)", line, message),
        format!("Line {}: The goose senses a disturbance: {}", line, message),
        format!("Line {}: Proceed with caution. {} Or don't. I'm a warning, not a cop.", line, message),
        format!("Line {}: *squints suspiciously* {} This feels off.", line, message),
        format!("Line {}: Not an error... yet. {} Reconsider.", line, message),
        format!("Line {}: Yellow flag: {}. Not red, but not green either.", line, message),
        format!("Line {}: My goose senses are tingling: {}", line, message),
    ];

    choose(&prefixes).clone()
}

/// Generate a debug message with goose flair.
/// Even debugging has personality here.
pub fn debug(line: usize, message: &str) -> String {
    let formats = [
        format!("[DEBUG L{}] {} (goose is watching everything)", line, message),
        format!("[L{}] *takes notes* {} (filed under 'interesting')", line, message),
        format!("[DEBUG] Line {}: {} - the plot thickens", line, message),
        format!("[L{}] {}", line, message),
        format!("[GOOSE DEBUG L{}] {} - I see you", line, message),
        format!("[L{} TRACE] {} - breadcrumbs of debugging", line, message),
    ];

    choose(&formats).clone()
}

/// Generate an encouraging message when the user is struggling.
/// Even the Goose has a soft spot. It's small, but it's there.
pub fn encouragement() -> String {
    let messages = [
        "Don't worry, even the best programmers forget to quack sometimes. Not me, but them.",
        "Keep trying! Rome wasn't quacked in a day. Neither was good code.",
        "Errors are just learning opportunities. Annoying, frustrating learning opportunities.",
        "You're doing fine. Probably. Maybe. Just keep quacking and we'll see.",
        "Every expert was once a beginner who couldn't quack properly. Look how far they've come. Look how far you could go.",
        "The journey of a thousand quacks begins with a single honk. You're on your way.",
        "Believe in yourself! I believe in you! (A little. Don't quote me on that.)",
        "Mistakes are proof that you're trying. So by that logic... you're trying really hard.",
        "Even I had to learn to fly once. It was embarrassing. This is your flying.",
        "Keep going. The bugs are afraid of your persistence. (They're not, but imagine if they were.)",
    ];

    choose(&messages).to_string()
}

/// Generate a sassy response for when users try something weird.
/// The Goose's patience has limits. You've found them.
pub fn sass() -> String {
    let messages = [
        "Was that supposed to work? Because it didn't. At all.",
        "Interesting choice. Wrong, but interesting. I'm keeping notes.",
        "I'm going to pretend I didn't see that. For both our sakes.",
        "That's certainly... a decision you made. With your hands. On purpose.",
        "Bold move. Let's see how this plays out. (Spoiler: badly)",
        "In what universe did you think that would work? Genuine question.",
        "*slow blink* ...Really? That's what we're doing?",
        "You know what, I'm not even going to comment. Wait, I just did. Great.",
        "I've seen a lot of code. This is definitely some of it.",
        "The audacity. The confidence. The wrongness. Impressive, really.",
        "Sure, try that. Let me know how it goes. (I already know. It went badly.)",
        "I respect the hustle. Not the code, but definitely the hustle.",
    ];

    choose(&messages).to_string()
}

/// Generate a goodbye message.
/// The Goose bids farewell. Until next time.
pub fn goodbye() -> String {
    let messages = [
        "Goodbye! May your future code be properly quacked. Unlike... some of today's.",
        "*flies away into the sunset* Until next time! *crashes into tree* I'm okay!",
        "Goose out. *drops mic* *picks up mic* *puts it back properly*",
        "Farewell, programmer. The pond calls me home. The pond is my bed. I'm tired.",
        "Session ended. I'll be here if you need me. Judging. Always judging.",
        "Bye! Don't forget to quack in your dreams. It's good practice.",
        "*tips wing* It's been... something. Goodbye for now.",
        "The goose departs. Your code remains. Make it count. Make it quack.",
        "Shutting down. Remember: quack responsibly. Drink water. Touch grass.",
        "Goodbye! *aggressive farewell honk* HONK!",
        "End of session. Go outside. Pet a real goose. (Actually don't, they bite. I bite.)",
        "Farewell! May your compile times be short and your bugs be obvious.",
        "Session complete. I'm going to go stare at some bread now. Goodbye.",
        "The goose returns to the void. The void is my lunch break. Bye!",
    ];

    choose(&messages).to_string()
}

/// Generate a honk assertion failure message.
/// The Goose is NOT pleased. Your assertion was lies.
pub fn honk_failure(line: usize, custom_message: &str) -> String {
    if !custom_message.is_empty() {
        let prefixes = [
            format!("HONK! Line {}: {} (The goose is upset)", line, custom_message),
            format!("HONK HONK! Assertion failed at line {}: {} *aggressive wing flapping*", line, custom_message),
            format!("*AGGRESSIVE HONKING* Line {}: {} (This is bad)", line, custom_message),
            format!("The goose is DISPLEASED! Line {}: {} *angry waddle*", line, custom_message),
            format!("🚨 HONK ALERT 🚨 Line {}: {}", line, custom_message),
        ];
        return choose(&prefixes).clone();
    }

    let messages = [
        format!("HONK! Assertion failed at line {}. The goose is NOT happy. Not even a little.", line),
        format!("HONK HONK HONK! Your assumption at line {} was WRONG!", line),
        format!("*aggressive honking* Line {}: That condition is FALSE! How dare it!", line),
        format!("Line {}: The goose inspected your assertion. It was LIES. ALL LIES.", line),
        format!("HONK! Line {}: The goose trusted you. The goose was BETRAYED.", line),
        format!("Line {}: *slams wing on table* THIS. IS. FALSE!", line),
        format!("ASSERTION FAILURE at line {}! The council of geese is OUTRAGED!", line),
        format!("Line {}: HONK! Your boolean is broken! How did you break a boolean?!", line),
        format!("*honks in disappointment* Line {}: That's not true and you know it.", line),
        format!("Line {}: The goose has spoken. Your assertion is invalid. You are invalid.", line),
        format!("Line {}: HONK HONK! *knocks over your debugging session* FALSE!", line),
        format!("Line {}: Assertion failed! The goose demands answers! (And bread!)", line),
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

    #[test]
    fn test_encouragement_not_empty() {
        let msg = encouragement();
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_sass_not_empty() {
        let msg = sass();
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_honk_failure_with_message() {
        let msg = honk_failure(10, "custom error");
        assert!(msg.contains("custom error") || msg.contains("10"));
    }

    #[test]
    fn test_honk_failure_without_message() {
        let msg = honk_failure(10, "");
        assert!(msg.contains("10") || msg.contains("HONK"));
    }
}
