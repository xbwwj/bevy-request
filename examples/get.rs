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
        .spawn((GET, Uri("https://example.com".to_string())))
        // handle complete event
        .observe(
            |response: On<RequestComplete>, mut app_exit: MessageWriter<AppExit>| {
                match &response.result() {
                    Ok(response) => {
                        println!("status: {}", response.status());
                        println!("text: {}", response.body());
                    }
                    Err(error) => {
                        eprintln!("{:?}", error);
                    }
                }

                // exit the program when request finish
                app_exit.write(AppExit::Success);
            },
        );
}
