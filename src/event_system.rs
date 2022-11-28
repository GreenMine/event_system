use std::collections::HashMap;

pub trait Event {}

impl Event for () {}

pub trait Handler {
    type Event: Event;

    fn process(&mut self, message: &Self::Event);
}

pub trait HandlerInit: Handler {
    fn init() -> Self
    where
        Self: Sized;
}

type HandlerContainer = Box<dyn Handler<Event = ()>>;
//FIXME: index will be reordered after delete
type HandlerId = usize;

pub struct Subscriber {
    handlers: HashMap<&'static str, Vec<HandlerContainer>>,
}

impl Subscriber {
    pub fn new() -> Self {
        Subscriber {
            handlers: HashMap::new(),
        }
    }

    //unsafe wrapped by trait bound
    pub fn add_uninit_handler<J: Handler>(&mut self) -> HandlerId
    where
        J: Handler + HandlerInit,
        <J as Handler>::Event: Event,
    {
        unsafe { self.push_handler(J::init()) }
    }

    //unsafe wrapped by trait bound
    pub fn add_handler<J: Handler>(&mut self, handler: J) -> HandlerId
    where
        J: Handler,
        <J as Handler>::Event: Event,
    {
        unsafe { self.push_handler(handler) }
    }

    pub fn remove_handler(&mut self, _handler_id: HandlerId) -> Option<HandlerContainer> {
        None
        // Some(self.handlers.remove(handler_id).1)
    }

    unsafe fn push_handler<J: Handler>(&mut self, handler: J) -> HandlerId {
        let message_name = std::any::type_name::<J::Event>();
        let handle = Self::cast_handler(handler);
        self.handlers
            .entry(message_name)
            .or_insert(Vec::new())
            .push(handle);

        usize::MAX
    }

    unsafe fn cast_handler<J: Handler>(mut j: J) -> Box<dyn Handler<Event = ()>> {
        let r#box = Box::new(j) as Box<dyn Handler<Event = <J as Handler>::Event>>;
        std::mem::transmute::<_, Box<dyn Handler<Event = ()>>>(r#box)
    }

    pub fn run<M>(&mut self, message: M)
    where
        M: Event,
        Self: Sized,
    {
        let message_name = std::any::type_name::<M>();
        if let Some(handlers) = self.handlers.get_mut(message_name) {
            let msg = unsafe { std::mem::transmute::<_, &()>(&message) };
            handlers.iter_mut().for_each(|f| f.process(msg));
        }
    }
}
