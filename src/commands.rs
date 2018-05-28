use optionset::OptionSet;
use trivia;

command!(trivia_start(context, message, args) {
    let optionset;
    let number_of_questions;
    let difficulty;

    let mut data = context.data.lock();
    let trivia_manager = data.get_mut::<trivia::TriviaManager>().expect("Error getting TriviaManager from bot data");

    //Have to check if there are zero extra args or Serenity panics on find()
    if args.is_empty() {
        number_of_questions = 10;
        difficulty = String::from("medium");
    } else {
        number_of_questions = match args.find::<u32>() {
            Ok(s) => s,
            Err(_e) => 10, //default number of questions
        };
        difficulty = match args.find::<String>() {
            Ok(diff) => diff,
            Err(_e) => String::from("medium"), //default question difficulty
        };
        //println!("Num: {}, Diff: {}", number_of_questions, difficulty);
    }

    optionset = OptionSet {
        number_of_questions: number_of_questions,
        difficulty: difficulty,
        category: String::from(""),
    };

    trivia_manager.set_channel(message);
    trivia_manager.start(optionset);
});

command!(trivia_stop(context, message, _args) {
    let mut data = context.data.lock();
    let trivia_manager = data.get_mut::<trivia::TriviaManager>().expect("Error getting TriviaManager from bot data");

    trivia_manager.set_channel(message);
    trivia_manager.stop();
});

command!(trivia_skip(context, _message, _args) {
    let mut data = context.data.lock();
    let trivia_manager = data.get_mut::<trivia::TriviaManager>().expect("Error getting TriviaManager from bot data");

    trivia_manager.skip();
});
