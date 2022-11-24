pub trait Message {}

pub trait Job {
    type ItemMessage: Message;

    fn init() -> Self
    where
        Self: Sized;
    fn process(message: &Self::ItemMessage);
}

type Handler = dyn Fn(&dyn Message) -> ();
pub struct Subscriber {
    handlers: Vec<(&'static str, Box<Handler>)>,
}

impl Subscriber {
    pub fn new() -> Self {
        Subscriber {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler<J: 'static + Job>(&mut self)
    where
        J: Job,
        <J as Job>::ItemMessage: Message,
    {
        let message_name = std::any::type_name::<J::ItemMessage>();
        self.handlers.push((
            message_name,
            Box::new(move |message| {
                let actual_ref = unsafe {
                    (message as *const dyn Message as *const J::ItemMessage)
                        .as_ref()
                        .unwrap()
                };
                J::process(actual_ref)
            }),
        ))
    }

    pub fn run<M>(&mut self, message: M)
    where
        M: Message + 'static,
    {
        let message_name = std::any::type_name::<M>();
        self.handlers
            .iter()
            .filter(|(name, _)| message_name == *name)
            .for_each(|(_, f)| f(&message));
    }
}
