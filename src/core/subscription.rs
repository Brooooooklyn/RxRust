#[derive(Debug)]
pub struct Subscription {
  closed: bool,
  subscriptions: Box<Vec<Subscription>>,
}

impl Subscription {
  pub fn create() -> Subscription {
    Subscription { closed: false, subscriptions: Box::new(vec![]) }
  }

  pub fn closed(&self) -> bool {
    self.closed
  }

  pub fn add(mut self, teardown_logic: Subscription) -> Subscription {
    self.subscriptions.push(teardown_logic);
    self
  }

  pub fn unsubscribe(&mut self) -> () {
    self.closed = true;
  }
}
