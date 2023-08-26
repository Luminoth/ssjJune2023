use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_tokio_tasks::TaskContext;
use futures_lite::FutureExt;

use common::http::*;

use crate::components::{client::character_select::*, reqwest::*};
use crate::events::client::*;
use crate::plugins::client::character_select::*;
use crate::resources::{client::auth::*, reqwest::*};

pub fn enter(
    mut commands: Commands,
    reqwest_client: Res<ReqwestClient>,
    authorization: Res<AuthorizationResource>,
    mut character_select_state: ResMut<NextState<CharacterSelectState>>,
) {
    info!("entering CharacterSelect state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnCharacterSelect));

    // TODO: move this to a Character plugin, similar to the Auth plugin

    // TODO: error handling
    let request = reqwest_client
        .get("http://localhost:3000/characters")
        .bearer_auth(authorization.get_access_token())
        .build()
        .unwrap();

    commands.spawn(ReqwestRequest::new(request, move |resp, ctx| {
        characters_response_handler(resp, ctx).boxed()
    }));

    character_select_state.set(CharacterSelectState::WaitForCharacters);
}

async fn characters_response_handler(
    resp: Result<bytes::Bytes, reqwest::Error>,
    mut ctx: TaskContext,
) {
    ctx.run_on_main_thread(move |ctx| {
        match resp {
            Ok(response) => {
                // TODO: error handling
                let _response = serde_json::from_slice::<GetCharactersResponse>(&response).unwrap();

                // TODO: insert characters resource

                ctx.world.send_event(CharactersUpdated);
            }
            Err(err) => {
                error!("http error: {:?}", err);

                // TODO: deeply error check this,

                // TODO: set character select error resource

                // TODO: need to tell the current game state what happened
            }
        }
    })
    .await
}

pub fn exit() {
    info!("exiting CharacterSelect state");
}

pub fn wait_for_characters(
    mut events: EventReader<CharactersUpdated>,
    mut character_select_state: ResMut<NextState<CharacterSelectState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Character Select").show(contexts.ctx_mut(), |ui| {
        ui.label("Retrieving characters ...");
    });

    if events.iter().next().is_some() {
        info!("retrieved characters: {:?}", "TODO");

        character_select_state.set(CharacterSelectState::WaitForCharacterSelect);
    }

    events.clear();
}

pub fn wait_for_character_select(mut contexts: EguiContexts) {
    // TODO: allow creating a new character (with a max that we get from the backend)

    egui::Window::new("Character Select").show(contexts.ctx_mut(), |ui| {
        ui.label("TODO: Character Select");
    });
}
