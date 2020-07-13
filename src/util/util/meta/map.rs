//! Map building macro.

/// Concisely construct a hashmap.
#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr), *) => {
        {
            let mut map = std::collections::BTreeMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}
