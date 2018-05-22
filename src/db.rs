extern crate serde_json;
extern crate reqwest;
extern crate url;

use trivia::questionset::QuestionSet;

#[derive(Serialize, Deserialize)]
struct EntrySet {
    results: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    category:   String,
    difficulty: String,
    question:   String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

pub fn get_question_set(num_questions: u32) -> QuestionSet {

    QuestionSet::new(5)

}

pub fn test() -> Result<String, serde_json::Error> {
    let text = get_json();

    // Parse the string of json into QuestionSet value
    let v: EntrySet = serde_json::from_str(&text)?;

    //for result in v.results.iter() {
    //    print!("Cat: {}", result.category);
    //}

    //println!("Results: {}", &v);
    Result::Ok(text)
}

///Requests JSON from the given server and returns it as a String
fn get_json() -> String {
    let url = compose_trivia_url();
    let text = reqwest::get(url).unwrap()
        .text().unwrap();
    text
}

///Composes a trivia request URL based on parameters.
fn compose_trivia_url() -> url::Url {
    let url = url::Url::parse_with_params("https://opentdb.com/api.php",
                                      &[("amount","1")])
                                    .unwrap();
    url
}
