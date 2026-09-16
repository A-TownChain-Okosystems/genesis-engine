//! Deterministic real-time weather model for Genesis Engine.
//!
//! This module models weather as simulation/presentation state. It does not
//! access external weather services and therefore remains deterministic and
//! suitable for replay/network synchronization.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherCell {
    pub temperature_c: f32,
    pub humidity: f32,
    pub pressure_hpa: f32,
    pub wind: [f32; 2],
    pub precipitation: f32,
    pub cloud_cover: f32,
    pub visibility_m: f32,
}

impl Default for WeatherCell {
    fn default() -> Self {
        Self {
            temperature_c: 15.0,
            humidity: 0.5,
            pressure_hpa: 1013.25,
            wind: [0.0, 0.0],
            precipitation: 0.0,
            cloud_cover: 0.0,
            visibility_m: 50_000.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherConfig {
    pub width: usize,
    pub depth: usize,
    pub cell_size_m: f32,
    pub advection_rate: f32,
    pub diffusion_rate: f32,
    pub precipitation_rate: f32,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self { width: 64, depth: 64, cell_size_m: 500.0, advection_rate: 1.0, diffusion_rate: 0.08, precipitation_rate: 0.15 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherError {
    InvalidDimensions,
    InvalidCellSize,
    InvalidRate,
    InvalidState,
}

fn finite(v: f32) -> bool { v.is_finite() }
fn clamp01(v: f32) -> f32 { v.clamp(0.0, 1.0) }

pub struct WeatherField {
    config: WeatherConfig,
    cells: Vec<WeatherCell>,
}

impl WeatherField {
    pub fn new(config: WeatherConfig) -> Result<Self, WeatherError> {
        if config.width == 0 || config.depth == 0 { return Err(WeatherError::InvalidDimensions); }
        if !finite(config.cell_size_m) || config.cell_size_m <= 0.0 { return Err(WeatherError::InvalidCellSize); }
        if [config.advection_rate, config.diffusion_rate, config.precipitation_rate].iter().any(|v| !finite(*v) || *v < 0.0) { return Err(WeatherError::InvalidRate); }
        Ok(Self { cells: vec![WeatherCell::default(); config.width * config.depth], config })
    }

    pub fn config(&self) -> WeatherConfig { self.config }
    pub fn dimensions(&self) -> (usize, usize) { (self.config.width, self.config.depth) }
    pub fn cells(&self) -> &[WeatherCell] { &self.cells }

    fn index(&self, x: usize, z: usize) -> Option<usize> {
        (x < self.config.width && z < self.config.depth).then_some(z * self.config.width + x)
    }

    pub fn get(&self, x: usize, z: usize) -> Option<WeatherCell> { self.index(x, z).map(|i| self.cells[i]) }

    pub fn set(&mut self, x: usize, z: usize, mut cell: WeatherCell) -> Result<(), WeatherError> {
        if !cell.temperature_c.is_finite() || !cell.pressure_hpa.is_finite() || !cell.wind.iter().all(|v| v.is_finite()) || !cell.visibility_m.is_finite() || cell.visibility_m < 0.0 { return Err(WeatherError::InvalidState); }
        cell.humidity = clamp01(cell.humidity);
        cell.precipitation = clamp01(cell.precipitation);
        cell.cloud_cover = clamp01(cell.cloud_cover);
        let i = self.index(x, z).ok_or(WeatherError::InvalidDimensions)?;
        self.cells[i] = cell;
        Ok(())
    }

    /// Deterministic semi-Lagrangian-style weather update. The current
    /// implementation transports wind/precipitation and diffuses scalar
    /// fields; it is intentionally a stable foundation rather than a claim
    /// of meteorological prediction accuracy.
    pub fn step(&mut self, delta_seconds: f32) -> Result<(), WeatherError> {
        if !delta_seconds.is_finite() || delta_seconds < 0.0 { return Err(WeatherError::InvalidState); }
        if delta_seconds == 0.0 { return Ok(()); }
        let old = self.cells.clone();
        let w = self.config.width;
        let d = self.config.depth;
        let blend = (self.config.diffusion_rate * delta_seconds).clamp(0.0, 1.0);
        for z in 0..d {
            for x in 0..w {
                let i = z * w + x;
                let mut sum = old[i];
                let mut count = 1.0;
                for (nx, nz) in [(x.wrapping_sub(1), z), (x + 1, z), (x, z.wrapping_sub(1)), (x, z + 1)] {
                    if let Some(n) = self.index(nx, nz) {
                        let c = old[n];
                        sum.temperature_c += c.temperature_c; sum.humidity += c.humidity; sum.pressure_hpa += c.pressure_hpa;
                        sum.wind[0] += c.wind[0]; sum.wind[1] += c.wind[1]; sum.precipitation += c.precipitation; sum.cloud_cover += c.cloud_cover; sum.visibility_m += c.visibility_m;
                        count += 1.0;
                    }
                }
                let avg = WeatherCell { temperature_c: sum.temperature_c / count, humidity: sum.humidity / count, pressure_hpa: sum.pressure_hpa / count, wind: [sum.wind[0] / count, sum.wind[1] / count], precipitation: sum.precipitation / count, cloud_cover: sum.cloud_cover / count, visibility_m: sum.visibility_m / count };
                let p = &old[i];
                let mut next = WeatherCell {
                    temperature_c: p.temperature_c + (avg.temperature_c - p.temperature_c) * blend,
                    humidity: p.humidity + (avg.humidity - p.humidity) * blend,
                    pressure_hpa: p.pressure_hpa + (avg.pressure_hpa - p.pressure_hpa) * blend,
                    wind: [p.wind[0] + (avg.wind[0] - p.wind[0]) * blend, p.wind[1] + (avg.wind[1] - p.wind[1]) * blend],
                    precipitation: p.precipitation + (avg.precipitation - p.precipitation) * blend,
                    cloud_cover: p.cloud_cover + (avg.cloud_cover - p.cloud_cover) * blend,
                    visibility_m: p.visibility_m + (avg.visibility_m - p.visibility_m) * blend,
                };
                next.precipitation = clamp01(next.precipitation + self.config.precipitation_rate * delta_seconds * 0.001 * next.humidity.max(0.0));
                next.cloud_cover = clamp01(next.cloud_cover + next.humidity * delta_seconds * 0.0005 - next.precipitation * delta_seconds * 0.0002);
                self.cells[i] = next;
            }
        }
        Ok(())
    }

    pub fn weather_class(&self, x: usize, z: usize) -> Option<WeatherClass> {
        self.get(x, z).map(|c| {
            if c.precipitation > 0.7 { WeatherClass::Storm }
            else if c.precipitation > 0.2 { WeatherClass::Rain }
            else if c.cloud_cover > 0.75 { WeatherClass::Overcast }
            else if c.cloud_cover > 0.35 { WeatherClass::Cloudy }
            else if c.temperature_c < 0.0 && c.humidity > 0.6 { WeatherClass::Snow }
            else { WeatherClass::Clear }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherClass { Clear, Cloudy, Overcast, Rain, Snow, Storm }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_is_deterministic() {
        let cfg = WeatherConfig::default();
        let mut a = WeatherField::new(cfg).unwrap();
        let mut b = WeatherField::new(cfg).unwrap();
        a.set(10, 10, WeatherCell { humidity: 0.9, cloud_cover: 0.8, ..Default::default() }).unwrap();
        b.set(10, 10, WeatherCell { humidity: 0.9, cloud_cover: 0.8, ..Default::default() }).unwrap();
        a.step(1.0).unwrap(); b.step(1.0).unwrap();
        assert_eq!(a.cells(), b.cells());
    }

    #[test]
    fn state_is_clamped() {
        let mut field = WeatherField::new(WeatherConfig::default()).unwrap();
        field.set(0, 0, WeatherCell { humidity: 2.0, precipitation: -1.0, cloud_cover: 4.0, ..Default::default() }).unwrap();
        let c = field.get(0, 0).unwrap();
        assert_eq!(c.humidity, 1.0); assert_eq!(c.precipitation, 0.0); assert_eq!(c.cloud_cover, 1.0);
    }
}
