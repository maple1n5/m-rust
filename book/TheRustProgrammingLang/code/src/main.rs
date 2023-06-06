struct MockMessenger {
    sent_messages: Vec<String>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: vec![],
        }
    }
}

fn main() {
    let mut mock = MockMessenger::new();
    mock.sent_messages.push("abc".to_string());

}
