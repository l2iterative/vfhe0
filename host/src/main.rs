use methods::METHOD_ELF;
use risc0_zkvm::{default_executor, ExecutorEnv};
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

    let _ = default_executor().execute(env, METHOD_ELF).unwrap();

    cycle_tracer.borrow().print();
}
