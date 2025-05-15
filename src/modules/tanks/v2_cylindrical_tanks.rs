use std::f64::consts::PI;

/// A struct to represent a measurement sample
#[derive(Debug, Clone, Copy)]
struct Sample {
    diameter_meters: f64,
    height_meters: f64,
    volume_liters: f64,
}

/// Calculate the volume of a hollow cylindrical tank
/// 
/// Parameters:
/// - outer_diameter: The outer diameter of the tank in meters
/// - outer_height: The outer height of the tank in meters
/// - wall_thickness: The thickness of the cylindrical wall in meters
/// - cap_thickness: The thickness of the end caps in meters
fn calculate_hollow_cylinder_volume(
    outer_diameter: f64,
    outer_height: f64,
    wall_thickness: f64,
    cap_thickness: f64,
) -> f64 {
    let outer_radius = outer_diameter / 2.0;
    let inner_radius = outer_radius - wall_thickness;
    
    // Check if the inner dimensions are valid
    if inner_radius <= 0.0 || outer_height <= 2.0 * cap_thickness {
        // If walls are too thick, there's no hollow part
        return PI * outer_radius * outer_radius * outer_height * 1000.0;
    }
    
    // Calculate outer volume
    let outer_volume = PI * outer_radius * outer_radius * outer_height;
    
    // Calculate hollow volume (excluding caps)
    let inner_height = outer_height - (2.0 * cap_thickness);
    let inner_volume = PI * inner_radius * inner_radius * inner_height;
    
    // Final volume in liters (convert from m³ to liters)
    (outer_volume - inner_volume) * 1000.0
}

/// Calculate the volume of a cylindrical tank with a cylindrical hollow core
/// 
/// Parameters:
/// - outer_diameter: The outer diameter of the tank in meters
/// - outer_height: The outer height of the tank in meters
/// - inner_diameter: The diameter of the hollow core in meters
/// - inner_height: The height of the hollow core in meters
fn calculate_cylinder_with_core(
    outer_diameter: f64,
    outer_height: f64,
    inner_diameter: f64,
    inner_height: f64,
) -> f64 {
    let outer_radius = outer_diameter / 2.0;
    let inner_radius = inner_diameter / 2.0;
    
    // Check if the inner dimensions are valid
    if inner_radius <= 0.0 || inner_height <= 0.0 {
        return PI * outer_radius * outer_radius * outer_height * 1000.0;
    }
    
    // Calculate outer volume
    let outer_volume = PI * outer_radius * outer_radius * outer_height;
    
    // Calculate inner volume
    let inner_volume = PI * inner_radius * inner_radius * inner_height;
    
    // Final volume in liters (convert from m³ to liters)
    (outer_volume - inner_volume) * 1000.0
}

/// Calculate the errors between predicted and actual volumes for all samples
fn calculate_errors(
    samples: &[Sample],
    calculate_volume: impl Fn(f64, f64) -> f64,
) -> Vec<f64> {
    samples
        .iter()
        .map(|sample| {
            let predicted_volume = calculate_volume(sample.diameter_meters, sample.height_meters);
            let error = ((predicted_volume - sample.volume_liters) / sample.volume_liters).abs() * 100.0;
            error
        })
        .collect()
}

