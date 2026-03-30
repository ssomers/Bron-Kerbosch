use super::pile::Pile;

#[test]
fn empty() {
    assert!(Pile::<bool>::EMPTY.iter().next().is_none());
}

#[test]
fn one_level() {
    let p1 = Pile::from(());
    assert_eq!(p1.iter().copied().collect::<Vec<_>>(), vec![()]);
}

#[test]
fn two_levels() {
    let p1 = Pile::from(22);
    {
        let p2 = p1.pile(11);
        assert_eq!(p2.iter().copied().collect::<Vec<_>>(), vec![11, 22]);
    }
    assert_eq!(p1.iter().copied().collect::<Vec<_>>(), vec![22]);
}
