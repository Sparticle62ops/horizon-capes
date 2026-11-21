//! capes (Horizon Rust port), crate root + mod API wiring.
//!
//! Upstream: https://github.com/CaelTheColher/Capes
//! Copyright (C) CaelTheColher and contributors. Licensed under the GNU LGPL
//! v2.1; this port carries the upstream license and credit per the port
//! policy.
//!
//! The 1:1 port lives in the sibling modules (`capes_mod`, `config`,
//! `handler`, `menus`, `mixins`, `render`, `util`, `tests`); this root carries
//! the mod API wiring on the pinned SDK (Horizon f00125f61f, mixins e59da909,
//! mod api 0.2):
//! - the original's config toggles the Settings page can hold (the five
//!   provider toggles and the elytra texture toggle), same defaults as the
//!   config file, upstream stylized names as lang labels;
//! - `client_setup` runs the mod entry 1:1 (`Capes.CONFIG`'s create-or-load
//!   against `configDir/capes.json5`, defaulting on any error and patching a
//!   missing `clientCapeType` to `MINECRAFT`), then syncs the settings into
//!   the live config cells the handler and menus read.
//!
//! Gaps, tracked: `clientCapeType` is an enum and the setting page has no
//! enum kind, so it stays file-held (the cape selector menu drives it);
//! the `/capes debug` chat command waits on a client-command hook (the
//! command service is server-side); the mixin hooks that apply capes in
//! game (skin lookup, cape feature renderer, entity render dispatcher,
//! skin customization screen) wait on skin/cape render hooks and stay
//! behind the port's host traits in `mixins`.
//!
use mod_api::{ClientSetupEvent, ClientHost, RegistrationContext, Registrar, SettingSpec};

use crate::capes_mod::Capes;

pub mod capes_mod;
pub mod config;
pub mod handler;
pub mod menus;
pub mod mixins;
pub mod render;
pub mod util;
pub mod tests;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// The original's config toggles as settings, same defaults as the
    /// config file (`CapeConfig`'s fields; upstream stylized names as the
    /// lang labels, `options.capes.elytra` is upstream's own key).
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enableOptifine",
            "options.capes.setting.optifine",
            true,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enableLabyMod",
            "options.capes.setting.labymod",
            false,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enableMinecraftCapesMod",
            "options.capes.setting.minecraftcapes",
            false,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enableCosmetica",
            "options.capes.setting.cosmetica",
            false,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enableCloaksPlus",
            "options.capes.setting.cloaksplus",
            false,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enableElytraTexture",
            "options.capes.elytra",
            true,
        ));
    }

    /// 1:1 of the original's mod init (`Capes.CONFIG`'s create-or-load, the
    /// error path defaults to `CapeConfig()` and the missing `clientCapeType`
    /// patches to `MINECRAFT`), then the settings sync into the live cells.
    fn client_setup(&self, host: &mut dyn ClientHost, _event: &mut ClientSetupEvent) {
        Capes::set_config_io(std::rc::Rc::new(FsConfigIo::default()));
        let config = Capes::config();
        let enable_optifine = host.setting_bool("enableOptifine").unwrap_or(true);
        let enable_labymod = host.setting_bool("enableLabyMod").unwrap_or(false);
        let enable_minecraft_capes_mod = host
            .setting_bool("enableMinecraftCapesMod")
            .unwrap_or(false);
        let enable_cosmetica = host.setting_bool("enableCosmetica").unwrap_or(false);
        let enable_cloaks_plus = host.setting_bool("enableCloaksPlus").unwrap_or(false);
        let enable_elytra_texture = host.setting_bool("enableElytraTexture").unwrap_or(true);
        Capes::install_config(crate::config::CapeConfig {
            enable_optifine,
            enable_labymod,
            enable_minecraft_capes_mod,
            enable_cosmetica,
            enable_cloaks_plus,
            enable_elytra_texture,
            ..config
        });
    }
}

/// The config-file surface upstream performs against Fabric's config dir
/// (`configDir/capes.json5`, Gson pretty-printed). `CAPES_CONFIG_DIR`
/// overrides the directory for tests, mirroring upstream's injectable path.
pub struct FsConfigIo {
    dir: std::path::PathBuf,
}

impl Default for FsConfigIo {
    fn default() -> Self {
        Self {
            dir: std::env::var_os("CAPES_CONFIG_DIR")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| std::path::PathBuf::from("config")),
        }
    }
}

impl crate::capes_mod::ConfigIo for FsConfigIo {
    fn read_config(&self) -> Option<String> {
        std::fs::read_to_string(self.dir.join("capes.json5")).ok()
    }

    fn write_config(&self, json: &str) {
        let path = self.dir.join("capes.json5");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(path, format!("{json}\n"));
    }
}
