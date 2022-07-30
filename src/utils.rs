// Source: https://stackoverflow.com/a/72427086

pub trait DiscreetUnwrap<T, E> {
    fn duwrp(self) -> T;
}

impl<T, E> DiscreetUnwrap<T, E> for Result<T, E> {
    fn duwrp(self) -> T  {
        match self {
            Ok(r) => r,
            Err(_) => {
                panic!("duwrp() failed.")
            },
        }
    }
}
