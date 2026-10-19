//! Whether a ribbon command exists and may run now. The ribbon asks for a few hundred ids each
//! frame, many of them repeated (split menus, slide-outs, collapsed-panel popups), so each id is
//! looked up once per pass and the answer kept in egui's temporary memory until the next pass.

use std::collections::HashMap;
use std::sync::Arc;

use asenacad_engine::Session;

/// What the ribbon knows about one command id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Availability {
    /// AsenaCAD doesn't have the command yet.
    Missing,
    /// It exists and may run now.
    Ready,
    /// It exists but refuses to run now; the reason its `enabled` check gave.
    Refused(Arc<str>),
}

impl Availability {
    pub fn is_ready(&self) -> bool {
        *self == Self::Ready
    }

    /// Whether AsenaCAD has the command at all (it may still refuse to run now).
    pub fn is_known(&self) -> bool {
        *self != Self::Missing
    }

    /// The tooltip of a control disabled for this reason: unbuilt commands are "not available
    /// yet"; built ones say why they can't run now.
    pub fn disabled_text(&self) -> String {
        match self {
            Self::Refused(reason) if !reason.trim().is_empty() => crate::i18n::t(reason).to_string(),
            Self::Refused(_) | Self::Ready => crate::tl!("Not available now").to_string(),
            Self::Missing => crate::tl!("Not available yet").to_string(),
        }
    }
}

/// A command's display label (engine commands have one) and its availability.
#[derive(Clone, Debug)]
pub struct CommandInfo {
    pub label: Option<&'static str>,
    pub availability: Availability,
}

#[derive(Clone, Default)]
struct Cache {
    pass: u64,
    entries: HashMap<String, CommandInfo>,
}

fn cache_id() -> egui::Id {
    egui::Id::new("asenacad-ribbon-commands")
}

fn look_up(session: &Session, id: &str) -> CommandInfo {
    match asenacad_engine::find_command(id) {
        // A placeholder that only refuses (`CommandSpec::stub`) isn't built yet.
        Some(spec) if spec.is_stub() => CommandInfo { label: Some(spec.label), availability: Availability::Missing },
        Some(spec) => {
            let availability = match (spec.enabled)(session) {
                Ok(()) => Availability::Ready,
                Err(reason) => Availability::Refused(reason.into()),
            };
            CommandInfo { label: Some(spec.label), availability }
        }
        None if crate::menus::is_ui_command(id) => CommandInfo { label: None, availability: Availability::Ready },
        None => CommandInfo { label: None, availability: Availability::Missing },
    }
}

/// `id`'s label and availability, looked up at most once per egui pass.
pub fn command(ctx: &egui::Context, session: &Session, id: &str) -> CommandInfo {
    let pass = ctx.cumulative_pass_nr();
    let cached = ctx.data_mut(|d| {
        let cache = d.get_temp_mut_or_default::<Cache>(cache_id());
        if cache.pass != pass {
            cache.pass = pass;
            cache.entries.clear();
        }
        cache.entries.get(id).cloned()
    });
    if let Some(info) = cached {
        return info;
    }
    let info = look_up(session, id);
    ctx.data_mut(|d| d.get_temp_mut_or_default::<Cache>(cache_id()).entries.insert(id.to_string(), info.clone()));
    info
}

/// `id`'s availability, looked up at most once per egui pass.
pub fn availability(ctx: &egui::Context, session: &Session, id: &str) -> Availability {
    command(ctx, session, id).availability
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_tooltips_tell_unbuilt_commands_from_refused_ones() {
        assert_eq!(Availability::Missing.disabled_text(), "Not available yet");
        assert_eq!(Availability::Refused("nothing selected".into()).disabled_text(), "nothing selected");
        assert_eq!(Availability::Refused(" ".into()).disabled_text(), "Not available now");
    }

    #[test]
    fn lookups_know_engine_and_ui_commands_and_are_cached_per_pass() {
        let ctx = egui::Context::default();
        let session = Session::default();
        assert_eq!(availability(&ctx, &session, "line"), Availability::Ready);
        assert_eq!(command(&ctx, &session, "LINE").label, Some("Line"), "ids ignore ASCII case");
        assert!(availability(&ctx, &session, "properties").is_known());
        assert_eq!(availability(&ctx, &session, "no-such-command"), Availability::Missing);
        assert_eq!(availability(&ctx, &session, "3dorbit"), Availability::Missing, "a placeholder isn't built yet");
        // A command that needs a selection refuses with its reason.
        let clear = availability(&ctx, &session, "erase.selection");
        assert_eq!(clear, Availability::Refused("nothing selected".into()));
        assert_eq!(clear.disabled_text(), "nothing selected");
        let cached = ctx.data_mut(|d| d.get_temp::<Cache>(cache_id())).map(|c| c.entries.len());
        assert_eq!(cached, Some(6), "one entry per distinct id ('line' and 'LINE' are looked up as given)");
    }
}
