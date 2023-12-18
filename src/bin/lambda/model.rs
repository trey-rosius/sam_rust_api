use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;

// Data Models
use serde::{Serialize, Deserialize};

mod error;

pub use error::Error;





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
/* 
#[derive(Deserialize, Serialize, Debug)]
pub enum AttributeValue {
    // B(Blob),
     Bool(bool),
    // Bs(Vec<Blob>),
    L(Vec<AttributeValue>),
    M(HashMap<String, AttributeValue>),
    N(String),
    Ns(Vec<String>),
    Null(bool),
    S(String),
    Ss(Vec<String>),
}

impl AttributeValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AttributeValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_l(&self) -> Option<&Vec<AttributeValue>> {
        match self {
            AttributeValue::L(l) => Some(l),
            _ => None,
        }
    }
    pub fn as_m(&self) -> Option<&HashMap<String, AttributeValue>> {
        match self {
            AttributeValue::M(m) => Some(m),
            _ => None,
        }
    }
    pub fn as_n(&self) -> Option<f64> {
        match self {
            AttributeValue::N(n) => n.parse::<f64>().ok(),
            _ => None,
        }
    }
    pub fn as_ns(&self) -> Vec<f64> {
        match self {
            AttributeValue::Ns(ns) => ns.iter().filter_map(|n| n.parse::<f64>().ok()).collect(),
            _ => Default::default(),
        }
    }
    pub fn as_null(&self) -> Option<bool> {
        match self {
            AttributeValue::Null(null) => Some(*null),
            _ => None,
        }
    }
    pub fn as_s(&self) -> Option<&str> {
        match self {
            AttributeValue::S(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_ss(&self) -> Vec<String> {
        match self {
            AttributeValue::Ss(ss) => ss.to_owned(),
            _ => Default::default(),
        }
    }
}

*/



impl From<&WeatherItem> for HashMap<String, AttributeValue> {
    /// Convert a &WeatherItem into a DynamoDB item
    fn from(value: &WeatherItem) -> HashMap<String, AttributeValue> {
        let mut retval = HashMap::new();
        retval.insert("id".to_owned(), AttributeValue::S(value.id.clone()));
        retval.insert("town".to_owned(), AttributeValue::S(value.town.clone()));
         retval.insert("temperature".to_owned(), AttributeValue::N(format!("{:}", value.temperature)));
          retval.insert("created_on".to_owned(), AttributeValue::S(value.created_on.clone()));
       

        retval
    }
}


/* 
impl From<&HashMap<String,aws_sdk_dynamodb::model::AttributeValue>> for WeatherItem {
    

    fn from(value: &HashMap<String,aws_sdk_dynamodb::model::AttributeValue>) -> Self {
         let result = WeatherItem{
        id: value
                .get("id")
                .ok_or(Error::InternalError("Missing id")).unwrap()
                .as_s()
             
                .unwrap()
                .to_string(),
           
            town: value
                .get("town").unwrap().as_s().unwrap().to_string(),
            weather: value
                .get("weather").unwrap().as_s().unwrap().to_string(),
            temperature:value
                .get("temperature").unwrap().as_n().unwrap().parse::<f64>().unwrap(),
            created_on: value
                .get("created_on").unwrap().as_s().unwrap().to_string(),
        };
      result
    }

   
    
}

*/

impl TryFrom<&HashMap<String, AttributeValue>> for WeatherItem {
    type Error = Error;

    /// Try to convert a DynamoDB item into a Product
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