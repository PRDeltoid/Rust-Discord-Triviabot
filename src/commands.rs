use optionset::OptionSet;
use trivia;

// This command is run when the start command is executed
command!(trivia_start(context, message, args) {
    let optionset;
    let mut number_of_questions;
    let difficulty;

    let mut data = context.data.lock();
    let trivia_manager = data.get_mut::<trivia::TriviaManager>().expect("Error getting TriviaManager from bot data");

    // Have to check if there are zero extra args or Serenity panics on find()
    if args.is_empty() {
        number_of_questions = 10;
        difficulty = String::from("medium");
    } else {
        //println!("{:?}", args);
        number_of_questions = args.find::<u32>().unwrap_or(10);
        difficulty = args.find::<String>().unwrap_or_else(|_| "medium".to_string());
        //println!("Num: {}, Diff: {}", number_of_questions, difficulty);
    }

    optionset = OptionSet {
        number_of_questions,
        difficulty,
        category: String::from(""),
        channel: message.channel_id,
    };

    trivia_manager.set_channel(message);
    trivia_manager.start(&optionset);
});

// This command is run when the stop command is executed
command!(trivia_stop(context, message, _args) {
    let mut data = context.data.lock();
    let trivia_manager = data.get_mut::<trivia::TriviaManager>().expect("Error getting TriviaManager from bot data");

    trivia_manager.set_channel(message);
    trivia_manager.stop();
});

// This command is run when the skip command is executed
command!(trivia_skip(context, message, _args) {
    let mut data = context.data.lock();
    let trivia_manager = data.get_mut::<trivia::TriviaManager>().expect("Error getting TriviaManager from bot data");

    trivia_manager.vote_skip(message);
});
