pub mod core;

#[cfg(test)]
mod tests {
    use core::observable;
    #[test]
    fn test_observable() {
        let o = observable::Observable::create(|observer| {
            (observer.next)(1);
        });
        let mut subscription = o.subscribe(|x| {
            assert_eq!(x, 1);
        });

        subscription.unsubscribe();
    }
}
