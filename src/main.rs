use iced::{text_input, Element, Sandbox, Settings, Text, TextInput};

#[derive(Default)]
struct Note {
    note_string: String,

    note: text_input::State,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl Sandbox for Note {
    type Message = Message;

    // Initializes the Application (Returns the initial state of the Application)
    fn new() -> Self {
        Note {
            note_string: "This is the beginning string".to_string(),
            note: text_input::State::new(),
        }
    }

    fn title(&self) -> String {
        "Amae".to_string()
    }

    // Handles a message and updates the state of the Sandbox
    // (Update logic - messages, which are produced by user interactions, are handled here)
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextInputChanged(new_input) => {
                self.note_string = new_input;
                println!("{}", self.note_string);
            }
        }
    }

    // Returns the widgets to display in the Sandbox
    // An Element is a generic widget
    fn view(&mut self) -> Element<Self::Message> {
        TextInput::new(
            &mut self.note,
            "placeholder",
            &self.note_string,
            Message::TextInputChanged,
        )
        .into()
    }
}

fn main() {
    Note::run(Settings::default());
}
