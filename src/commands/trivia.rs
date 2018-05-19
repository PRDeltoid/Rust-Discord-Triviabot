use trivia;

command!(trivia_handler(context, message, args) {
    let mut data = context.data.lock();

    let trivia_manager = data.get_mut::<trivia::TriviaManager>().unwrap();

    //Parse the trivia command given
    let command = args.single::<String>().unwrap();

    //Do an action based on the command
    match command.as_ref() {
        "start" => { trivia_manager.start(&message); },
        "stop"  => { trivia_manager.stop(&message); },
        _       => { trivia_manager.unrecognized_command(&message); },
    };
});

