use std::collections::HashMap;

pub trait Message {}

impl Message for () {}

pub trait Job {
    type ItemMessage: Message;

    fn process(&mut self, message: &Self::ItemMessage);
}

pub trait JobInit: Job {
    fn init() -> Self
    where
        Self: Sized;
}

type Handler = Box<dyn Job<ItemMessage = ()>>;
//FIXME: index will be reordered after delete
type HandlerId = usize;

pub struct Subscriber {
    handlers: HashMap<&'static str, Vec<Handler>>,
}

impl Subscriber {
    pub fn new() -> Self {
        Subscriber {
            handlers: HashMap::new(),
        }
    }

    //unsafe wrapped by trait bound
    pub fn add_uninit_handler<J: Job>(&mut self) -> HandlerId
    where
        J: Job + JobInit,
        <J as Job>::ItemMessage: Message,
    {
        unsafe { self.push_handler(J::init()) }
    }

    //unsafe wrapped by trait bound
    pub fn add_handler<J: Job>(&mut self, handler: J) -> HandlerId
    where
        J: Job,
        <J as Job>::ItemMessage: Message,
    {
        unsafe { self.push_handler(handler) }
    }

    pub fn remove_handler(&mut self, _handler_id: HandlerId) -> Option<Handler> {
        None
        // Some(self.handlers.remove(handler_id).1)
    }

    unsafe fn push_handler<J: Job>(&mut self, handler: J) -> HandlerId {
        let message_name = std::any::type_name::<J::ItemMessage>();
        let handle = Self::cast_handler(handler);
        self.handlers
            .entry(message_name)
            .or_insert(Vec::new())
            .push(handle);

        usize::MAX
    }

    unsafe fn cast_handler<J: Job>(mut j: J) -> Box<dyn Job<ItemMessage = ()>> {
        let r#box = Box::new(j) as Box<dyn Job<ItemMessage = <J as Job>::ItemMessage>>;
        std::mem::transmute::<_, Box<dyn Job<ItemMessage = ()>>>(r#box)
    }

    pub fn run<M>(&mut self, message: M)
    where
        M: Message,
        Self: Sized,
    {
        let message_name = std::any::type_name::<M>();
        if let Some(handlers) = self.handlers.get_mut(message_name) {
            handlers.iter_mut().for_each(|f| {
                f.process(unsafe {
                    (&message as *const dyn Message as *const ())
                        .as_ref()
                        .unwrap()
                })
            });
        }
    }
}
