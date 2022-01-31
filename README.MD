# RUST-XERO
RUST based CRUD API to connect to XERO

## Quick start
These instructions will get a copy of this project up and running on your local machine for further development and testing. See deployment for notees on how to deploy the project on a live system.

## Prerequisites
- RUST
- `cargo` Package manager

### Dependencies
- `rocket = "0.4.10"`
- `rocket_oauth2 = "0.4.1"`
- `urlencoding = "2.1.0"`

## Installing
- Pull this repo
- Navigate to the home directory
- You need to make sure you have RUST installed to as to use the cargo commands
- Use the command `cargo install <package-name>` to install dependencies
- Open the `Rocket.toml` file and update the xero client_id, client_secret

## Launching the program
Use the command `cargo run` to build and run a binary of the local package

## Browser Based API End Points
### ping
    Path : http://localhost:8000/
    Method: GET
    Fields: {

    }

    Responses: Hello, world! - PING ALIVE

### login
    Path : http://localhost:8000/login/xero
    Method: GET
    Fields: {

    }

    Response : Redirect - Xero authentication based on app configured in `Rocket.toml` 
    
### callback 
    Path : http://localhost:8000/auth/xero
    Method: GET
    Fields: {

    }

    Responses: Redirect to `/xero_active` route after successfull authentication

### access_token 
    Path : http://localhost:8000/xero_active
    Method: GET
    Fields: {

    }

    Responses: <access_token>

## Postman End points
### active_connections
    Path : https://api.xero.com/connections
    Method: GET
    Headers: {
        Accept:application/json
        Authorization:<access_token>
    }
    Responses: 200, 400

### disconnect_connections
    Path : https://api.xero.com/connections
    Method: DELETE
    Headers: {
        Accept:application/json
        Authorization:<access_token>
    }
    Responses: 200, 400

## Deployment
Not deployed to any cloud CI/CD
- 

## Testing
No testing cases added

## Built with 
- Rust - Core backend language

## Authors
- Kennedy Kinoti

## Acknowledgments
Once more, I give a thumbs up to google and r/rust

> "Change is slow, always has been."

~ :studio_microphone: J. Cole