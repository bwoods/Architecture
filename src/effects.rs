use flume::Sender;

pub trait Effects {
    type Action;

    fn send(&self, action: impl Into<Self::Action>);
}

#[doc(hidden)]
impl<Action> Effects for Sender<Action> {
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: impl Into<Action>) {
        let _ = self.send(action.into());
    }
}
