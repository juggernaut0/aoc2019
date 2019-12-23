use crate::intcode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run1(input: Vec<String>) -> Value {
    let mut comps = Vec::new();
    let out_s = Stream::new_wrapped();
    for i in 0..50 {
        let mut comp = Computer::new(parse_program(&input[0]));
        let in_s = Stream::new_wrapped();
        in_s.borrow_mut().write(i);
        comp.set_input(Some(in_s));
        comp.set_output(Some(Rc::clone(&out_s)));
        comps.push(comp);
    }
    let mut router = Router::new(out_s);
    loop {
        if let &Some((_, y)) = &router.nat {
            return y;
        }
        for current in 0..50 {
            log::debug!("Executing comp {}", current);
            if comps[current].execute() == ComputerState::WaitingOnInput {
                comps[current].input().unwrap().borrow_mut().write(-1);
            }
        }
        router.route(&comps);
    }
}

pub fn run2(input: Vec<String>) -> Value {
    let mut comps = Vec::new();
    let out_s = Stream::new_wrapped();
    for i in 0..50 {
        let mut comp = Computer::new(parse_program(&input[0]));
        let in_s = Stream::new_wrapped();
        in_s.borrow_mut().write(i);
        comp.set_input(Some(in_s));
        comp.set_output(Some(Rc::clone(&out_s)));
        comps.push(comp);
    }
    let mut router = Router::new(out_s);
    let mut last_nat_packet_y = None;
    let mut first = true; // HACK: for some reason the first round is idle
    loop {
        for current in 0..50 {
            log::debug!("Executing comp {}", current);
            if comps[current].execute() == ComputerState::WaitingOnInput {
                comps[current].input().unwrap().borrow_mut().write(-1);
            }
        }
        if router.route(&comps) && !first {
            let nat_packet = router.nat.clone();
            let (x, y) = nat_packet.expect("Expected nat to have a packet when network is idle");
            if last_nat_packet_y.is_some() && last_nat_packet_y.unwrap() == y {
                return y;
            }
            comps[0].input().unwrap().borrow_mut().write_all(&[x, y]);
            last_nat_packet_y = Some(y)
        }
        first = false;
    }
}

struct Router {
    stream: Rc<RefCell<Stream>>,
    nat: Option<(Value, Value)>,
}

impl Router {
    fn new(stream: Rc<RefCell<Stream>>) -> Router {
        Router {
            stream,
            nat: None,
        }
    }

    fn route(&mut self, comps: &[Computer]) -> bool {
        let new_packets = self.stream.borrow_mut().read_all();
        log::debug!("new packets: {}", new_packets.len() / 3);
        if new_packets.is_empty() {
            return true;
        }
        for packet in new_packets.chunks(3) {
            let a = packet[0];
            let x = packet[1];
            let y = packet[2];
            if a == 255 {
                self.nat = Some((x, y));
            } else {
                comps[a as usize].input().unwrap().borrow_mut().write_all(&[x, y]);
            }
        }
        false
    }
}
