use parser;
use error;

pub trait Pass {

}

pub trait MutPass {

}

impl<T: MutPass> Pass for T {

}