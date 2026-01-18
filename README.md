# bevy-request

Bevy native http client.

This library is still in its early stage.

## Example

This is a minimal example which makes a get request to `https://example.com`.

```rust
fn get_example_com(mut commands: Commands) {
    commands
        // request as components
        .spawn((
            GET,
            Url("https://example.com".to_string()),
            GetContent::Text,
        ))
        // response as entity events
        .observe(|text: On<ResponseText>| {
            println!("text:\n{}", text.text);
        });
}
```

## Features

- Bevy-ish style using entity, component and entity event
- Support multiple Formats: bytes, text and (TODO) json
- Asynchronous content handling
- TODO: WebAssembly support
- TODO: platform native http stack

