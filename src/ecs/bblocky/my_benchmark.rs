// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use crate::ecs::bblocky::super_any::SuperAny;
// use crate::ecs::bblocky::super_vec::SuperVec;
// use serde::*;
// use rand::random;
//
// #[derive(Default, Clone, Serialize, Deserialize)]
// pub struct BenchStruct{
//     pub num: i32,
//     pub float: f32,
// }
// trait ListLike{
//     fn list_push(&mut self, item: BenchStruct);
//     fn list_get(&self, index: usize) -> Option<&BenchStruct>;
// }
// impl ListLike for Vec<SuperAny>{
//     fn list_push(&mut self, item: BenchStruct) {
//         self.push(SuperAny::new(item));
//     }
//     fn list_get(&self, index: usize) -> Option<&BenchStruct> {
//         return Some(self.get(index).unwrap().get());
//     }
// }
// impl ListLike for SuperVec{
//     fn list_push(&mut self, item: BenchStruct) {
//         self.push(item);
//     }
//     fn list_get(&self, index: usize) -> Option<&BenchStruct> {
//         return self.get(index);
//     }
// }
// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }
// fn do_process<T : ListLike>(mut gogo: T) -> f32{
//     let list_size = 1_000_000;
//     for i in 0..list_size{
//         gogo.list_push(BenchStruct{
//             num: 10,
//             float: random()
//         });
//     }
//     let mut total = 0.0;
//     for i in 0..list_size{
//         total += gogo.list_get(i).unwrap().float;
//     }
//     return total;
//
// }
//
// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
//     c.bench_function("Vec<SuperAny> (Old)", |b| b.iter(||{
//         do_process(vec![])
//     }));
//     c.bench_function("SuperVec (New)", |b| b.iter(||{
//         do_process(SuperVec::new(crate::utils::gett::<BenchStruct>()))
//     }));
// }
//
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
