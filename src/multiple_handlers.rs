use crate::{EventHandler, EventHandlerBoxed, MutEventHandler, MutEventHandlerBoxed};

pub struct MultipleEventHandlersRunner<EventType> {
    pub event_handlers: Vec<EventHandlerBoxed<EventType>>,
}
impl<EventType> MultipleEventHandlersRunner<EventType> {
    pub fn new() -> Self{
        Self {event_handlers: Vec::new()}
    }
}
impl<EventType> EventHandler<EventType> for MultipleEventHandlersRunner<EventType> {
    fn handle_event(&self, event: &EventType) {
        for event_handler in self.event_handlers.iter() {
            event_handler.handle_event(event);
        }
    }
}
pub struct MutMultipleEventHandlersRunner<EventType> {
    pub event_handlers: Vec<MutEventHandlerBoxed<EventType>>,
}
impl<EventType> MutMultipleEventHandlersRunner<EventType> {
    pub fn new() -> Self {
        Self {event_handlers: Vec::new()}
    }
}
impl<EventType> MutEventHandler<EventType> for MutMultipleEventHandlersRunner<EventType> {
    fn handle_event(&mut self, event: &EventType) {
        for event_handler in self.event_handlers.iter_mut() {
            event_handler.handle_event(event);
        }
    }
}
