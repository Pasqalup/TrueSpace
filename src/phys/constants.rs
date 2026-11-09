use std::f64::consts::PI;


#[derive(Debug, Clone, Copy)]
pub struct AcousticEnvironment {
    pub frequency: f64,
    pub speed_of_sound: f64,
    pub density: f64,
    pub omega: f64,
    pub wavenumber: f64,
}
impl AcousticEnvironment {
    pub fn new(frequency: f64, speed_of_sound: f64, density: f64) -> Self {
        let omega = 2.0 * PI * frequency;
        let wavenumber = omega / speed_of_sound;
        AcousticEnvironment {
            frequency,
            speed_of_sound,
            density,
            omega,
            wavenumber,
        }
    }
    pub fn new_air(frequency: f64) -> Self {
        let speed_of_sound = 343.0; // m/s
        let density = 1.225; // kg/m^3
        AcousticEnvironment::new(frequency, speed_of_sound, density)
    }
    pub fn characteristic_impedance(&self) -> f64 {
        self.density * self.speed_of_sound
    }

}