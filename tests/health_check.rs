use std::net::TcpListener;
#[tokio::test]
async fn health_check_works() {
    // Arrange
    // This will both spawn the background instance of the server and return the proper address to use
    let address = spawn_app(); 

    // We need to bring in reqwest to perform
    // HTTP requests against our app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


// This spawns the app without using any async components. This is a boundary that we have 
// to enforce in order to get this test to run. Otherwise, tokio would just be plodding along
// doing nothing with it
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future, 
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    
    // return the application address back to the caller
    format!("http://127.0.0.1:{}", port)
}