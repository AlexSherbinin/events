use std::collections::HashMap;
use std::hash::Hash;

pub mod event_observer;
pub mod multiple_handlers;

type EventHandlerBoxed<Event> = Box<dyn EventHandler<Event>>;
type MutEventHandlerBoxed<Event> = Box<dyn MutEventHandler<Event>>;

pub trait EventHandler<Event> {
    fn handle_event(&self, event: &Event);
}
pub trait MutEventHandler<Event> {
    fn handle_event(&mut self, event: &Event);
}

pub struct EventBus<EventKey, Event>
where
    EventKey: Hash + Eq,
{
    pub event_handlers: HashMap<EventKey, EventHandlerBoxed<Event>>,
    pub mut_event_handlers: HashMap<EventKey, MutEventHandlerBoxed<Event>>,
}

impl<EventKey, Event> EventBus<EventKey, Event>
where
    EventKey: Hash + Eq,
{
    pub fn new() -> Self {
        EventBus {
            event_handlers: HashMap::new(),
            mut_event_handlers: HashMap::new(),
        }
    }
    pub fn publish_event(&self, event_key: &EventKey, event: &Event) {
        if let Some(subscriber) = self.event_handlers.get(event_key) {
            subscriber.handle_event(event);
        }
    }
    pub fn publish_event_mut(&mut self, event_key: &EventKey, event: &Event) {
        if let Some(subscriber) = self.mut_event_handlers.get_mut(event_key) {
            subscriber.handle_event(event);
        }
        self.publish_event(event_key, event);
    }
}