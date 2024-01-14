use anyhow::{Result, anyhow};

struct Gpio0 {

}

struct Gpio1 {

}

struct Peripherals {
    gpio0: Gpio0,
    gpio1: Gpio1,
}

impl Peripherals {
    pub fn init(&mut self) {

    }

    pub fn read(&self) {

    }
}

type FnInit<T> = Box<dyn FnMut(i32, &T) -> Result<()>>;
type FnExec<T> = Box<dyn Fn(i32, &T) -> Result<f64>>;

struct PinOp<T> {
    fn_init: FnInit<T>,
    fn_op: FnExec<T>,
    allowed: Vec<i32>,
}

impl<T> PinOp<T> {
    fn new(fn_init: FnInit<T>, fn_op: FnExec<T>, allowed: Vec<i32>) -> Self {
        return PinOp{
            fn_init,
            fn_op,
            allowed,
        };
    }
}

struct PinInterface<T> {
    pin_ops: Vec<PinOp<T>>,
    active: Vec<i32>,
    state: T,
}

impl<T> PinInterface<T> {
    pub fn new(pin_ops: Vec<PinOp<T>>, state: T) -> Self {
        let mut interface = Self{
            pin_ops,
            active: Vec::new(),
            state,
        };

        for _ in 0..10 {
            interface.active.push(0);
        }
        return interface;
    }

    pub fn exec_op(&self, pin_nr: i32) -> Result<f64> {
        let Some(idx_fn_op) = self.active.get(pin_nr as usize) else {
            return Err(anyhow!("Invalid pin number"));
        };

        let Some(fn_op) = self.pin_ops.get(*idx_fn_op as usize) else {
            return Err(anyhow!("Invalid pin number"));
        };

        return (*fn_op.fn_op)(pin_nr, &self.state);
    }

    pub fn init_op(&mut self, pin_nr: i32, op_nr: i32) -> Result<()> {
        let Some(pin_op) = self.pin_ops.get_mut(op_nr as usize) else {
            return Err(anyhow!("Invalid op number"));
        };

        if !pin_op.allowed.contains(&pin_nr) {
            return Err(anyhow!("Invalid pin number"));
        }

        (pin_op.fn_init)(pin_nr, &self.state)?;
        let Some(idx_fn_op) = self.active.get_mut(pin_nr as usize) else {
            return Err(anyhow!("Invalid pin number"));
        };

        *idx_fn_op = op_nr;

        return Ok(());
    }
}

fn disabled(pin_nr: i32, per: &Peripherals) -> Result<f64> {
    return Ok(10.);
}

fn init_digital_read(pin_nr: i32, per: &Peripherals) -> Result<()> {
    return Ok(());
}

fn digital_read(pin_nr: i32, per: &Peripherals) -> Result<f64> {
    return Ok(10.);
}

fn main() {
    let per = Peripherals{ gpio0: Gpio0{}, gpio1: Gpio1{} };

    let interface = PinInterface::new(
        vec![PinOp::new(Box::new(init_digital_read), Box::new(digital_read), vec![1, 2, 3, 10])],
        per,
    );
}
