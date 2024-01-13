use anyhow::{Result, anyhow};

struct Peripherals {

}

impl Peripherals {
    pub fn init(&mut self) {

    }

    pub fn read(&self) {

    }
}

type FnInit = Box<dyn FnMut(i32) -> Result<()>>;
type FnExec = Box<dyn Fn(i32) -> Result<f64>>;

struct PinOp {
    fn_init: FnInit,
    fn_op: FnExec,
    allowed: Vec<i32>,
}

impl PinOp {
    fn new(fn_init: FnInit, fn_op: FnExec, allowed: Vec<i32>) -> Self {
        return PinOp{
            fn_init,
            fn_op,
            allowed,
        };
    }
}

struct PinInterface<'a> {
    pin_ops: Vec<PinOp>,
    active: Vec<&'a FnExec>,
}

impl<'a> PinInterface<'a> {
    pub fn new(pin_ops: Vec<PinOp>) -> Self {
        let mut active: Vec<&FnExec> = Vec::new();
        for _ in 0..10 {
            active.push(&pin_ops[0].fn_op);
        }
        return Self{
            pin_ops,
            active
        };
    }

    pub fn exec_op(&self, pin_nr: i32) -> Result<f64> {
        let Some(fn_op) = self.active.get(pin_nr as usize) else {
            return Err(anyhow!("Invalid pin number"));
        };

        return fn_op(pin_nr);
    }

    pub fn init_op(&mut self, pin_nr: i32, op_nr: i32) -> Result<()> {
        let Some(pin_op) = self.pin_ops.get_mut(op_nr as usize) else {
            return Err(anyhow!("Invalid op number"));
        };

        if !pin_op.allowed.contains(&pin_nr) {
            return Err(anyhow!("Invalid pin number"));
        }

        (pin_op.fn_init)(pin_nr)?;
        self.active[pin_nr as usize] = &pin_op.fn_op;

        return Ok(());
    }
}

fn disabled(pin_nr: i32) -> Result<f64> {
    return Ok(10.);
}

fn init_digital_read(pin_nr: i32) -> Result<()> {
    return Ok(());
}

fn digital_read(pin_nr: i32) -> Result<f64> {
    return Ok(10.);
}

fn main() {
    let mut per = Peripherals{};

    let fn_init_digital_read = |_: i32| {
        per.init();
        Ok(())
    };

    let fn_digital_read = |_: i32| {
        per.read();
        let ret: f64 = 10.;
        return Ok(ret);
    };

    let interface = PinInterface::new(
        vec![PinOp::new(Box::new(fn_init_digital_read), Box::new(fn_digital_read), vec![1, 2, 3, 10])],
    );
}
