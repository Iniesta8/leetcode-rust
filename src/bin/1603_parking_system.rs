struct ParkingSystem {
    capacity: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            capacity: vec![big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let index = car_type - 1;
        if index < 0 || index >= self.capacity.len() as i32 {
            return false;
        }

        if self.capacity[index as usize] > 0 {
            self.capacity[index as usize] -= 1;
            return true;
        }
        false
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parking_system() {
        let mut obj = ParkingSystem::new(1, 1, 0);
        assert_eq!(obj.add_car(1), true);
        assert_eq!(obj.add_car(2), true);
        assert_eq!(obj.add_car(3), false);
        assert_eq!(obj.add_car(1), false);
    }
}
