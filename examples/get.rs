use bevy::prelude::*;
use bevy_request::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RequestPlugin)
        .add_systems(Startup, get_example_com)
        .run();
}

fn get_example_com(mut commands: Commands) {
    commands
        // spawn a request
        .spawn((
            GET,
            Url("https://example.com".to_string()),
            GetContent::Text,
        ))
        .observe(|received: On<ResponseReceived>| {
            println!("status: {}", received.status);
        })
        .observe(
            |text: On<ResponseText>, mut app_exit: MessageWriter<AppExit>| {
                println!("text:\n{}", text.text);
                app_exit.write(AppExit::Success);
            },
        )
        .observe(
            |error: On<ResponseError>, mut app_exit: MessageWriter<AppExit>| {
                println!("error: {:?}", error.error);
                app_exit.write(AppExit::error());
            },
        );
}
