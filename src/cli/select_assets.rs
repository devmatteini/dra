use crate::cli::result::HandlerError;
use crate::github::release::Asset;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub struct Messages<'a> {
    pub select_prompt: &'a str,
    pub quit_select: &'a str,
}

pub type AskSelectAssetResult = Result<Asset, HandlerError>;

pub fn ask_select_asset(assets: Vec<Asset>, messages: Messages) -> AskSelectAssetResult {
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

fn assets_names(assets: &[Asset]) -> Vec<String> {
    assets.iter().map(|x| x.show_name().to_string()).collect()
}

fn find_asset_by_name(name: &str, assets: Vec<Asset>) -> Asset {
    assets.into_iter().find(|x| x.is_same_name(name)).unwrap()
}
