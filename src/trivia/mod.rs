use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use typemap::Key;

pub mod question;
pub mod questionset;
use super::trivia::questionset::QuestionSet;

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
        TriviaManager {
            running: false,
            question_set: QuestionSet::new(0),
            channel: None,
        }
    }

    /// Starts the trivia bot
    ///
    /// This function sets the `channel` member, which is used for sending messages during a game 
    pub fn start(&mut self, message: &Message) {
        match self.running {
            false => {
                //Configure the trivia manager
                self.running = true;
                self.channel = Some(message.channel_id);

                self.question_set.generate_questions();

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
    pub fn stop(&mut self, message: &Message) {
        let text = match self.running {
            true => {
                self.running = false;
                String::from("Trivia Stopping")
            },
            false => {
                String::from("Trivia is not running")
            },
        };
        let _ = message.channel_id.say(text);
    }
    
    /// Handler for an unrecognized trivia command
    pub fn unrecognized_command(&self, message: &Message) {
        let _ = message.channel_id.say("Invalid Command");
    }

    /// Method which runs whenever a new message is recieved.
    ///
    /// If the triviabot is running, the text is checked to see if it is an answer
    pub fn on_message(&self, message: Message) {
        if self.running {
            let correct = self.check_answer(message.content);
            if correct {
                let _ = self.channel.unwrap().say("Correct");
            }
        }
    }

    /// Sends a message to the active trivia channel with the current question
    fn ask_question(&mut self) {
        let question = self.question_set.get_current_question();
        let question = format!("Question: {}", question.prompt);
        self.say(&question);
    }

    /// Checks if a given string matches the current question's answer
    fn check_answer(&self, _message: String) -> bool {
        /*TODO let question = self.question_set.get_current_question();
        if message == question.answer {
            true
        } else {
            false 
        }*/
        true
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
