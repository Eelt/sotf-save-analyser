use std::fs;

use serde_json::Value;

pub fn deserialize_json(path: String) -> Vec<JsonStore> {

    let mut json_data = fs::read_to_string(path);
    let mut json_string;

    match json_data {
        Ok(_) => json_string = json_data.unwrap(),
        Err(_) => panic!("Error file likely doesn't exist")
    }

    let json_object: Value = serde_json::from_str(&json_string).expect("Something went wrong with deseralizing the file. Is it JSON formatted?");

    let data_field = json_object["Data"].clone();

    let store = recursive_deserialization(data_field);

    store
}

pub fn json_store_walker(store: Vec<JsonStore>) {

    for x in store {
        let has_children = x.children.is_some();
        let has_payload = x.payload.is_some();

        println!("Field: {}, Contains Payload?: {}, Contains Children?: {}", x.field, has_payload, has_children);

        // TODO
        if has_payload {
            println!("Payload: {}", payload_to_string(x.payload.unwrap()));
        }

        if has_children {
            println!("Zooming into {}", x.field);

            json_store_walker(x.children.unwrap());
        }

    }
    
}

pub fn payload_to_string(value: Value) -> String {

    if value.is_array() {
        // TODO
    }

    if value.is_boolean() {
        return value.as_bool().unwrap().to_string();
    }

    if value.is_f64() {
        return value.as_f64().unwrap().to_string();
    }

    if value.is_i64() {
        return value.as_i64().unwrap().to_string();
    }

    if value.is_u64(){
        return value.as_u64().unwrap().to_string();
    }

    if value.is_string(){
        return String::from(value.as_str().unwrap());
    }

    if value.is_object(){

        let x = serde_json::to_string_pretty(&value);

        if !x.is_err() {
            return x.unwrap();
        }
        
    }

    if value.is_null(){
        return String::from("");
    }

    String::from("")
}

#[derive(Clone)]
pub struct JsonStore {
    pub field: String,
    pub payload: Option<Value>,
    pub children: Option<Vec<JsonStore>>
}

fn recursive_deserialization(json: Value) -> Vec<JsonStore> {
    let mut store: Vec<JsonStore> = Vec::new();

    if let Value::Object(fields) = json {
        for (key, value) in fields.iter() {
            // println!("Checking key {}", key); //DEBUG

            if value.as_str().is_none() { // Value is straight up not a string
                if value.is_array() {
                    let json_array = value.as_array().expect("Failed to unwrap array");
                    for x in json_array {

                        if !x.as_str().is_none() {
                            let recieved_stores = latter_recursion_helper(key, x);

                            let recived_encap = JsonStore {
                                field: key.clone(), 
                                payload: Option::None, 
                                children: Option::Some(recieved_stores)
                            };

                            store.push(recived_encap);
                        }
                    }

                    continue;
                } else { // Not string nor array; but MAY have JSON contents

                    if value.as_object().is_none() {
                        // println!("k: {} v: {}", key, value); //DEBUG

                        store.push(JsonStore {
                            field: key.clone(),
                            payload: Option::Some(value.clone()),
                            children: Option::None
                        });

                    } else {
                        let recieved_stores = latter_recursion_helper_obj(value.clone());

                        let recieved_encap = JsonStore {
                            field: key.clone(), 
                            payload: Option::None, 
                            children: Option::Some(recieved_stores)
                        };

                        store.push(recieved_encap);
                    }

                    continue;
                }

            }

            let recieved_stores = latter_recursion_helper(key, &value);

            let recieved_encap = JsonStore {
                field: key.clone(),
                payload: Option::None,
                children: Option::Some(recieved_stores)
            };

            store.push(recieved_encap);
        }
    }

    store
}

fn latter_recursion_helper(key: &String, value: &Value) -> Vec<JsonStore> {

    let sub_json_object = serde_json::from_str(value.clone().as_str().unwrap());

    if !sub_json_object.is_err() {
        // println!("k: {} holds json", key); //DEBUG
        return recursive_deserialization(sub_json_object.unwrap())
    } else { // Value is a string, but doesn't constitute a JSON object
        // println!("k: {} v: {}", key, value); //DEBUG
    }

    let mut store = Vec::new();

    store.push(JsonStore { field: key.clone(), payload: Some(value.clone()), children: Option::None });

    store
}

fn latter_recursion_helper_obj(value: Value) -> Vec<JsonStore> {
    let mut store = Vec::new();

    let subjson_collection = value.as_object().unwrap();
    for (key, value) in subjson_collection {

        store.push(
            JsonStore { 
                field: key.clone(), 
                payload: Option::None, 
                children: Option::Some(recursive_deserialization(value.clone()))
            }
        );

    }

    store
}