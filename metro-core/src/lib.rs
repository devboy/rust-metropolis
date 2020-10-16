#![no_std]

pub mod musical;
pub mod sequencer;
pub mod analog;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
