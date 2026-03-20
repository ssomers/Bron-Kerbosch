#[cfg(test)]
mod tests {
    use crate::core::main_testing::all_test_data;
    use fnv::FnvHashSet;
    use hashbrown;
    use std::collections::BTreeSet;
    use std::collections::HashSet;

    #[test]
    fn bk_btree() {
        for td in all_test_data() {
            td.run::<BTreeSet<_>>();
        }
    }

    #[test]
    fn bk_hash() {
        for td in all_test_data() {
            td.run::<HashSet<_>>();
        }
    }

    #[test]
    fn bk_fnv() {
        for td in all_test_data() {
            td.run::<FnvHashSet<_>>();
        }
    }

    #[test]
    fn bk_hashbrown() {
        for td in all_test_data() {
            td.run::<hashbrown::HashSet<_>>();
        }
    }

    #[test]
    fn bk_ordvec() {
        for td in all_test_data() {
            td.run::<Vec<_>>();
        }
    }
}
