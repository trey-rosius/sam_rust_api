
use aws_sdk_dynamodb::{types::AttributeValue, Client};

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use tracing::{info, warn};

use std::env;

use crate::model::WeatherItem;

pub mod model;



/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {

     tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);

    // Register the Lambda handler
    //
    // We use a closure to pass the `dynamodb_client` and `table_name` as arguments
    // to the handler function.
    lambda_runtime::run(service_fn(|request: LambdaEvent<ApiGatewayProxyRequest>| {
        list_weather_item(&dynamodb_client, &table_name, request)
    }))
    .await?;

    Ok(())
}

/// List weather Item Lambda function
///
/// This function will run for every invoke of the Lambda function.
async fn list_weather_item(
    client: &Client,
    table_name: &str,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let mut resp = ApiGatewayProxyResponse {
        status_code: Default::default(),
        is_base64_encoded: Some(false),
        body: Some(event.payload.path.unwrap().into()),
        multi_value_headers: Default::default(),
        headers: Default::default(),
    };

    let  next: Option<&str> = None;
    
    info!("Scanning DynamoDB table");
 

    // Get the item in the DynamoDB table
   let mut req = client
        .scan()
        .table_name(table_name).limit(20);
         req = if let Some(next) = next {
            req.exclusive_start_key("id", AttributeValue::S(next.to_owned()))
        } else {
            req
        };
        let res = req.send().await?;

         let weather_items = match res.items {
            Some(items) => items
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<Vec<WeatherItem>, Error>>()?,
            None => Vec::default(),
        };
        let next = res.last_evaluated_key.map(|m| m.get_s("id").unwrap());
    


    match item_result{
        Ok(Some(value)) => {
tracing::info!("(got weather item)={:?}", value);
            resp.status_code = 200;
            resp.body =Some(json!(value).to_string().into());
           
            Ok(resp)
        },
       Ok( None) => {
            warn!("Item not found: {}", id);
tracing::info!("Item not found: {}", id);
             resp.status_code = 200;
            resp.body =Some(json!({"message": "Item not found"}).to_string().into());
           
           Ok(resp)
            
        }
        Err(_err) => {
           warn!("Internal server error: {}", id);
           tracing::info!("Internal server error: {}", id);

             resp.status_code = 500;
            resp.body =Some(json!({"message": "Internal server error"}).to_string().into()); 
             Ok(resp)
        },
       
    }
   

}


    