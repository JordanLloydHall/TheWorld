use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .continue_to_state(GameState::Playing)
            .build(app);
    }
}

// fn configure_background_image(background: Res<TextureAssets>, mut images: ResMut<Assets<Image>>) {
//     // Doing this in response to AssetEvents seem to be broken, instead we try to set the sampler every frame
//     let mut image = images.get_mut(background.0.clone());
//     if let Some(image) = image {
//         image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
//         image.sampler_descriptor.address_mode_v = AddressMode::Repeat;
//     }
// }

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 57, rows = 30, padding_x = 1., padding_y = 1.))]
    #[asset(path = "textures/main_sprite_sheet.png")]
    pub sprites: Handle<TextureAtlas>,
    #[asset(path = "textures/rabbit.png")]
    pub rabbit: Handle<Image>,
    #[asset(path = "textures/burrow.png")]
    pub burrow: Handle<Image>,
    
}
