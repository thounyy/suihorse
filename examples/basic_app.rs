use std::env;
use suihorse::App;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new()
        .usage("single_app [args]")
        .action(action);

    app.run(args);
}

fn action(args: Vec<String>) {
    println!("Hello, {:?}", args);
}
