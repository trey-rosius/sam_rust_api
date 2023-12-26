use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;


use serde::{Serialize, Deserialize};
use crate::error::Error;




//Data model
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WeatherItem{
    pub id:String,
    pub town:String,
    pub weather:String,
    pub temperature:f64,
    pub created_on:String,
  
}


fn as_string(val: Option<&AttributeValue>, default: &String) -> String {
    if let Some(v) = val {
        if let Ok(s) = v.as_s() {
            return s.to_owned();
        }
    }
    default.to_owned()
}





fn as_f64(val: Option<&AttributeValue>, default: f64) -> f64{
     if let Some(v) = val {
        if let Ok(n) = v.as_n() {
            if let Ok(n) = n.parse::<f64>() {
                return n;
            }
        }
    }
    default
}



impl TryFrom<&HashMap<String, AttributeValue>> for WeatherItem {
    type Error = Error;

    /// Try to convert a DynamoDB item into a Weather Item
    ///
    /// This could fail as the DynamoDB item might be missing some fields.
    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(WeatherItem {
            id: as_string(value.get("id"), &"".to_string()),
            town: as_string(value.get("town"), &"".to_string()),
            weather: as_string(value.get("weather"), &"".to_string()),
            temperature:as_f64(value.get("temperature"), 0.0),
            created_on: as_string(value.get("created_on"), &"".to_string()),
           
        })
    }
}
