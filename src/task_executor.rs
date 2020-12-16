// use std::{cell::RefCell, future::Future, rc::Rc, time::{Duration, Instant}};
// use std::{
//     collections::VecDeque, pin::Pin, ptr, task::Context, task::Poll, task::RawWaker,
//     task::RawWakerVTable, task::Waker,
// };

// use crate::app::App;

// pub struct FrameFuture {
//     run: bool,
//     start: Instant,
// }

// impl Future for FrameFuture {
//     type Output = Duration;

//     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
//         if self.run {
//             Poll::Ready(self.start.elapsed())
//         } else {
//             self.get_mut().run = true;
//             Poll::Pending
//         }
//     }
// }

// impl FrameFuture {
//     pub fn new() -> FrameFuture {
//         FrameFuture {
//             run: false,
//             start: Instant::now(),
//         }
//     }
// }

// pub struct Task<'t> {
//     pub gen: Pin<Box<dyn Future<Output = ()> + 't>>,
// }

// pub struct Executor<'t> {
//     app: Rc<RefCell<App<'t>>>,
//     // tasks: VecDeque<Task<'t>>,
//     scroll_task: Option<Task<'t>>
// }

// impl<'t> Executor<'t> {

//     pub fn new<'a>(app: &Rc<RefCell<App<'a>>>) -> Executor<'a> {
//         Executor {
//             // tasks: VecDeque::new(),
//             app: Rc::clone(app),
//             scroll_task: None,
//         }
//     }

//     pub fn schedule(&mut self, task: Box< dyn FnOnce(&Rc<RefCell<App<'t>>>) -> Pin<Box<dyn Future<Output = ()>>>>) {

//         let gen = task(&self.app);

//         // self.tasks.push_back(Task { gen });
//     }

//     pub fn scroll_to(&mut self) {
//         let clone = Rc::clone(&self.app);

//         self.scroll_task = Some(Task { gen: Box::pin(async move {

//             })
//         });
//     }

//     pub fn run(&mut self) {
//         let waker = my_waker();
//         let mut context = Context::from_waker(&waker);

//         if let Some(mut task) = self.scroll_task.take() {
//             match task.gen.as_mut().poll(&mut context) {
//                 Poll::Ready(_) => {}
//                 Poll::Pending => {
//                     self.scroll_task = Some(task);
//                 }
//             }
//         }
//     }
//         // let t = self.scroll_task.;

//     //     let mut new_queue = VecDeque::new();

//     //     while let Some(mut task) = self.tasks.pop_front() {
//     //         let waker = my_waker();
//     //         let mut context = Context::from_waker(&waker);

//     //         match task.gen.as_mut().poll(&mut context) {
//     //             Poll::Ready(_) => {}
//     //             Poll::Pending => {
//     //                 new_queue.push_back(task);
//     //             }
//     //         }
//     //     }
//     //     self.tasks = new_queue;
//     // }

//     pub fn pending(&self) -> bool {
//         self.scroll_task.is_some()
//     }

// }

//                             //                 let mut app = clone.borrow_mut();
//                             //                 if app.scroll.1 >= start + 10 {
//                             //                     break;
//                             //                 }
//                             //                 let d = start_time.elapsed().as_secs_f64() / duration;
//                             //                 let l = lerp(start as f64, (start + 10) as f64, d);

//                             //                 app.scroll.1 = l as i64;
//                             //             }
//                             //             FrameFuture::new().await;
//                             //             {
//                             //                 clone.borrow_mut().should_rerender = true;
//                             //             }

// type WakerData = *const ();

// unsafe fn clone(_: WakerData) -> RawWaker {
//     my_raw_waker()
// }
// unsafe fn wake(_: WakerData) {}
// unsafe fn wake_by_ref(_: WakerData) {}
// unsafe fn drop(_: WakerData) {}

// static MY_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

// fn my_raw_waker() -> RawWaker {
//     RawWaker::new(ptr::null(), &MY_VTABLE)
// }

// fn my_waker() -> Waker {
//     unsafe { Waker::from_raw(my_raw_waker()) }
// }
