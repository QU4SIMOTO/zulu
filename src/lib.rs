use anyhow::Result;

pub mod cli;
pub mod printer;
pub mod sdg;
pub mod upload;

pub trait AsZpl {
    fn as_zpl(self: &Self) -> Result<Vec<u8>>;
}

pub trait AsSgd {
    fn as_sgd(self: &Self) -> Vec<u8>;
}
