# bevy-request

Bevy native http client.

This library is still in its early stage.

```rust
fn get_example_com(mut commands: Commands) {
    commands
        // spawn a request
        .spawn((GET, Uri("https://example.com".to_string())))
        // handle complete event
        .observe(
            |response: On<RequestComplete>| {
                match &response.result() {
                    Ok(response) => {
                        println!("status: {}", response.status());
                        println!("text: {}", response.body());
                    }
                    Err(error) => {
                        eprintln!("{:?}", error);
                    }
                }
            },
        );
}

```

## HTTP Backend

This library should switch to [nyquest](https://github.com/bdbai/nyquest) when it's mature enough, but in early development we choose to use `reqwest` + `async-compat`.
