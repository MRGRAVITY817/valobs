pub mod communication;
pub mod financial;
pub mod geography;
pub mod result;
pub mod temporal;
pub mod traits;

pub mod prelude {
    pub use crate::{
        communication::{Email, PhoneNumber},
        financial::Money,
        temporal::{Date, DateTime, DateTimeTZ, Time},
    };
}
