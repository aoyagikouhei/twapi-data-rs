pub mod v1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let value: v1::error::TwErrorItemWithStatusCode = v1::error::TwError::UserNotFound.into();
        println!("{:?}", serde_json::to_string(&value.make_errors()));
    }
}
