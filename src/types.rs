pub use uom::si::thermodynamic_temperature::*;
pub use uom::si::length::*;
pub use uom::si::pressure::*;
pub use uom::si::*;

pub type Altitude<T> = Length<SI<T>, T>;

pub type Temperature<T> = ThermodynamicTemperature<SI<T>, T>;

pub type AtmosphericPressure<T> = Pressure<SI<T>, T>;
