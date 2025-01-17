use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Date {
    num: usize,
}

fn update_set() {
    let mut set: HashSet<Date> = HashSet::new();
    set.insert(Date { num: 1 });
}

mod test {
    use super::*;

    #[test]
    fn test_update_set() {
        update_set();
    }

    #[test]
    fn test_update_set_size() {
        let mut set: HashSet<Date> = HashSet::new();
        set.insert(Date { num: 1 });
        set.insert(Date { num: 1 });
        assert_eq!(set.len(), 1);
    }
}
