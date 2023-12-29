pub mod communication;
pub mod financial;
pub mod result;
pub mod temporal;
pub mod traits;

pub mod prelude {
    pub use crate::{
        communication::{Email, PhoneNumber},
        financial::{iso, Money},
        temporal::{Date, DateTime, DateTimeTZ, Time},
    };
}
