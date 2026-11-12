#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeometryUnits {
    Nanometer(f64),
    Micrometer(f64),
    Millimeter(f64),
    Centimeter(f64),
    Meter(f64),
    Kilometer(f64),
}

impl GeometryUnits {
    pub fn to_base_unit(&self) -> Self {
        self.to_meters()
    }

    pub fn to_meters(&self) -> Self {
        match self {
            GeometryUnits::Nanometer(v) => GeometryUnits::Meter(v * 10_f64.powi(-9)),
            GeometryUnits::Micrometer(v) => GeometryUnits::Meter(v * 10_f64.powi(-6)),
            GeometryUnits::Millimeter(v) => GeometryUnits::Meter(v * 10_f64.powi(-3)),
            GeometryUnits::Centimeter(v) => GeometryUnits::Meter(v / 100.0),
            GeometryUnits::Meter(_v) => {
                return *self;
            }
            GeometryUnits::Kilometer(v) => GeometryUnits::Meter(v * 1000.0),
        }
    }

    pub fn to_nanos(&self) -> Self {
        let meters = self.get_value_in_meters();

        GeometryUnits::Nanometer(meters * 10_f64.powi(9))
    }

    pub fn to_micros(&self) -> Self {
        let meters = self.get_value_in_meters();

        GeometryUnits::Micrometer(meters * 10_f64.powi(6))
    }

    pub fn to_millis(&self) -> Self {
        let meters = self.get_value_in_meters();

        GeometryUnits::Millimeter(meters * 10_f64.powi(3))
    }

    pub fn to_centis(&self) -> Self {
        let meters = self.get_value_in_meters();

        GeometryUnits::Centimeter(meters * 100.0)
    }

    pub fn to_kilos(&self) -> Self {
        let meters = self.get_value_in_meters();

        GeometryUnits::Kilometer(meters / 1000.0)
    }

    pub fn get_value_in_meters(&self) -> f64 {
        match self.to_meters() {
            GeometryUnits::Meter(v) => v,
            _ => panic!("get_value_in_meters"),
        }
    }
    pub fn get_value_in_base(&self) -> f64 {
        match self.to_base_unit() {
            GeometryUnits::Meter(v) => v,
            _ => panic!("get_value_in_base")
        }
    }
}
