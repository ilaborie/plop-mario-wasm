// use std::rc::Rc;
// use std::cell::RefCell;
// use crate::utils::{time, log, request_animation_frame};
// use wasm_bindgen::prelude::*;
//
// const DELTA_TIME: f64 = 1. / 60.;
//
// #[derive(Default)]
// pub struct Timer {
//     last_time: f64,
//     accumulated_time: f64,
// }
//
// impl Timer {
//     pub fn start(&mut self, callback: Box<dyn FnMut() -> bool>) {
//         let f = Rc::new(RefCell::new(None));
//         let g = f.clone();
//
//
//         *g.borrow_mut() = Some(Closure::wrap(Box::new( || {
//             let time = time();
//             self.accumulated_time += (time - self.last_time) / 1000.;
//             while self.accumulated_time > DELTA_TIME {
//                 let cont = callback();
//
//                 if !cont {
//                     log("Finished!");
//                     let _ = f.borrow_mut().take();
//                     return;
//                 }
//
//                 self.accumulated_time -= DELTA_TIME;
//             }
//             self.last_time = time;
//
//             request_animation_frame(f.borrow().as_ref().unwrap());
//         }) as Box<dyn FnMut()>));
//
//         request_animation_frame(g.borrow().as_ref().unwrap());
//     }
// }
//
//
