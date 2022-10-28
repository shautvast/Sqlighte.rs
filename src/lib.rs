#![allow(dead_code)]

mod page;
mod database;
mod values;
mod varint;
mod record;
mod builder;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
