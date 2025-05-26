pub mod cli;
pub mod printer;
pub mod sdg;
pub mod upload;

pub type Error = Box<dyn std::error::Error>;

pub trait IntoZpl {
    fn into_zpl(self: Self) -> Result<Vec<u8>, Error>;
}

pub trait IntoSgd {
    fn into_sgd(self: Self) -> Vec<u8>;
}