/// Find the best parameters for the hollow cylinder model
fn find_best_hollow_cylinder_params(samples: &[Sample]) -> (f64, f64, f64) {
    let mut best_wall_thickness = 0.0;
    let mut best_cap_thickness = 0.0;
    let mut min_max_error = f64::INFINITY;
    
    // Strategy 1: Search with absolute values
    for wall_thickness in (1..=500).map(|i| i as f64 * 0.001) {
        for cap_thickness in (1..=500).map(|i| i as f64 * 0.001) {
            let errors = samples
                .iter()
                .map(|sample| {
                    let predicted_volume = calculate_hollow_cylinder_volume(
                        sample.diameter_meters,
                        sample.height_meters,
                        wall_thickness,
                        cap_thickness,
                    );
                    let error = ((predicted_volume - sample.volume_liters) / sample.volume_liters).abs() * 100.0;
                    error
                })
                .collect::<Vec<_>>();
            
            let max_error = errors.iter().cloned().fold(0.0, f64::max);
            
            if max_error < min_max_error {
                min_max_error = max_error;
                best_wall_thickness = wall_thickness;
                best_cap_thickness = cap_thickness;
            }
        }
    }
    
    // If error is still high, try Strategy 2: Relative to diameter/height
    if min_max_error > 1.0 {
        for wall_thickness_percent in (1..=70).map(|i| i as f64 * 0.01) {
            for cap_thickness_percent in (1..=70).map(|i| i as f64 * 0.01) {
                let errors = samples
                    .iter()
                    .map(|sample| {
                        let wall_thickness = sample.diameter_meters * wall_thickness_percent;
                        let cap_thickness = sample.height_meters * cap_thickness_percent;
                        
                        let predicted_volume = calculate_hollow_cylinder_volume(
                            sample.diameter_meters,
                            sample.height_meters,
                            wall_thickness,
                            cap_thickness,
                        );
                        let error = ((predicted_volume - sample.volume_liters) / sample.volume_liters).abs() * 100.0;
                        error
                    })
                    .collect::<Vec<_>>();
                
                let max_error = errors.iter().cloned().fold(0.0, f64::max);
                
                if max_error < min_max_error {
                    min_max_error = max_error;
                    // Store percentages instead of absolute values
                    best_wall_thickness = -wall_thickness_percent;  // Negative indicates percentage
                    best_cap_thickness = -cap_thickness_percent;    // Negative indicates percentage
                }
            }
        }
    }
    
    (best_wall_thickness, best_cap_thickness, min_max_error)
}

