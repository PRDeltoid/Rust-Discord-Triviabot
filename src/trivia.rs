use serenity::model::channel::Message;
use serenity::model::id::{ ChannelId, UserId };
use serenity::model::user::User;
use typemap::Key;
use std::collections::HashMap;
use std::fmt::Display;

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
    user_answered_list: HashMap<UserId, bool>,
    user_skipped_list: HashMap<UserId, bool>,
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
            scores: None,
            skips: 0,
            user_answered_list: HashMap::new(),
            user_skipped_list: HashMap::new(),
        }
    }

    /// Starts the trivia bot
    ///
    /// This function sets the `channel` member, which is used for sending messages during a game
    pub fn start(&mut self, optionset: &OptionSet) {
        if !self.running {
            //Configure the trivia manager
            self.running = true;
            self.question_set = Some(db::get_question_set(&optionset));
            self.channel = Some(optionset.channel);
            self.scores = Some(Scores::new());

            //Tell the user we've started and ask a question
            self.say("Trivia Starting");
            self.ask_question();
        } else {
            self.say("Trivia is already running");
        }
    }

    /// Stops the trivia bot
    pub fn stop(&mut self) {
        if self.running {
            self.print_scores();
            self.say("Trivia Stopping");
            self.running = false;
            self.question_set = None;
            self.channel = None;
            self.scores = None;
        } else {
            self.say("Trivia is not running");
        }
    }

    /// Skips the current question on the triviabot
    pub fn vote_skip(&mut self, message: &Message) {
        if self.running {
            // Check if the user has already skipped
            // If so, exit early and say nothing
            if self.has_skipped(&message.author) {
                return;
            }
            self.skips += 1;
            if self.skips >= 3 {
                self.say("Skipping question.");

                self.next_question();
                self.ask_question();
            } else {
                self.say(format!("{} voted to skip. **Votes Needed: {}/3**", message.author.name, self.skips).as_str());
            }
        } else {
            self.say("Can't skip because trivia is not running");
        }
    }

    /// Method which runs whenever a new message is recieved.
    ///
    /// If the triviabot is running, the text is checked to see if it is an answer
    pub fn on_message(&mut self, message: Message) {
        if self.running && TriviaManager::valid_letter(&message) {

            //Check if the answer is correct
            let correct = self.check_answer(message.content.as_str());
            //Check if this is the users first guess
            let has_answered = self.has_answered(&message.author);
            //If the answer is correct AND it is the user's first guess, they got the question
            //right
            if correct && !has_answered {
                //Congradulate the user
                self.say(
                    format!("{} got the correct answer", &message.author.name));

                //Increase the user's score by 1
                self.scores
                    .as_mut()
                    .expect("Error getting scores in on_message()")
                    .increase_score(message.author, 1);

                self.next_question();
                self.ask_question();
            }
        }
    }

    fn next_question(&mut self) {
        self.question_set
            .as_mut()
            .expect("Error getting questionset")
            .next_question();
        
        self.user_answered_list.clear();
        self.user_skipped_list.clear();
    }

    /// Sets the channel where the triviabot will send it's messages
    pub fn set_channel(&mut self, message: &Message) {
        self.channel = Some(message.channel_id);
    }

    fn valid_letter(message: &Message) -> bool {
        let message = message.content.to_lowercase();
        if message == "a" ||
            message == "b" ||
            message == "c" ||
            message == "d" {
                true
            } else {
                false
            }
    }
    // Prints out the scorelist to the current channel
    fn print_scores(&self) {
        let scores = self.scores
                        .as_ref()
                        .expect("Error getting scores")
                        .output_scores();
        self.say(scores);
    }

    // Checks AND sets if a user has answered the question
    // Returns true if the user has already answered, and false if they haven't.
    // After this function is run, the passed user will not be able to answer a question again
    // until the answered list is cleared
    fn has_answered(&mut self, user: &User) -> bool {
        //Mark that the given user has attempted to answer the question
        let answered = self.user_answered_list.insert(user.id, true);

        //If answered is a Some value, the user already answered. Return true.
        //If the value is None, this is the users first answer. Return false, the user HASN'T
        //answered yet
        match answered {
            Some(_) => true,
            None => false,
        }

        //After the execution of this function, the user will be marked as having attempted to
        //answer the question
    }

    //Checks AND sets if a user has skipped the current question
    // Returns true if the user has already voted to skip, and false if they haven't.
    // After this functionis run, the passed user will not be able to skip the current question
    // again until the user_skipped_list has been cleared
    fn has_skipped(&mut self, user: &User) -> bool {
        let skipped = self.user_skipped_list.insert(user.id, true);

        match skipped {
            Some(_) => true,
            None    => false,
        }

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
                self.say(format!("```Question: {}\n{}```", &q.prompt, &q.answer_prompt));
                println!("Answer: {}", q.answer);
                true
            }
            None => {
                self.say("Out of questions");
                false
            }
        };

        // Stop if we don't have any more questions to ask
        if !question {
            self.stop();
        }
    }

    // Checks if a given string matches the current question's answer
    fn check_answer(&mut self, message: &str) -> bool {
        let question = self.question_set
                            .as_ref()
                            .expect("Error getting question set")
                            .get_current_question();

        match question {
            Some(q) => {
                //Check if the message is the same as the answer
                message.to_lowercase() == q.answer_letter.to_lowercase() //answer.to_lowercase()
            }
            None => false,
        }
    }

    // Sends a message to the currently set channel
    // Accepts a String object
    // If no channel exists, outputs an error message to the console
    //
    fn say<T: Display>(&self, message: T) {
        let _ = self.channel
            .expect("Tried using say without a channel set")
            .say(format!("{}", message));
    }
}
