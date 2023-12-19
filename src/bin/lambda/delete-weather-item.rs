
use aws_sdk_dynamodb::{types::AttributeValue, Client};

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use tracing::{info, warn};

use std::env;
use sam_rust_api::utils::{setup_tracing,api_gw_response};

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
        delete_weather_item(&dynamodb_client, &table_name, request)
    }))
    .await?;

    Ok(())
}

/// delete weather Item Lambda function
///
/// This function will run for every invoke of the Lambda function.
async fn delete_weather_item(
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
        .delete_item()
        .table_name(table_name)
         .key("id", AttributeValue::S(id.to_owned()))
        .send()
        .await;
   

      
    match response{
        Ok(_response)=>{
  warn!("Item deleted successfully: {}", id);
tracing::info!("Item deleted successfully: {}", id);
             resp.status_code = 200;
            resp.body =Some(json!({"message": "Item deleted successfully"}).to_string().into());
           
           Ok(resp)
            
        },
      
       
        Err(err) => {
           warn!("Failed to delete weather item: {}", id);
           tracing::info!("Failed to delete weather item: {:?}",err );

             resp.status_code = 500;
            resp.body =Some(json!({"message": "Failed to delete weather item"}).to_string().into()); 
             Ok(resp)
        },
       
    }
   

}


    