use crate::setup::{revert_test_db, prepare_test_db};


#[test]
fn test_user_creation() {
    prepare_test_db().expect("Test DB setup failed");

    // test logic here

    revert_test_db().expect("Test DB revert failed");
}