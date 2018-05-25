use trivia;
use optionset::OptionSet;

command!(trivia_handler(context, message, args) {
    let mut data = context.data.lock();

    let trivia_manager = data.get_mut::<trivia::TriviaManager>().unwrap();

    //Parse the trivia command given
    let command = args.single::<String>().unwrap();

    //Do an action based on the command
    match command.as_ref() {
        "start" => { 
            let optionset;
            let number_of_questions;
            let difficulty;

            //Have to check if there are zero extra args or Serenity panics
            if args.len() == 0 {
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
        },
        "stop"  => { 
            trivia_manager.set_channel(message);
            trivia_manager.stop(); 
        },
        _       => { trivia_manager.unrecognized_command(message); },
    };
});

