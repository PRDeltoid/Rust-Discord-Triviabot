use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use typemap::Key;

use optionset::OptionSet;
use questionset::QuestionSet;
use question::Question;
use db;

pub struct TriviaManager {
    pub running: bool,
    question_set: QuestionSet,
    channel: Option<ChannelId>,
}


impl Key for TriviaManager {
    type Value = TriviaManager;
}

/// This object keeps track of the current gamestate
impl TriviaManager {

    /// Generates a new trivia manager. Channel and running are unset by default.
    pub fn new() -> TriviaManager {
        let questions: Vec<Question> = Vec::new();
        TriviaManager {
            running: false,
            question_set: QuestionSet::new(questions, 1),
            channel: None,
        }
    }

    /// Starts the trivia bot
    ///
    /// This function sets the `channel` member, which is used for sending messages during a game 
    pub fn start(&mut self, optionset: OptionSet) {
        match self.running {
            false => {
                //Configure the trivia manager
                self.running = true;

                self.question_set = db::get_question_set(optionset);

                //Tell the user we've started and ask a question
                self.say("Trivia Running");
                self.ask_question();
            },
            true => {
                self.say("Trivia is already running");
            },
        };
    }

    /// Stops the trivia bot
    pub fn stop(&mut self) {
        let text = match self.running {
            true => {
                self.running = false;
                String::from("Trivia Stopping")
            },
            false => {
                String::from("Trivia is not running")
            },
        };
        self.say(text.as_str());
    }
    
    /// Handler for an unrecognized trivia command
    pub fn unrecognized_command(&self, message: &Message) {
        let _ = message.channel_id.say("Invalid Command");
    }

    /// Method which runs whenever a new message is recieved.
    ///
    /// If the triviabot is running, the text is checked to see if it is an answer
    pub fn on_message(&mut self, message: Message) {
        if self.running {
            let correct = self.check_answer(message.content);
            if correct {
                let _ = self.channel.unwrap().say("Correct");
                self.question_set.next_question();
                self.ask_question();
            }
        }
    }

    pub fn set_channel(&mut self, message: &Message) {
        self.channel = Some(message.channel_id);
    }


    /// Sends a message to the active trivia channel with the current question
    /// When no more questions are available, this method calls the stop() method
    fn ask_question(&mut self) {
        //If question is false, there was no question to ask
        let question = match self.question_set.get_current_question() {
            Some(q) => {
                let question = format!("Question: {}", q.prompt);
                self.say(&question);
                println!("Answer: {}", q.answer);
                true
            },
            None => {
                self.say("Out of questions");
                false
            }
        };

        //Stop if we don't have any more questions to ask
        if question == false {
            self.stop();
        }

    }

    /// Checks if a given string matches the current question's answer
    fn check_answer(&mut self, message: String) -> bool {
        let question = self.question_set.get_current_question();

        match question {
            Some(q) => {
                if message == q.answer {
                    true
                } else {
                    false 
                }
            },
            None => false
        }
    }

    /// Sends a message to the currently set channel
    /// If no channel exists, outputs an error message to the console 
    /// TODO: Add real error handling here
    fn say(&self, message: &str) {
        match self.channel {
            Some(_channel) => { let _ = self.channel.unwrap().say(message); },
            None => { println!("Error. Tried to use say without a channel set"); },
        }

    }

}
