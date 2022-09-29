use web_server_lib::http::req_res::internal_responses;

#[test]
fn valid_asset_packaging_img() {
    assert_eq!(internal_responses::get_img().get_code(), 200);
}

#[test]
fn valid_asset_packaging_404() {
    assert_eq!(internal_responses::get_not_found().get_code(), 404);
}
