
#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message = ChatMessage {
        content:"Hi",
        time: String::from("2025-05-12")
    };
    message.retrieve_time();    


    let audio = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("2025-05-12")
    };

    audio.retrieve_time();
    audio.consume_entertainment();
}