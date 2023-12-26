use aws_lambda_events::apigw::ApiGatewayProxyResponse;


pub fn setup_tracing(){
        tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();
}

pub fn api_gw_response()->ApiGatewayProxyResponse{
         let resp = ApiGatewayProxyResponse {
        status_code: Default::default(),
        is_base64_encoded: Some(false),
        body:Default::default(),
        multi_value_headers: Default::default(),
        headers: Default::default(),
    };
   resp
}