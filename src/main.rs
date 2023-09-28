#![no_std]
#![no_main]

use num::{abs, pow};

use core::{time::Duration, f32::consts::PI};

use pros::{prelude::*, position::Position, task::sleep};

struct Drivebase {
    left_motor: Motor,
    right_motor: Motor,
    circumference: f32,
    axle_track: f32,
    gear_ratio: f32,
}

impl Drivebase {

    fn move_speed(&mut self, speed: i32) {
        self.left_motor.set_raw_output(-speed as i8);
        self.right_motor.set_raw_output(speed as i8);
    }

    fn move_distance(&mut self, speed: i32, distance: i32) {
        let degrees = distance as f32 / self.circumference * 360.0;
        self.left_motor.set_position_relative(Position::Degrees(degrees as f64), -speed);
        self.right_motor.set_position_relative(Position::Degrees(degrees as f64), speed);
    }

    fn turn_speed(&mut self, speed: i32) {
        self.left_motor.set_raw_output(speed as i8);
        self.right_motor.set_raw_output(speed as i8);
    }

    fn turn_degrees(&mut self, speed: i32, turn: i32) {
        let degrees = (PI * self.axle_track / (360.0 / turn as f32)) / self.circumference * 360.0;
        self.left_motor.set_position_relative(Position::Degrees(degrees as f64), speed);
        self.right_motor.set_position_relative(Position::Degrees(degrees as f64), speed);
    }

    fn stop(&mut self) {
        self.left_motor.brake();
        self.right_motor.brake();
    }

    fn arcade_drive(&mut self, controller: &Controller) -> Result<(), pros::motor::MotorError> {
        let left_speed;
        let right_speed;

        let controller_y = pow(controller.state().joysticks.left.y, 3);
        let controller_x = pow(controller.state().joysticks.right.x, 3) / 2.0;
        
        println!("Controllers: {controller_y}, {controller_x}");
        
        if controller_x >= 0.0 {
            if controller_y >= 0.0 {
                left_speed = f32::max(abs(controller_x), abs(controller_y));
                right_speed = controller_y - controller_x;
            } else {
                left_speed = controller_x + controller_y;
                right_speed = -f32::max(abs(controller_x), abs(controller_y));
            }
        } else {
            if controller_y >= 0.0 {
                left_speed = controller_x + controller_y;
                right_speed = f32::max(abs(controller_x), abs(controller_y));
            } else {
                left_speed = -f32::max(abs(controller_x), abs(controller_y));
                right_speed = abs(controller_x) - abs(controller_y);
            }
        }

        self.left_motor.set_output(-left_speed)?;
        self.right_motor.set_output(right_speed)?;

        Ok(())
    }
}

struct Robot;
impl pros::Robot for Robot {
    fn opcontrol() -> pros::Result {
        let diameter: f32 = 101.7;
        let left_motor = Motor::new(1, BrakeMode::Brake)?;
        let right_motor = Motor::new(2, BrakeMode::Brake)?;
        let controller = Controller::new(ControllerId::Master);

        let mut drivebase = Drivebase {
            left_motor,
            right_motor,
            circumference: PI * diameter,
            axle_track: 305.0,
            gear_ratio: 5.0,
        };
        
        loop {
            drivebase.arcade_drive(&controller)?;
            sleep(Duration::from_millis(20));
        }
    }
}
robot!(Robot);