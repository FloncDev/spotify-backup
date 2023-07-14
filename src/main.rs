use rspotify::{
    scopes, AuthCodeSpotify, Credentials, OAuth, Config,
    prelude::*, model::{SearchType, SearchResult::Playlists, PlaylistId, PlayableItem},
};
use serde::{Deserialize, Serialize};

async fn get_playlist_id(spotify: &AuthCodeSpotify, name: String) -> PlaylistId<'static> {
    let results = spotify.search(
        &name,
        SearchType::Playlist,
        None,
        None,
        Some(1),
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

#[derive(Serialize, Deserialize, Debug)]
struct PlaylistStore {
    id: String,
    tracks: Vec<String>
}

async fn searalize_playlist(spotify: &AuthCodeSpotify, id: PlaylistId<'_>) -> PlaylistStore {
    let playlist = spotify.playlist(id.clone(), None, None).await.unwrap();

    let mut tracks: Vec<String> = vec![];
    
    for track in playlist.tracks.items {
        let playable_item: PlayableItem = match track.track {
            None => {continue;}
            Some(playable_item) => {playable_item}
        };

        tracks.push(playable_item.id().unwrap().uri())
    }
    
    PlaylistStore{
        id: id.uri(),
        tracks
    }
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

    let playlist_id = get_playlist_id(&spotify, "Daily Drive".to_string()).await;
    let serialized = searalize_playlist(&spotify, playlist_id).await;

    println!("{:?}", serialized);
}
