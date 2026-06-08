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

    /// # Is Empty
    ///
    /// True if there are no callbacks for this event.
    pub fn is_empty(&self) -> bool {
        self.callbacks.is_empty()
    }

    /// # Keys
    ///
    /// The collection of keys
    pub fn keys(&self) -> Vec<&EventKey> {
        self.callbacks.keys().collect()
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

/// # Event Type
///
/// Macro that allows you to easily create an Event Type
///
/// For example:
///
/// ```
/// # use flake::events::Event;
/// # use flake::event_type;
/// # use std::boxed::Box;
/// # use std::ops::Fn;
/// pub type CustomEvent = Event<Box<dyn Fn(i32)>, i32>;
/// // produces the same event as above
/// pub type MCustomEvent = event_type!(i32);
/// // 'e_lifetime represents the life time of the Fn
/// pub type MLCustomEvent<'e_lifetime> = event_type!(i32, 'e_lifetime);
/// // thread safe (Send + Sync + 'static) fn lifetime
/// pub type MTCustomEvent = event_type!(i32, TSafe);
///
/// ```
#[macro_export]
//Event<Box<dyn Fn(&Scroll) + Send + Sync + 'static>, Scroll>
macro_rules! event_type {
    ($args:ty) => {
        $crate::events::Event<::std::boxed::Box<dyn ::std::ops::Fn(&$args)>, $args>
    };
    ($args:ty, $life:lifetime) => {
        $crate::events::Event<::std::boxed::Box<dyn ::std::ops::Fn(&$args) + $life>, $args>
    };
    ($args:ty, TSafe) => {
        $crate::events::Event<::std::boxed::Box<dyn ::std::ops::Fn(&$args) + Send + Sync + 'static>, $args>
    };
}
