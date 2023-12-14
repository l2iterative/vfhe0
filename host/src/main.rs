use methods::METHOD_ELF;
use risc0_zkvm::{ExecutorEnv, ExecutorImpl};
use std::cell::RefCell;
use std::rc::Rc;

mod cycle_trace;
use crate::cycle_trace::CycleTracer;

fn main() {
    let cycle_tracer = Rc::new(RefCell::new(CycleTracer::default()));

    let env = ExecutorEnv::builder()
        .trace_callback(|e| {
            cycle_tracer.borrow_mut().handle_event(e);
            Ok(())
        })
        .build()
        .unwrap();

    let mut exec = ExecutorImpl::from_elf(env, METHOD_ELF).unwrap();
    let _ = exec.run().unwrap();

    cycle_tracer.borrow().print();
}