/// Find the best parameters for the cylinder with core model
fn find_best_core_params(samples: &[Sample]) -> (f64, f64, f64) {
    let mut best_inner_diameter = 0.0;
    let mut best_inner_height = 0.0;
    let mut min_max_error = f64::INFINITY;
    
    // Strategy 1: Search with absolute values
    for inner_diameter in (1..=500).map(|i| i as f64 * 0.001) {
        for inner_height_factor in (1..=500).map(|i| i as f64 * 0.001) {
            let errors = samples
                .iter()
                .map(|sample| {
                    // Inner height is proportional to outer height
                    let inner_height = sample.height_meters * inner_height_factor;
                    
                    let predicted_volume = calculate_cylinder_with_core(
                        sample.diameter_meters,
                        sample.height_meters,
                        inner_diameter,
                        inner_height,
                    );
                    let error = ((predicted_volume - sample.volume_liters) / sample.volume_liters).abs() * 100.0;
                    error
                })
                .collect::<Vec<_>>();
            
            let max_error = errors.iter().cloned().fold(0.0, f64::max);
            
            if max_error < min_max_error {
                min_max_error = max_error;
                best_inner_diameter = inner_diameter;
                best_inner_height = inner_height_factor;  // Store as factor
            }
        }
    }
    
    // If error is still high, try Strategy 2: Relative to outer diameter
    if min_max_error > 1.0 {
        for inner_diameter_percent in (1..=70).map(|i| i as f64 * 0.01) {
            for inner_height_factor in (1..=99).map(|i| i as f64 * 0.01) {
                let errors = samples
                    .iter()
                    .map(|sample| {
                        let inner_diameter = sample.diameter_meters * inner_diameter_percent;
                        let inner_height = sample.height_meters * inner_height_factor;
                        
                        let predicted_volume = calculate_cylinder_with_core(
                            sample.diameter_meters,
                            sample.height_meters,
                            inner_diameter,
                            inner_height,
                        );
                        let error = ((predicted_volume - sample.volume_liters) / sample.volume_liters).abs() * 100.0;
                        error
                    })
                    .collect::<Vec<_>>();
                
                let max_error = errors.iter().cloned().fold(0.0, f64::max);
                
                if max_error < min_max_error {
                    min_max_error = max_error;
                    // Store percentages/factors
                    best_inner_diameter = -inner_diameter_percent;  // Negative indicates percentage
                    best_inner_height = inner_height_factor;        // Always a factor
                }
            }
        }
    }
    
    (best_inner_diameter, best_inner_height, min_max_error)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_tank_parameters() {
        let samples = vec![
            Sample { diameter_meters: 1.0, height_meters: 0.5, volume_liters: 217.2935 },
            Sample { diameter_meters: 1.0, height_meters: 1.0, volume_liters: 543.2337 },
            Sample { diameter_meters: 1.0, height_meters: 2.0, volume_liters: 1195.1142 },
            Sample { diameter_meters: 1.0, height_meters: 5.0, volume_liters: 3150.7556 },
            Sample { diameter_meters: 1.0, height_meters: 10.0, volume_liters: 6410.1581 },
            Sample { diameter_meters: 5.0, height_meters: 3.0, volume_liters: 35310.1939 },
        ];
        
        // Test hollow cylinder model
        println!("Finding best parameters for hollow cylinder model...");
        let (wall_thickness, cap_thickness, hollow_error) = find_best_hollow_cylinder_params(&samples);
        
        if wall_thickness < 0.0 && cap_thickness < 0.0 {
            println!(
                "Best hollow cylinder model: Wall thickness = {:.2}% of diameter, Cap thickness = {:.2}% of height",
                -wall_thickness * 100.0, 
                -cap_thickness * 100.0
            );
        } else {
            println!(
                "Best hollow cylinder model: Wall thickness = {:.4} m, Cap thickness = {:.4} m",
                wall_thickness, 
                cap_thickness
            );
        }
        println!("Maximum error: {:.4}%", hollow_error);
        
        // Verify the results
        println!("\nHollow cylinder model verification:");
        for sample in &samples {
            let calculated_volume = if wall_thickness < 0.0 && cap_thickness < 0.0 {
                // Using percentages
                let actual_wall = -wall_thickness * sample.diameter_meters;
                let actual_cap = -cap_thickness * sample.height_meters;
                calculate_hollow_cylinder_volume(
                    sample.diameter_meters,
                    sample.height_meters,
                    actual_wall,
                    actual_cap,
                )
            } else {
                // Using absolute values
                calculate_hollow_cylinder_volume(
                    sample.diameter_meters,
                    sample.height_meters,
                    wall_thickness,
                    cap_thickness,
                )
            };
            
            let error = ((calculated_volume - sample.volume_liters) / sample.volume_liters).abs() * 100.0;
            
            println!(
                "Diameter: {:.1} m, Height: {:.1} m, Actual: {:.4} L, Calculated: {:.4} L, Error: {:.4}%",
                sample.diameter_meters, 
                sample.height_meters, 
                sample.volume_liters, 
                calculated_volume, 
                error
            );
        }
        
        // Test cylinder with core model
        println!("\nFinding best parameters for cylinder with core model...");
        let (inner_diameter, inner_height_factor, core_error) = find_best_core_params(&samples);
        
        if inner_diameter < 0.0 {
            println!(
                "Best cylinder with core model: Inner diameter = {:.2}% of outer diameter, Inner height factor = {:.2}",
                -inner_diameter * 100.0, 
                inner_height_factor
            );
        } else {
            println!(
                "Best cylinder with core model: Inner diameter = {:.4} m, Inner height factor = {:.4}",
                inner_diameter, 
                inner_height_factor
            );
        }
        println!("Maximum error: {:.4}%", core_error);
        
        // Verify the results
        println!("\nCylinder with core model verification:");
        for sample in &samples {
            let inner_diameter_value = if inner_diameter < 0.0 {
                -inner_diameter * sample.diameter_meters
            } else {
                inner_diameter
            };
            
            let inner_height = inner_height_factor * sample.height_meters;
            
            let calculated_volume = calculate_cylinder_with_core(
                sample.diameter_meters,
                sample.height_meters,
                inner_diameter_value,
                inner_height,
            );
            
            let error = ((calculated_volume - sample.volume_liters) / sample.volume_liters).abs() * 100.0;
            
            println!(
                "Diameter: {:.1} m, Height: {:.1} m, Actual: {:.4} L, Calculated: {:.4} L, Error: {:.4}%",
                sample.diameter_meters, 
                sample.height_meters, 
                sample.volume_liters, 
                calculated_volume, 
                error
            );
        }
        
        // Determine which model is better
        let best_model = if core_error < hollow_error { "Cylinder with core" } else { "Hollow cylinder" };
        let min_error = f64::min(hollow_error, core_error);
        
        println!("\nBest model: {} (Max error: {:.4}%)", best_model, min_error);
        
        // Assert that the error is below threshold
        assert!(
            min_error < 1.0,
            "Error too high: {:.2}%. Need better model or parameters.",
            min_error
        );
    }
}