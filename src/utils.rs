use std::borrow::Borrow;
use std::collections::HashMap;

pub fn merge_params<I, K, V, I2, K2, V2>(left: I, right: I2) -> Vec<(String, String)>
where
    I: IntoIterator,
    I2: IntoIterator,
    I::Item: Borrow<(K, V)>,
    I2::Item: Borrow<(K2, V2)>,
    K: AsRef<str>,
    K2: AsRef<str>,
    V: AsRef<str>,
    V2: AsRef<str>,
{
    let mut merged: Vec<(String, String)> = Vec::new();

    for pair in left {
        let &(ref k, ref v) = pair.borrow();
        merged.push((k.as_ref().to_owned(), v.as_ref().to_owned()));
    }

    for pair in right {
        let &(ref k, ref v) = pair.borrow();
        merged.push((k.as_ref().to_owned(), v.as_ref().to_owned()));
    }

    merged
}

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
