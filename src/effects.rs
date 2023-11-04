use flume::Sender;

pub trait Effects<Action> {
    fn send(&self, action: impl Into<Action>);
}

#[doc(hidden)]
impl<Action> Effects<Action> for Sender<Action> {
    #[inline(always)]
    fn send(&self, action: impl Into<Action>) {
        let _ = self.send(action.into());
    }
}
