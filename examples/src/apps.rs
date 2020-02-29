#![allow(dead_code)] // Just warns about un-used methods until they're used.

use heroku_rs::client::{Executor, Heroku};
use heroku_rs::defaults::{AppPatch, EnableFeature};
use serde_json::Value;

pub fn run() {
    let client = Heroku::new("API_KEY").unwrap();
    get_apps(&client);
    //get_app_features(&client);
    // patch_app(&client);
    // patch_feature(&client);
}

// == Getting an app ==
// Requires only the Heroku client to get all the apps
// If you want to get a specific app you can do so..run()
// by quering .app_name("NAME_HERE") or .app_id("ID_HERE")


fn get_apps(client: &Heroku) {
    let me = client
        .get()
        .apps() 
        .app_name("APP_NAME")
        .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json);
            }
        }
        Err(e) => println!("Err {}", e),
    }
}
// == Getting app features ==
// Requires the client and the app name
//  .app_features() returns all the features on this app
// or get a specific feature by name e.g. .feature_name("web-auto-scaling") OR can by id: feature_id("47d1998e-f8f4-432d-b4cc-f105f4d76407")

fn get_app_features(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_features()
        .feature_name("web-auto-scaling")
        .execute::<Value>();

    match me {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json);
            }
        }
        Err(e) => println!("Err {}", e),
    }
}

fn post_apps() {
    //TODO
}

// == Patching an app ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-update

fn patch_app(client: &Heroku) {
    // create the patch object, these are optional
    let app_patch = AppPatch {
        build_stack: String::from("BUILD_STACK"), // unique name or identifier of stack, you can get build_stack id from the get method
        maintenance: true,                        // maintenance status of app
        name: String::from("APP_NAME"),           //name of app
    };

    let result = client
        .patch(app_patch)
        .apps()
        .app_name("APP_NAME") //must specify exactly which app you want to patch
        .execute::<Value>();

    match result {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json);
            }
        }
        Err(e) => println!("Err {}", e),
    }
}
// == Patching a feature ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update
// Library provides a default struct EnableFeature which has only 1 prop, boolean, should enable the feature or not

fn patch_feature(client: &Heroku) {
    let enable = EnableFeature { enabled: true };
    let result = client
        .patch(enable)
        .apps()
        .app_name("spreventionbott")
        .app_features()
        .feature_name("web-auto-scaling")
        .execute::<Value>();

    match result {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json);
            }
        }
        Err(e) => println!("Err {}", e),
    }
}