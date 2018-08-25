use std::borrow::Borrow;

pub fn merge_params<I, K, V, I2, K2, V2>(left: I, right: I2) -> Vec<(String, String)>
    where I: IntoIterator, I2: IntoIterator,
                 I::Item: Borrow<(K, V)>, I2::Item: Borrow<(K2, V2)>,
                 K: AsRef<str>, K2: AsRef<str>,
                 V: AsRef<str>, V2: AsRef<str>
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
