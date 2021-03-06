use super::{rocket, TemplateContext};

use rocket::local::blocking::Client;
use rocket::http::Method::*;
use rocket::http::Status;
use rocket_contrib::templates::Template;

macro_rules! dispatch {
    ($method:expr, $path:expr, |$client:ident, $response:ident| $body:expr) => ({
        let $client = Client::new(rocket()).unwrap();
        let $response = $client.req($method, $path).dispatch();
        $body
    })
}

#[test]
fn test_root() {
    // Check that the redirect works.
    for method in &[Get, Head] {
        dispatch!(*method, "/", |client, response| {
            assert_eq!(response.status(), Status::SeeOther);
            assert!(response.body().is_none());

            let location: Vec<_> = response.headers().get("Location").collect();
            assert_eq!(location, vec!["/hello/Unknown"]);
        });
    }

    // Check that other request methods are not accepted (and instead caught).
    for method in &[Post, Put, Delete, Options, Trace, Connect, Patch] {
        dispatch!(*method, "/", |client, response| {
            let mut map = std::collections::HashMap::new();
            map.insert("path", "/");
            //let expected = Template::show(client.cargo(), "error/404", &map).unwrap();

            assert_eq!(response.status(), Status::MethodNotAllowed);
            // FIND A MATCHING TEMPLATE TO HTTP 405 HERE
            //assert_eq!(response.body_string(), Some(expected));
        });
    }
}

#[test]
fn test_name() {
    // Check that the /hello/<name> route works.
    dispatch!(Get, "/hello/Jack%20Daniels", |client, response| {
        let context = TemplateContext {
            title: "Hello",
            name: Some("Jack Daniels".into()),
            items: vec!["One", "Two", "Three"],
            parent: "layout",
        };

        let expected = Template::show(client.cargo(), "index", &context).unwrap();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected));
    });
}

#[test]
fn test_404() {
    // Check that the error catcher works.
    dispatch!(Get, "/hello/", |client, response| {
        let mut map = std::collections::HashMap::new();
        map.insert("path", "/hello/");

        let expected = Template::show(client.cargo(), "error/404", &map).unwrap();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.into_string(), Some(expected));
    });
}
