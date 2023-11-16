use chdb::{flag, option, Query};

fn main() {
    let result = Query::new("SELECT number FROM numbers(10)")
        .option(option!("format", "TSVWithNames"))
        .option(flag!("verbose"))
        .exec()
        .unwrap();

    println!("Elapsed: {}", result.elapsed);
    println!("Rows: {}", result.rows_read);
    println!("Bytes: {}", result.bytes_read);
    println!("Result:\n{}", result.to_string().unwrap());
}
