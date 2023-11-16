use chdb::{option, options::QueryOption, Query};

fn main() {
    let result = Query::new("SELECT number FROM numbers(10)")
        .option(option!("log-level", "trace"))
        .exec()
        .unwrap();
    println!("{}", result.to_string().unwrap());
}
