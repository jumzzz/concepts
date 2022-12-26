use std::cell::{Cell, RefCell};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Robot {
    hardware_error_count: Cell<u32>,
    event_logs: RefCell<Vec<String>>,
}

impl Robot {

    pub fn new() -> Robot {
        Robot {
            hardware_error_count: Cell::new(0),
            event_logs: RefCell::new(vec![]),
        }
    }

    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn add_event_log(&self, event: String) {
        let mut events = self.event_logs.borrow_mut();
        events.push(event);
    }
}


fn main() {

    let robot = Robot::new();

    println!("{:?}", robot);
    robot.add_event_log(String::from("andy"));
    
    println!("{:?}", robot);
    robot.add_event_log(String::from("guts"));

    println!("{:?}", robot);
    robot.add_event_log(String::from("machi"));

    println!("{:?}", robot);
    robot.add_event_log(String::from("bebby"));

    println!("{:?}", robot);
}
