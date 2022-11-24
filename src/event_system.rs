pub trait Message {}

impl Message for () {}

pub trait Job {
    type ItemMessage: Message;

    fn init() -> Self
    where
        Self: Sized;
    fn process(&mut self, message: &Self::ItemMessage);
}

pub struct Subscriber {
    handlers: Vec<(&'static str, Box<dyn Job<ItemMessage = ()>>)>,
}

impl Subscriber {
    pub fn new() -> Self {
        Subscriber {
            handlers: Vec::new(),
        }
    }

    //unsafe wrapped by trait bound
    pub fn add_uninit_handler<J: 'static + Job>(&mut self)
    where
        J: Job,
        <J as Job>::ItemMessage: Message,
    {
        unsafe { self.push_handler(J::init()) }
    }

    //unsafe wrapped by trait bound
    pub fn add_handler<J: 'static + Job>(&mut self, handler: J)
    where
        J: Job,
        <J as Job>::ItemMessage: Message,
    {
        unsafe { self.push_handler(handler) }
    }

    unsafe fn push_handler<J: 'static + Job>(&mut self, handler: J) {
        let message_name = std::any::type_name::<J::ItemMessage>();
        self.handlers
            .push((message_name, Self::cast_handler(handler)))
    }

    unsafe fn cast_handler<J: 'static + Job>(mut j: J) -> Box<dyn Job<ItemMessage = ()>> {
        let r#box = Box::new(j) as Box<dyn Job<ItemMessage = <J as Job>::ItemMessage>>;
        std::mem::transmute::<_, Box<dyn Job<ItemMessage = ()>>>(r#box)

        // unsafe {
        //     ((&j as &dyn Job<ItemMessage = <J as Job>::ItemMessage> as *const _
        //         as *mut dyn Job<ItemMessage = ()>)
        //         .as_mut()
        //         .unwrap())
        // }
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
