use super::subscriber::*;

pub fn create<T>(observer: fn(subscriber: Subscriber<T>) -> ()) -> Observable<T> {
  Observable { observer }
}

pub struct Observable<T> {
  observer: fn(Subscriber<T>) -> (),
}

impl <T> Observable<T> {
  pub fn subscribe(&self, next: fn(value: T) -> ()) -> () {
    let subscriber = super::subscriber::create(next);
    (self.observer)(subscriber);
  }
}
