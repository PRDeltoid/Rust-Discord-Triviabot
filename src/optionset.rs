pub struct OptionSet {
    pub number_of_questions: u32,
    pub difficulty: String,
    pub categories: Vec<String>,
}

impl OptionSet {
    pub fn new() -> OptionSet {
        OptionSet {
            number_of_questions: 10,
            difficulty: "Easy".to_string(),
            categories: Vec::new(),
        }
    }
}
