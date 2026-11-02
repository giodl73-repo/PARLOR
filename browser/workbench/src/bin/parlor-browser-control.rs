use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        println!(
            "{}",
            parlor_browser_workbench::evaluate(&line.expect("UTF-8 request"))
        );
    }
}
