
fn call_panic() {
    panic!("Panic!");
    // error: process didn't exit successfully: `target\debug\err1.exe` (exit code: 101)
}

fn case_ref_index() {
    let v = vec![1, 2, 3];
    v[99];
    // error: process didn't exit successfully: `target\debug\err1.exe` (exit code: 101)
}

fn main() {
    // call_panic();
    case_ref_index();
}
