use std::error::Error;

#[derive(Debug)]
pub struct Subscriber<T> {
  is_stopped: bool,
  pub next: fn(value: T) -> (),
  pub error: fn(value: Box<Error>) -> ()
}

impl <T> Subscriber<T> {
  pub fn create(next: fn(value: T) -> ()) -> Subscriber<T> {
    Subscriber { next, error: | _ | { }, is_stopped: false }
  }
}
