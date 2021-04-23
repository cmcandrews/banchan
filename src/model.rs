use message::Message;

pub trait Model {
    fn view(&self) -> String;
    fn update(&self, message: Message) -> String;
}