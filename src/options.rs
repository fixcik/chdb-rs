use enum2str::EnumStr;

#[derive(Debug, EnumStr)]
pub enum QueryOption {
    #[enum2str("--{}")]
    Flag(&'static str),
    #[enum2str("--{}={}")]
    Option(&'static str, &'static str),
}

#[macro_export]
macro_rules! flag {
    ($flag_name:expr) => {
        chdb::options::QueryOption::Flag($flag_name)
    };
}

#[macro_export]
macro_rules! option {
    ($option_name: expr, $option_value: expr) => {
        chdb::options::QueryOption::Option($option_name, $option_value)
    };
}
