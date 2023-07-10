use rspotify::{
    scopes, AuthCodeSpotify, Credentials, OAuth, Config,
    prelude::*, model::{SearchType, SearchResult::Playlists, PlaylistId},
};

async fn get_playlist_id(spotify: AuthCodeSpotify, name: &str) -> PlaylistId {
    let results = spotify.search(
        format!("playlist: {}", name).as_str(),
        SearchType::Playlist,
        None,
        None,
        Some(10),
        None
    ).await;

    match results {
        Ok(Playlists(page)) => {
            page.items[0].id.clone()
        },
        Ok(_) => { panic!("Somehow returned non-playlist") },
        Err(err) => { println!("Couldn't find {name}: {err:?}"); panic!() }
    }
}

async fn searalize_playlist(spotify: AuthCodeSpotify, id: PlaylistId<'_>) {
    let playlist = spotify.playlist(id, None, None).await.unwrap();

    // Need to figure out how im going to be storing the playlists
    // For now probably just going to make it into json
    todo!()
}

#[tokio::main]
async fn main() {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("playlist-read-private")).unwrap();

    let config = Config {
        token_refreshing: true,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let auth_url = spotify.get_authorize_url(false).unwrap();

    spotify
        .prompt_for_token(&auth_url)
        .await
        .expect("Couldn't Auth Successfully!");

    // let me = spotify.me().await.unwrap();

    let playlist_id = get_playlist_id(spotify, "Discover Weekly").await;

    println!("{playlist_id:?}");
}
