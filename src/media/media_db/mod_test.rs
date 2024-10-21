use super::*;
use crate::feed::route;

#[test]
fn test_encode_decode_account_route() {
    let account_route = Route::Account(account::route::Route::Index);
    let encoded = encode(account_route.clone());
    let decoded = decode(&encoded);
    assert_eq!(decoded, account_route);
}
