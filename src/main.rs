mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A hard-coded JSON
    let json = r#"
            {
              "main": {
                "temp": 30.94
              }
            }
        "#;

    // Deserialize the hardcoded JSON into a Weather struct
    let weather1: model::Weather = serde_json::from_str(json).unwrap();

    println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //

    //let client = reqwest::Client::new();


    let client = reqwest::Client::new();
    let p = model::User {
        username: "Amulya".into(),
        password: "123".into(),
    };

    let res = reqwest::Client::new()
        .post("http://54.234.254.71:3000/v1/auth")
        .json(&p)
        .send()
        .await?;

    let js = res.json::<model::UserResponse>().await?;

    // println!("{:#?}", js);

    // println!("Access token: {:#?}", js.access_token);
    let response = client
        .get("http://54.234.254.71:3000/v1/weather")
        .header("Authorization", "Bearer ".to_owned() + &js.access_token)
        .send()
        .await?;

    let weather2 = response.json::<model::Weather>().await?;

    println!(
        "\nWeather from server running on ec2 instance:\n {:?}",
        weather2
    );

    // fetch greeting
    let response = client
        .get("http://54.234.254.71:3000/v1/hello")
        .header("Authorization", "Basic ".to_owned() + &js.access_token)
        .send()
        .await?;

    let greeting = response.json::<model::GreetingResponse>().await?;

    println!(
        "\nGreeting from server running on ec2 instance:\n {:?}",
        greeting.message
    );


    Ok(())
}
