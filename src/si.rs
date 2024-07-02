use super::units::BaseUnitType;

pub trait TimeUnit: BaseUnitType {}
pub trait LengthUnit: BaseUnitType {}
pub trait MassUnit: BaseUnitType {}
pub trait CurrentUnit: BaseUnitType {}
pub trait TemperatureUnit: BaseUnitType {}
pub trait SubstanceAmountUnit: BaseUnitType {}
pub trait LuminousIntensityUnit: BaseUnitType {}
pub trait AngleUnit: BaseUnitType {}
pub trait SolidAngleUnit: BaseUnitType {}

#[derive(Clone, Copy)]
pub struct Second {}
impl BaseUnitType for Second { const Value: Self = Self{}; }
impl TimeUnit for Second {}

#[derive(Clone, Copy)]
pub struct Meter {}
impl BaseUnitType for Meter { const Value: Self = Self{}; }
impl LengthUnit for Meter {}

#[derive(Clone, Copy)]
pub struct Kg {}
impl BaseUnitType for Kg { const Value: Self = Self{}; }
impl MassUnit for Kg {}

#[derive(Clone, Copy)]
pub struct Ampere {}
impl BaseUnitType for Ampere { const Value: Self = Self{}; }
impl CurrentUnit for Ampere {}

#[derive(Clone, Copy)]
pub struct Kelvin {}
impl BaseUnitType for Kelvin { const Value: Self = Self{}; }
impl TemperatureUnit for Kelvin {}

#[derive(Clone, Copy)]
pub struct Mole {}
impl BaseUnitType for Mole { const Value: Self = Self{}; }
impl SubstanceAmountUnit for Mole {}

#[derive(Clone, Copy)]
pub struct Candela {}
impl BaseUnitType for Candela { const Value: Self = Self{}; }
impl LuminousIntensityUnit for Candela {}

#[derive(Clone, Copy)]
pub struct Radian {}
impl BaseUnitType for Radian { const Value: Self = Self{}; }
impl AngleUnit for Radian {}

#[derive(Clone, Copy)]
pub struct Steradian {}
impl BaseUnitType for Steradian { const Value: Self = Self{}; }
impl SolidAngleUnit for Steradian {}


