use equatorial_energia::{Client, Credentials};

#[tokio::main]
async fn main() {
    let client = Client::new(
        equatorial_energia::States::AL,
        Credentials::new(
            equatorial_energia::LoginType::Birthday,
            "02483819460".to_string(),
            "1977-05-06".to_string(),
        ),
    )
    .unwrap();

    let client = client.login().await.unwrap();

    println!("{:#?}", client);
}
