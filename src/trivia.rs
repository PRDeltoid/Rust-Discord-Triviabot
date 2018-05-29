use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use typemap::Key;

use db;
use optionset::OptionSet;
use questionset::QuestionSet;
use scores::Scores;

/// The TriviaManager holds the current gamestate
pub struct TriviaManager {
    pub running: bool,
    question_set: Option<QuestionSet>,
    channel: Option<ChannelId>,
    scores: Option<Scores>,
    skips: u32,
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
            question_set: None,
            channel: None,
            scores: None, //Scores::new(),
            skips: 0,
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

                self.question_set = Some(db::get_question_set(optionset));

                //Tell the user we've started and ask a question
                self.say_raw("Trivia Running");
                self.ask_question();
            }
            true => {
                self.say_raw("Trivia is already running");
            }
        };
    }

    /// Stops the trivia bot
    pub fn stop(&mut self) {
        let text = match self.running {
            true => {
                self.running = false;
                self.print_scores();
                String::from("Trivia Stopping")
            }
            false => String::from("Trivia is not running"),
        };
        self.say(text);
    }

    /// Skips the current question on the triviabot
    pub fn skip(&mut self) {
        match self.running {
            true => {
                self.skips += 1;
                if self.skips >= 3 {
                    self.say_raw("Skipping question.");

                    self.question_set
                        .as_mut()
                        .expect("Error getting questionset")
                        .next_question();

                    self.ask_question();
                } 
                    self.say_raw(format!("Votes Needed: {}/3", self.skips).as_str());
            }
            false => {
                self.say_raw("Can't skip because trivia is not running");
            }
        };
    }

    /// Method which runs whenever a new message is recieved.
    ///
    /// If the triviabot is running, the text is checked to see if it is an answer
    pub fn on_message(&mut self, message: Message) {
        if self.running {
            let correct = self.check_answer(message.content);
            if correct {
                self.say_raw("Correct");

                self.scores
                    .as_mut()
                    .expect("Error getting scores in on_message()")
                    .increase_score(message.author, 1);

                self.question_set
                    .as_mut()
                    .expect("Error getting questionset in on_message()")
                    .next_question();

                self.ask_question();
            }
        }
    }

    /// Sets the channel where the triviabot will send it's messages
    pub fn set_channel(&mut self, message: &Message) {
        self.channel = Some(message.channel_id);
    }

    // Prints out the scorelist to the current channel
    fn print_scores(&self) {
        let scores = self.scores
                        .as_ref()
                        .expect("Error getting scores")
                        .output_scores();
        self.say(scores);
    }

    // Sends a message to the active trivia channel with the current question
    // When no more questions are available, this method calls the stop() method
    fn ask_question(&mut self) {
        // If question is false, there was no question to ask
        let question = match self.question_set
                                .as_ref()
                                .expect("Error getting questionset in ask_question()")
                                .get_current_question() 
        {
            Some(q) => {
                let prompt = format!("Question: {}", q.prompt);
                self.say_raw(&prompt);
                println!("Answer: {}", q.answer);
                true
            }
            None => {
                self.say_raw("Out of questions");
                false
            }
        };

        // Stop if we don't have any more questions to ask
        if question == false {
            self.stop();
        }
    }

    // Checks if a given string matches the current question's answer
    fn check_answer(&mut self, message: String) -> bool {
        let question = self.question_set
                            .as_ref()
                            .expect("Error getting question set")
                            .get_current_question();

        match question {
            Some(q) => {
                if message.to_lowercase() == q.answer.to_lowercase() {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    // Sends a message to the currently set channel
    // Accepts a raw Str object
    // If no channel exists, outputs an error message to the console
    fn say_raw(&self, message: &str) {
        match self.channel {
            Some(channel) => {
                let _ = channel.say(message);
            }
            None => {
                println!("Error. Tried to use say without a channel set");
            }
        }
    }

    // Sends a message to the currently set channel
    // Accepts a String object
    // If no channel exists, outputs an error message to the console
    //
    fn say(&self, message: String) {
        match self.channel {
            Some(channel) => {
                let _ = channel.say(message.as_str());
            }
            None => {
                println!("Error. Tried to use say without a channel set");
            }
        }
    }
}
