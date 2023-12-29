pub mod communication;
pub mod financial;
pub mod temporal;

pub mod prelude {
    pub use crate::{
        communication::{Email, EmailError, PhoneNumber},
        financial::{iso, Money},
        temporal::{Date, DateTime, DateTimeTZ, Time},
    };
}
