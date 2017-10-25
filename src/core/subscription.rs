pub trait subscription {
  fn closed(&self) -> bool;
  fn add(&self, teardown_logic: Subscription) -> Subscription;
  fn unsubscribe(&self) -> ();
}

#[derive(Debug)]
pub struct Subscription {
  closed: bool,
  subscriptions: Box<Vec<Subscription>>,
}
