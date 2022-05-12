use crate::{EventHandler, EventHandlerBoxed, MutEventHandler, MutEventHandlerBoxed};

type EventObserverBoxed<EventType> = Box<dyn EventObserver<EventType>>;
type MutEventObserverBoxed<EventType> = Box<dyn MutEventObserver<EventType>>;

pub enum EventState {
    Accept,
    Reject,
}

pub trait EventObserver<EventType> {
    fn observe(&self, event: &EventType) -> EventState;
}

pub struct EventObservingHelper<EventType> {
    observer: EventObserverBoxed<EventType>,
    event_handler: EventHandlerBoxed<EventType>,
}

impl<EventType> EventObservingHelper<EventType> {
    pub fn new(
        observer: EventObserverBoxed<EventType>,
        event_handler: EventHandlerBoxed<EventType>,
    ) -> Self {
        EventObservingHelper {
            observer,
            event_handler,
        }
    }
}

impl<EventType> EventHandler<EventType> for EventObservingHelper<EventType> {
    fn handle_event(&self, event: &EventType) {
        if let EventState::Accept = self.observer.observe(event) {
            self.event_handler.handle_event(event);
        }
    }
}

pub trait MutEventObserver<EventType> {
    fn observe(&mut self, event: &EventType) -> EventState;
}

pub struct MutEventObservingHelper<EventType> {
    observer: MutEventObserverBoxed<EventType>,
    event_handler: MutEventHandlerBoxed<EventType>,
}

impl<EventType> MutEventHandler<EventType> for EventObservingHelper<EventType> {
    fn handle_event(&mut self, event: &EventType) {
        if let EventState::Accept = self.observer.observe(event) {
            self.event_handler.handle_event(event);
        }
    }
}
