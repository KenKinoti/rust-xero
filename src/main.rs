// Script should do the following
// 1. NIMBUS-1372 : Create a script to identify active connections for Xero organisation
// Active connection is an organisation that has an active token.

// 2. NIMBUS-1371 : Create a script to disconnect identified connections for Xero organisations
// Dion Appana (AU) to provide a spreadsheet.

// #################
// Hello world - Ping

#![feature(proc_macro_hygiene, decl_macro)]

#[warn(unused_variables)]
#[macro_use]
extern crate rocket;
use rocket::http::{Cookie, Cookies, Header, RawStr, SameSite};
use rocket::response::Redirect;
use rocket_oauth2::{OAuth2, TokenResponse};
use urlencoding::encode;


#[derive(Debug)]
struct GitHub;

#[get("/")]
fn ping() -> &'static str {
    "Hello, world! - PING ALIVE -- xero-oauth success"
}

// #################
// Set the app_family and create and Oauth connetion to
#[post("/disconnect")]
fn disconnect() -> &'static str {
    "Disconnect... Pending action"
}

#[get("/login/xero")]
fn xero_login(oauth2: OAuth2<GitHub>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2
        .get_redirect(
            &mut cookies,
            &["accounting.transactions accounting.contacts accounting.settings"],
        )
        .unwrap()
}

#[get("/auth/xero")]
fn xero_callback(token: TokenResponse<GitHub>, mut cookies: Cookies<'_>) -> Redirect {
    cookies.add_private(
        Cookie::build("token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish(),
    );

    Redirect::to("/xero/active")
}


// Active xero client
#[get("/xero/active")]
fn xero_active(mut cookies: Cookies) -> String {
    
    // Body

    // let resp = token;

    let token = cookies.get_private("token");
    
    // Set header
    // let header = Header::new("Authorization",token);
    // assert_eq!(header.to_string(), "Authorization: custom value");

    format!("Welcome to the xero_active api ---- = {:?} -----", token)

}

// Disconnect xero client
#[delete("/xero/disconnect")]
fn xero_disconnect(mut cookies: Cookies) -> String {
    // Set header 
    // url : https://api.xero.com/connections
        // Body

        let token = cookies.get_private("token");
        format!("Welcome to the XERO RUST api -------- xero_disconnect = {:?}", token)  
} 


// #################
// Main function calls all the routes
fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                disconnect,
                ping,
                xero_active,
                xero_callback,
                xero_disconnect,
                xero_login,
            ],
        )
        .attach(OAuth2::<GitHub>::fairing("custom"))
        // oauth connection
        // active connections endpoint
        // disconnect connections
        .launch();
}
