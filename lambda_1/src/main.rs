// Crates in order to implement parsing a request, generating a Taxi instance and adding a record to DynamoDb
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

use chrono::Utc;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::debug;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemOutput};
use serde_derive::{Serialize, Deserialize};

// Main function that calls the handler function
fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    debug!("Starting up lambda with Rust...");
    lambda!(handler);
    Ok(())
}

// Implementing the Handler
// Creates a connection to DynamoDB using default Region, if no region is passed uses us-east-1
// Afterwards extracts username provided by Cognito that is used to authorize users 
// No need to implement user registration manually
fn handler(event: Request, _: Context) -> Result<Response,
HandlerError> {
    let region = Region::default();
    let client = DynamoDbClient::new(region);
    let username = event 
        .request_context
        .authorizer
        .claims
        .get("cognito:username")
        .unwrap()
        .to_owned();
    debug!("USERNAME: {}, username");
    // Generate unique ID of a ride and exrtact the body of a request 
        let ride_id = Uuid::new_v4().to_string
        let request: RequestBody = serdejson::from_str(&event.body).unwrap();
        let car = find_car(&request.pickup_location);
        record_ride(&client, &ride_id, &username, &car).unwrap();
        // After record is added in DB, construct a response
        let body = ResponseBody {
            ride_id: ride_id.clone(),
            car_nameL car.name.clone(),
            car,
            eta: "30 seconds".into(),
            rider: username.clone(),
        };
        let mut headers = HashMap::new();
        headers.insert("Access-Control-Allow-Origin".into(), "*".into());
        let body = serde_json::to_string(&body_).unwrap();
        let resp = Response {
            status_code: 201,
            body,
            headers
        };
        Ok(resp)
}

// Main struct Car that contains details about the driver
#[derive(Clone, Serialize)]
 #[serde(rename_all = "PascalCase")]
 struct Car {
    name: String,
    color: String,
    gender: String,
 }

// Constructor to simplifying creation of instance in the code
impl Car {
    fn new(name: &str, color: &str, gender: &str) -> Self{
        Car {
            name: name.to_owned(),
            color: color.to_owned(),
            gender: gender.to_owned(),
        }
    }
}

// Location struct that represents a point on a map that will be set by the UI of the application
#[dervive(Deserialize)]
 #[serde(rename_all = "PascalCase")]
 struct Location {
    latitude: f64,
    longtitude: f64,
 }

// Declaring the Request struct that contains body and request_context fields to get usename provided by Cognito
#[derive(Deserialize)]
 #[serde(rename_all = "camelCase")]
 struct Request {
    body: String,
    request_context: RequestContext,
 }

// RequestContext is a map filled by the runtime that is parsed to a struct
#[derive(Deserialize)]
 #[serde(rename_all = "camelCase")]
 struct RequestContext {
    authorizer: Authorizer,
 }

#[derive(Deserialize)]
 #[serde(rename_all = "camelCase")]
 struct Authorizer {
    claims: HashMap<String, String>,
 }

#[derive(Deserialize)]
 #[serde(rename_all = "PascalCase")]
 struct RequestBody {
    pickup_locaiton: Location,
 }

// Declaring a Response used by API Gateway
#[derive(Serialize)]
 #[serde(rename_all = "camelCase")]
 struct Response {
    body : String,
    status_code: u16,
    headers: HashMap<String, String>,
 }

#[derive(Serialize)]
 #[serde(rename_all = "PascalCase")]
 struct ResponseBody {
    ride_id: String,
    car: Car,
    car_name: String,
    eta: String,
    rider: String,
 }