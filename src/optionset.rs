pub struct OptionSet {
    pub number_of_questions: u32,
    pub difficulty: String,
    pub category: String,
}

impl OptionSet {
    pub fn new() -> OptionSet {
        OptionSet {
            number_of_questions: 10,
            difficulty: "easy".to_string(),
            category: "".to_string(),
        }
    }
}
