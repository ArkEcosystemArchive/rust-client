pub mod api;

#[cfg(test)]
mod tests {
    use super:: *;

    #[test]
    fn proof_of_concept() {
        let client = api::Client::new();
        let anything = client.anything.all()
            .expect("Some failed");
        println!("{}", anything)
    }
}
