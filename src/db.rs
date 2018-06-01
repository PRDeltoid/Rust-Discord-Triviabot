extern crate reqwest;
extern crate serde_json;
extern crate url;

use optionset::OptionSet;
use question::Question;
use questionset::QuestionSet;
use url::Url;
use htmlescape::decode_html;

#[derive(Serialize, Deserialize)]
struct EntrySet {
    response_code: i32,
    results: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    category: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

/// Produces a QuestionSet based on the given OptionSet.
pub fn get_question_set(options: &OptionSet) -> QuestionSet {
    // Grab the number of questions before we consume the optionset
    let number_of_questions = options.number_of_questions;

    // Pull our trivia data as JSON
    let url = compose_url(options).expect("Error creating db URL");
    let json = get_json(url).expect("Error pulling JSON data");
    // println!("JSON: {}\n", json);

    // Create our raw dataset from the JSON
    let res: EntrySet = serde_json::from_str(&json).expect("Error converting JSON to questionset");
    //TODO: Check response code to see if it returned an OK (0) or an error (2)

    // Create an empty questionset
    let mut questions: Vec<Question> = Vec::new();

    // For each result in our raw dataset, create a question and add it to 'questions'
    for result in &res.results {
        let question = Question {
            prompt: decode_html(&result.question).expect("Error decoding a question prompt"),
            answer: decode_html(&result.correct_answer).expect("Error decoding a question answer"),
            category: result.category.clone(),
            difficulty: result.difficulty.clone(),
            answered: false,
        };
        questions.push(question);
    }

    // Return the new questionset
    QuestionSet::new(questions, number_of_questions)
}

// Requests JSON from the given URL and returns it as a String
fn get_json(url: Url) -> Result<String, reqwest::Error> {
    let json = reqwest::get(url)?.text()?;
    Ok(json)
}

// Composes a trivia request URL based on parameters.
fn compose_url(options: &OptionSet) -> Result<Url, url::ParseError> {
    let num = options.number_of_questions.to_string();
    let url = Url::parse_with_params(
        "https://opentdb.com/api.php",
        &[
            ("amount", num),
            ("type", "multiple".to_string()),
            ("difficulty", options.difficulty.clone()),
            ("category", options.category.clone()),
        ],
    )?;

    // println!("URL: {}", url);
    Ok(url)
}
