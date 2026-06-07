use std::{collections::HashMap, marker::PhantomData};

pub type EventKey = u128;

pub struct Event<Callback, Args>
where
    Callback: Fn(&Args) -> (),
{
    callbacks: HashMap<EventKey, Callback>,
    phant: std::marker::PhantomData<Args>,
}

impl<Callback, Args> Event<Callback, Args>
where
    Callback: Fn(&Args) -> (),
{
    /// Initializes an empty set of events
    pub fn empty() -> Event<Callback, Args> {
        Event {
            callbacks: HashMap::new(),
            phant: PhantomData::default(),
        }
    }

    /// Add a new callback to this event. Returns an Event Key.
    #[must_use]
    pub fn add(&mut self, callback: Callback) -> EventKey {
        let event_key: EventKey = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time rewound")
            .as_nanos()
            + (self.callbacks.len() as u128);

        self.callbacks.insert(event_key, callback);

        dbg!(self.callbacks.len());

        event_key
    }

    /// Removes a function callback based on the event key
    pub fn remove(&mut self, key: EventKey) -> Option<Callback> {
        self.callbacks.remove(&key)
    }

    /// Invokes the events that are current added
    pub fn invoke(&self, args: &Args) -> () {
        for (_, e) in &self.callbacks {
            e(args);
        }
    }
}
