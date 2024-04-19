use demo_1::demo_1;
use demo_2::demo_2;
use demo_3::demo_3;
use demo_4::demo_4;

mod demo_1;
mod demo_2;
mod demo_3;
mod demo_4;

fn main() {
    demo_1();
    // demo_2();
    demo_3();
    demo_4();
}