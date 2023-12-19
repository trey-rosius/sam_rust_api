
use aws_sdk_dynamodb::{types::AttributeValue, Client};

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use tracing::{info, warn};

use std::env;

use sam_rust_api::{utils::{setup_tracing,api_gw_response},model::WeatherItem};








/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {

     // Initialize logger
    setup_tracing();

    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);

    // Register the Lambda handler
    //
    // We use a closure to pass the `dynamodb_client` and `table_name` as arguments
    // to the handler function.
    lambda_runtime::run(service_fn(|request: LambdaEvent<ApiGatewayProxyRequest>| {
        get_weather_item(&dynamodb_client, &table_name, request)
    }))
    .await?;

    Ok(())
}

/// get weather Item Lambda function
///
/// This function will run for every invoke of the Lambda function.
async fn get_weather_item(
    client: &Client,
    table_name: &str,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let mut resp = api_gw_response();
    
    // Extract path parameter from request
    let path_parameters = event.payload.path_parameters.get("id");
    let id:String = match path_parameters {
        Some(id) => {

            info!("Weather item id is {}",id);
             tracing::info!("(Value)={}", id);
            id.to_string()
        },
         None =>{
            warn!("Missing 'id' parameter in path");
              tracing::error!("Key doesn't exist");
            resp.status_code = 404;
            return Ok(resp)
         }
    };
    


    // Get the item in the DynamoDB table
    let response = client
        .get_item()
        .table_name(table_name)
         .key("id", AttributeValue::S(id.to_owned()))
        .send()
        .await?;
    let item_result:Result<Option<WeatherItem>, Error> =  Ok(match &response.item {
            Some(item) => Some(item.try_into()?),
            None => None,
        });


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


    