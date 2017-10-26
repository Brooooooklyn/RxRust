use super::subscriber::*;
use super::subscription::*;

pub struct Observable<T> {
  observer: fn(Subscriber<T>) -> (),
}

impl <T> Observable<T> {
  pub fn create(observer: fn(subscriber: Subscriber<T>) -> ()) -> Observable<T> {
    Observable { observer }
  }
}

impl <T> Observable<T> {
  pub fn subscribe(&self, next: fn(value: T) -> ()) -> Subscription {
    let subscriber = Subscriber::create(next);
    (self.observer)(subscriber);
    Subscription::create()
  }
}
