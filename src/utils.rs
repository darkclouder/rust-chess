use std::{error::Error, fmt};


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


#[derive(Debug)]
pub struct ValueError;


impl Error for ValueError {}


impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueError")
    }
}
