use rust_embed::EmbeddedFile;
use std::error::Error;

use crate::http::assets;
use crate::http::req_res::response::{Kvp, Response};

pub fn get_not_found() -> Response {
    let replacements = vec![
        Kvp {
            key: String::from("msg"),
            value: String::from("NOT FOUND"),
        },
        Kvp {
            key: String::from("description"),
            value: String::from("The resource was not found on the server"),
        },
    ];
    let html: EmbeddedFile = assets::HtmlAssets::get("html/error_page.html").unwrap();
    Response::build(
        404,
        std::str::from_utf8(html.data.as_ref()).unwrap(),
        replacements,
    )
}

pub fn get_error(err: Box<dyn Error>) -> Response {
    let replacements = vec![
        Kvp {
            key: String::from("msg"),
            value: String::from("ERROR EXECUTING REQUEST"),
        },
        Kvp {
            key: String::from("description"),
            value: format!(
                "Encountered {} error while executing the request",
                err.to_string()
            ),
        },
    ];
    let html: EmbeddedFile = assets::HtmlAssets::get("html/error_page.html").unwrap();
    Response::build(
        404,
        std::str::from_utf8(html.data.as_ref()).unwrap(),
        replacements,
    )
}

pub fn get_not_authorized() -> Response {
    let replacements = vec![
        Kvp {
            key: String::from("msg"),
            value: String::from("NOT AUTHORIZED"),
        },
        Kvp {
            key: String::from("description"),
            value: String::from("You are not authorized to access the requested resource"),
        },
    ];
    let html: EmbeddedFile = assets::HtmlAssets::get("html/error_page.html").unwrap();
    Response::build(
        401,
        std::str::from_utf8(html.data.as_ref()).unwrap(),
        replacements,
    )
}

pub fn get_css() -> Response {
    let css: EmbeddedFile = assets::HtmlAssets::get("html/css/index.css").unwrap();
    let mut response = Response::build(
        200,
        std::str::from_utf8(css.data.as_ref()).unwrap(),
        Vec::new(),
    );

    response.add_header("Content-Type", "text/css");
    response
}

pub fn get_img() -> Response {
    let img: EmbeddedFile = assets::HtmlAssets::get("html/img/img.png").unwrap();
    let mut response = Response::build_binary(200, img.data.as_ref());
    response.add_header("Content-Type", "image/png");
    response
}
