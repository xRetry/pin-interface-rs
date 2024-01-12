use anyhow::{Result, anyhow};

struct PinOp {
    fn_init: fn(i32) -> Result<()>,
    fn_op: fn(i32) -> Result<f64>,
    allowed: Vec<i32>,
}

impl PinOp {
    fn new(fn_init: fn(i32) -> Result<()>, fn_op: fn(i32) -> Result<f64>, allowed: Vec<i32>) -> Self {
        return PinOp{
            fn_init,
            fn_op,
            allowed,
        };
    }
}

struct PinInterface {
    pin_ops: Vec<PinOp>,
    active: Vec<fn(i32) -> Result<f64>>,
}

impl PinInterface {
    pub fn new(pin_ops: Vec<PinOp>) -> Self {
        let active = vec![disabled; 10];
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

    pub fn init_op(&self, pin_nr: i32, op_nr: i32) -> Result<()> {
        let Some(pin_op) = self.pin_ops.get(op_nr as usize) else {
            return Err(anyhow!("Invalid op number"));
        };

        if !pin_op.allowed.contains(&pin_nr) {
            return Err(anyhow!("Invalid pin number"));
        }

        (pin_op.fn_init)(pin_nr)?;
        self.active[pin_nr as usize] = pin_op.fn_op;

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
    let interface = PinInterface::new(
        vec![PinOp::new(init_digital_read, digital_read, vec![1, 2, 3, 10])],
    );
}
