//! Mekanism Rust: ponto de entrada do mod nativo (trait mod do Horizon).
//! Fatia 1: página de configurações com um interruptor por módulo. A lógica fica em `modules`.
mod modules;

use horizon_client::mod_api::{Hooks, Host, SettingsClickEvent, SettingsEvent};
use modules::State;
use std::sync::Mutex;

/// Mesmo `id` do `horizon-mixin.toml`.
const MOD_ID: &str = "rustek";

/// O retrato do que estava ligado quando o jogo abriu, tirado na primeira vez que a página abre.
static STATE: Mutex<Option<State>> = Mutex::new(None);

pub fn hooks() -> Box<dyn Hooks> {
    Box::new(Mod)
}

struct Mod;

fn with_state<R>(host: &dyn Host, f: impl FnOnce(&mut State) -> R) -> R {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    let state = guard.get_or_insert_with(|| State::load(|key| host.setting(MOD_ID, key)));
    f(state)
}

impl Hooks for Mod {
    fn settings(&self, host: &mut dyn Host, event: &mut SettingsEvent<'_>) {
        if event.mod_id != MOD_ID {
            return;
        }
        let labels = with_state(host, |s| s.labels());
        event.buttons.extend(labels);
    }

    fn settings_click(&self, host: &mut dyn Host, event: &mut SettingsClickEvent<'_>) {
        if event.mod_id != MOD_ID {
            return;
        }
        if let Some((module, on)) = with_state(host, |s| s.toggle(event.button)) {
            host.set_setting(MOD_ID, module.key(), modules::flag_text(on));
        }
    }
}
