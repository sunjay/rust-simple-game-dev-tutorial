mod window;

use window::Window;

fn main() {
    let win = Window::new("Hello, window!");
    println!("{:?}", win);
}
