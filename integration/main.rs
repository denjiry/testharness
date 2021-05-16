pub mod tests;
use tests::IntegrationTest;


fn setup() {
    println!("Setup")
}

fn teardown() {
    println!("Teardown")
}
fn main() {
    // Setup test environment
    setup();

    // TODO: Run the test
    for t in inventory::iter::<IntegrationTest> {
        (t.test_fn)()
    }
    // Teardown test environment
    teardown();
}
