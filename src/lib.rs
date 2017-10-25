pub mod core;

#[cfg(test)]
mod tests {
    use core;
    #[test]
    fn test_observable() {
        let o = core::observable::create(|observer| {
            (observer.next)(1);
        });
        o.subscribe(|x| {
            assert_eq!(x, 1);
        });
    }
}
