pub struct AethelgardV2 {
    weights: Vec<f32>,
    buffer: Vec<f32>,
    head: usize,
    order: usize,
    learning_rate: f32,
    leakage: f32,
    eps: f32,
    nl_coeff: f32,
}

impl AethelgardV2 {
    pub fn new(order: usize, learning_rate: f32) -> Self {
        Self {
            weights: vec![0.0; order],
            buffer: vec![0.0; order],
            head: 0,
            order,
            learning_rate,
            leakage: 0.999999,
            eps: 1e-6,
            nl_coeff: 0.01,
        }
    }

    pub fn process_sample(&mut self, dirty_sample: f32, clean_ref: f32) -> 
(f32, f32) {
        self.buffer[self.head] = dirty_sample;
        let mut y_linear = 0.0;
        for i in 0..self.order {
            let idx = (self.head + i) % self.order;
            y_linear += self.weights[i] * self.buffer[idx];
        }
        let y_nonlinear = self.nl_coeff * y_linear.powi(3);
        let estimate = y_linear + y_nonlinear;
        let error = clean_ref - estimate;
        let mut norm = 0.0;
        for i in 0..self.order {
            norm += self.buffer[i] * self.buffer[i];
        }
        norm += self.eps;
        let step = (self.learning_rate * error) / norm;
        for i in 0..self.order {
            let idx = (self.head + i) % self.order;
            self.weights[i] = (self.leakage * self.weights[i]) + (step * 
self.buffer[idx]);
        }
        if self.head == 0 {
            self.head = self.order - 1;
        } else {
            self.head -= 1;
        }
        (estimate, error)
    }
}

#[no_mangle]
pub extern "C" fn aethelgard_create(order: usize, lr: f32) -> *mut 
AethelgardV2 {
    Box::into_raw(Box::new(AethelgardV2::new(order, lr)))
}

#[no_mangle]
pub extern "C" fn aethelgard_process(ptr: *mut AethelgardV2, dirty: f32, 
clean: f32, out_est: *mut f32, out_err: *mut f32) {
    let core = unsafe { &mut *ptr };
    let (est, err) = core.process_sample(dirty, clean);
    unsafe {
        *out_est = est;
        *out_err = err;
    }
}

#[no_mangle]
pub extern "C" fn aethelgard_destroy(ptr: *mut AethelgardV2) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

