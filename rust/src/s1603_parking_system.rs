/*
 * @lc app=leetcode.cn id=1603 lang=rust
 *
 * [1603] 设计停车系统
 */

// @lc code=start
struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            big: big,
            medium: medium,
            small: small,
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => {
                if self.big == 0 {
                    return false;
                } else {
                    self.big -= 1
                }
            }
            2 => {
                if self.medium == 0 {
                    return false;
                } else {
                    self.medium -= 1
                }
            }
            3 => {
                if self.small == 0 {
                    return false;
                } else {
                    self.small -= 1
                }
            }
            _ => return false,
        }
        true
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
// @lc code=end

#[cfg(test)]
mod test {
    use super::ParkingSystem;

    #[test]
    fn it_works() {
        let mut park = ParkingSystem::new(1, 1, 0);
        assert_eq!(park.add_car(1), true);
        assert_eq!(park.add_car(2), true);
        assert_eq!(park.add_car(3), false);
    }
}
