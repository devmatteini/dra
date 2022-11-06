use crate::github::release_new::AssetNew;
use crate::HandlerError;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub struct Messages<'a> {
    pub select_prompt: &'a str,
    pub quit_select: &'a str,
}

pub type AskSelectAssetResult = Result<AssetNew, HandlerError>;

pub fn ask_select_asset(assets: Vec<AssetNew>, messages: Messages) -> AskSelectAssetResult {
    let items = assets_names(&assets);
    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(messages.select_prompt)
        .default(0)
        .items(&items)
        .interact_opt()
        .map_err(|e| HandlerError::new(e.to_string()))?;
    if index.is_none() {
        return Err(HandlerError::op_cancelled(messages.quit_select));
    }
    let selected_name = &items[index.unwrap()];
    Ok(find_asset_by_name(selected_name, assets))
}

fn assets_names(assets: &[AssetNew]) -> Vec<String> {
    assets
        .iter()
        .map(|x| x.display_name.clone().unwrap_or_else(|| x.name.clone()))
        .collect::<Vec<String>>()
}

fn find_asset_by_name(name: &str, assets: Vec<AssetNew>) -> AssetNew {
    assets
        .into_iter()
        .find(|x| x.display_name.as_deref().filter(|&n| n == name).is_some() || x.name == name)
        .unwrap()
}
