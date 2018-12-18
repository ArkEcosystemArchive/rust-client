use std::borrow::Borrow;
use std::collections::HashMap;

pub fn to_map<I, K, V>(params: I) -> HashMap<String, String>
where
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let mut map = HashMap::new();
    for pair in params {
        let &(ref k, ref v) = pair.borrow();
        map.insert(k.as_ref().to_owned(), v.as_ref().to_owned());
    }

    map
}
