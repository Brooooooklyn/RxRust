
pub fn create<T>(next: fn(value: T) -> ()) -> Subscriber<T> {
  Subscriber { next, is_stopped: false }
}

#[derive(Debug)]
pub struct Subscriber<T> {
  is_stopped: bool,
  pub next: fn(value: T) -> ()
}

