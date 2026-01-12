# bevy-request

Bevy native http client.

This library is still in its early stage.

```rust
commands
    .spawn((GET, uri))
    .observe(
        |mut response: On<Response>, mut app_exit: MessageWriter<AppExit>| match &mut response
            .result
        {
            Ok(response) => {
                println!("{}", response.status());
                // TODO: read_to_string should be background
                let text = response.body_mut().read_to_string().unwrap_or_default();
                println!("text: {}", text);

                // successfully exit
                app_exit.write(AppExit::Success);
            }
            Err(error) => {
                eprintln!("{:?}", error);

                // exit with error
                app_exit.write(AppExit::error());
            }
        },
    );
```

## HTTP Backend

This library should switch to [nyquest](https://github.com/bdbai/nyquest) when it's mature enough, but in early development we choose to use `reqwest` + `async-compat`.
