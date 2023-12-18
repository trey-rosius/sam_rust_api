
use aws_sdk_dynamodb::{types::{AttributeValue, ReturnValue}, Client, };

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use tracing::{info, warn};

use std::env;
use svix_ksuid::*;

use crate::model::WeatherItem;

pub mod model;


/// Main function
#[::tokio::main]
async fn main() -> Result<(), Error> {
    
 tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    
     let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);

    lambda_runtime::run(service_fn(|request: LambdaEvent<ApiGatewayProxyRequest>| {
        update_weather_item(&dynamodb_client, &table_name, request)
    }))
    .await?;

    Ok(())
}

/// update Item Lambda function
///
/// This function will run for every invoke of the Lambda function.
async fn update_weather_item(
    client: &Client,
    table_name: &str,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());

     let mut resp = ApiGatewayProxyResponse {
        status_code: Default::default(),
        is_base64_encoded: Some(false),
        body: Some(event.payload.path.unwrap().into()),
        multi_value_headers: Default::default(),
        headers: Default::default(),
    };

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
    let request_body= match event.payload.body {
        Some(weather) => weather,
        None => {

            resp.body = Some("couldn't get body".into());
            resp.status_code = 400;
            return Ok(resp);
            

        },
    };

    let  weather_item:WeatherItem= serde_json::from_str(&request_body).unwrap();
   

     info!("Parsed item: {:?}", weather_item);
 
    // Put the item in the DynamoDB table
    let res =  client
        .update_item()
        .table_name(table_name)
         .key("id", AttributeValue::S(id.to_owned()))
        .update_expression("SET #weather=:weather, #town=:town, 
        #temperature=:temperature")
        .expression_attribute_names("#weather", "weather")
         .expression_attribute_names("#town", "town")
          .expression_attribute_names("#temperature", "temperature")
          
        .expression_attribute_values(":weather",AttributeValue::S(weather_item.weather.to_owned()) )
        .expression_attribute_values(":town",AttributeValue::S(weather_item.town.to_owned()) )
        .expression_attribute_values(":temperature",AttributeValue::N(weather_item.temperature.to_owned().to_string()))
     
        .return_values(ReturnValue::UpdatedNew)
      
       
        .send()
        .await;

        
         
    // Return a response to the end-user
    match res {
        Ok(_response) => {

            resp.body = Some("Weather item updated successfully".into());
            resp.status_code = 200;
            Ok(resp)

        },
        Err(err) =>{
             warn!("failed to update weather items with error {:?}",err);
              tracing::error!("failed to update weather items with error {:?}",err);
             resp.body = Some("Failed to update weather item".into());
            resp.status_code = 500;
            Ok(resp)
        }
        
   
    }
    
}

