use bevy::prelude::*;
use bevy_request::{GET, RequestPlugin, Response, Uri};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RequestPlugin)
        .add_systems(Startup, get_example_com)
        .run();
}

fn get_example_com(mut commands: Commands) {
    let uri = Uri("https://example.com".parse().unwrap());
    commands
        .spawn((GET, uri))
        // handle response
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
}
