pub trait Message {}

impl Message for () {}

pub trait Job {
    type ItemMessage: Message;

    fn init() -> Self
    where
        Self: Sized;
    fn process(&mut self, message: &Self::ItemMessage);
}

type Handler = Box<dyn Job<ItemMessage = ()>>;
//FIXME: index will be reordered after delete
type HandlerId = usize;

pub struct Subscriber {
    handlers: Vec<(&'static str, Handler)>,
}

impl Subscriber {
    pub fn new() -> Self {
        Subscriber {
            handlers: Vec::new(),
        }
    }

    //unsafe wrapped by trait bound
    pub fn add_uninit_handler<J: 'static + Job>(&mut self) -> HandlerId
    where
        J: Job,
        <J as Job>::ItemMessage: Message,
    {
        unsafe { self.push_handler(J::init()) }
    }

    //unsafe wrapped by trait bound
    pub fn add_handler<J: 'static + Job>(&mut self, handler: J) -> HandlerId
    where
        J: Job,
        <J as Job>::ItemMessage: Message,
    {
        unsafe { self.push_handler(handler) }
    }

    pub fn remove_handler(&mut self, handler_id: HandlerId) -> Option<Handler> {
        Some(self.handlers.remove(handler_id).1)
    }

    unsafe fn push_handler<J: 'static + Job>(&mut self, handler: J) -> HandlerId {
        let message_name = std::any::type_name::<J::ItemMessage>();
        self.handlers
            .push((message_name, Self::cast_handler(handler)));

        self.handlers.len() - 1
    }

    unsafe fn cast_handler<J: 'static + Job>(mut j: J) -> Box<dyn Job<ItemMessage = ()>> {
        let r#box = Box::new(j) as Box<dyn Job<ItemMessage = <J as Job>::ItemMessage>>;
        std::mem::transmute::<_, Box<dyn Job<ItemMessage = ()>>>(r#box)
    }

    pub fn run<M>(&mut self, message: M)
    where
        M: Message + 'static,
        Self: Sized,
    {
        let message_name = std::any::type_name::<M>();
        self.handlers
            .iter_mut()
            .filter(|(name, _)| message_name == *name)
            .for_each(|(_, f)| {
                f.process(unsafe {
                    (&message as *const dyn Message as *const ())
                        .as_ref()
                        .unwrap()
                })
            });
    }
}
