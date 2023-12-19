
use aws_sdk_dynamodb::{types::AttributeValue, Client};

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use tracing::info;

use std::env;
use svix_ksuid::*;
use sam_rust_api::{utils::{setup_tracing,api_gw_response},model::WeatherItem};


/// Main function
#[::tokio::main]
async fn main() -> Result<(), Error> {
    
  // Initialize logger
    setup_tracing();
    
     let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);

    lambda_runtime::run(service_fn(|request: LambdaEvent<ApiGatewayProxyRequest>| {
        post_weather_item(&dynamodb_client, &table_name, request)
    }))
    .await?;

    Ok(())
}

/// Put Item Lambda function
///
/// This function will run for every invoke of the Lambda function.
async fn post_weather_item(
    client: &Client,
    table_name: &str,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());

    let mut resp = api_gw_response();

     let ksuid = Ksuid::new(None, None);
   

    let request_body= match event.payload.body {
        Some(weather) => weather,
        None => {

            resp.body = Some("couldn't get body".into());
            resp.status_code = 400;
            return Ok(resp);
            

        },
    };

    let  weather_item:WeatherItem= serde_json::from_str(&request_body).unwrap();
   

    let mut item = weather_item.clone();
    item.id = ksuid.to_string();
    item.created_on = ksuid.timestamp_seconds().to_string();


     info!("Parsed item: {:?}", item);


   

    // Put the item in the DynamoDB table
    let res =  client
        .put_item()
        .table_name(table_name)
        .item("id", AttributeValue::S(item.id))
         .item("town", AttributeValue::S(item.town))
          .item("weather", AttributeValue::S(item.weather))
           .item("temperature", AttributeValue::N(item.temperature.to_string()))
             .item("created_on", AttributeValue::S(item.created_on))
       
        .send()
        .await;

        
         
    // Return a response to the end-user
    match res {
        Ok(_response) => {

            resp.body = Some("Weather item saved".into());
            resp.status_code = 200;
            Ok(resp)

        },
        Err(_err) =>{
             resp.body = Some("internal error".into());
            resp.status_code = 500;
            Ok(resp)
        }
        
   
    }
    
}

