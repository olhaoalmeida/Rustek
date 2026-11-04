//! Lógica pura dos módulos (sem tipos do Horizon, para poder ser testada sozinha).
//!
//! O estado "ativo" é o que estava ligado quando o jogo abriu; o "desejado" é o que o jogador
//! escolheu depois. Mudar o desejado só vale após reiniciar o jogo (DESENHO.md, seção 3).

/// Os módulos que dependem só do núcleo (DESENHO.md, seção 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Module {
    Generators,
    Tools,
    Additions,
    MoreMachine,
    Unleashed,
    Evolved,
}

pub const ALL: [Module; 6] = [
    Module::Generators,
    Module::Tools,
    Module::Additions,
    Module::MoreMachine,
    Module::Unleashed,
    Module::Evolved,
];

/// Valor de um módulo na primeira execução, quando nada foi gravado. O DESENHO.md não define
/// isso: escolha provisória (só o núcleo), a confirmar com o dono.
pub const DEFAULT_ENABLED: bool = false;

impl Module {
    /// Nome gravado no arquivo de configuração. Não muda nunca: mundos salvos vão depender dele.
    pub fn key(self) -> &'static str {
        match self {
            Module::Generators => "module.generators",
            Module::Tools => "module.tools",
            Module::Additions => "module.additions",
            Module::MoreMachine => "module.more_machine",
            Module::Unleashed => "module.unleashed",
            Module::Evolved => "module.evolved",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Module::Generators => "Generators",
            Module::Tools => "Tools",
            Module::Additions => "Additions",
            Module::MoreMachine => "More Machine",
            Module::Unleashed => "Unleashed",
            Module::Evolved => "Evolved",
        }
    }
}

/// Lê o texto gravado de um módulo. Qualquer valor desconhecido vira `None` (usa o padrão).
pub fn parse_flag(text: &str) -> Option<bool> {
    match text.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

pub fn flag_text(on: bool) -> &'static str {
    if on { "true" } else { "false" }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    active: [bool; 6],
    wanted: [bool; 6],
}

impl State {
    /// `read` devolve o texto gravado de uma chave, se houver. Tira o retrato do que está ativo.
    pub fn load(read: impl Fn(&str) -> Option<String>) -> State {
        let mut flags = [DEFAULT_ENABLED; 6];
        for (flag, module) in flags.iter_mut().zip(ALL) {
            if let Some(on) = read(module.key()).as_deref().and_then(parse_flag) {
                *flag = on;
            }
        }
        State {
            active: flags,
            wanted: flags,
        }
    }

    pub fn wanted(&self, module: Module) -> bool {
        self.wanted[index(module)]
    }

    pub fn active(&self, module: Module) -> bool {
        self.active[index(module)]
    }

    /// Vira o desejado do botão `button` (a posição na lista de botões). Devolve o módulo e o
    /// novo valor, para gravar; `None` se o botão não existe.
    pub fn toggle(&mut self, button: usize) -> Option<(Module, bool)> {
        let module = *ALL.get(button)?;
        let now = !self.wanted[button];
        self.wanted[button] = now;
        Some((module, now))
    }

    /// Há mudança que só vale depois de reiniciar.
    pub fn restart_needed(&self) -> bool {
        self.active != self.wanted
    }

    /// Rótulos dos botões, na ordem de `ALL`, mais o aviso de reinício quando precisa.
    pub fn labels(&self) -> Vec<String> {
        let mut out: Vec<String> = ALL
            .iter()
            .map(|m| {
                let state = if self.wanted(*m) {
                    "Ligado"
                } else {
                    "Desligado"
                };
                let pending = if self.active(*m) != self.wanted(*m) {
                    " (reinicie o jogo)"
                } else {
                    ""
                };
                format!("{}: {}{}", m.title(), state, pending)
            })
            .collect();
        if self.restart_needed() {
            out.push("Reinicie o jogo para aplicar as mudanças".to_owned());
        }
        out
    }
}

fn index(module: Module) -> usize {
    ALL.iter()
        .position(|m| *m == module)
        .expect("módulo em ALL")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn first_run_uses_the_default_for_every_module() {
        let s = State::load(none);
        assert!(ALL.iter().all(|m| s.wanted(*m) == DEFAULT_ENABLED));
        assert!(!s.restart_needed());
    }

    #[test]
    fn saved_values_are_read_and_garbage_is_ignored() {
        let s = State::load(|k| match k {
            "module.tools" => Some("true\n".to_owned()),
            "module.generators" => Some("talvez".to_owned()),
            _ => None,
        });
        assert!(s.active(Module::Tools));
        assert_eq!(s.active(Module::Generators), DEFAULT_ENABLED);
    }

    #[test]
    fn toggling_marks_restart_and_toggling_back_clears_it() {
        let mut s = State::load(|_| Some("false".to_owned()));
        assert_eq!(s.toggle(1), Some((Module::Tools, true)));
        assert!(s.restart_needed());
        assert!(s.active(Module::Tools) == false && s.wanted(Module::Tools));
        s.toggle(1);
        assert!(!s.restart_needed());
    }

    #[test]
    fn unknown_button_does_nothing() {
        let mut s = State::load(none);
        assert_eq!(s.toggle(6), None);
        assert!(!s.restart_needed());
    }

    #[test]
    fn labels_are_exact() {
        let mut s = State::load(|_| Some("false".to_owned()));
        s.toggle(0);
        let labels = s.labels();
        assert_eq!(labels.len(), 7);
        assert_eq!(labels[0], "Generators: Ligado (reinicie o jogo)");
        assert_eq!(labels[1], "Tools: Desligado");
        assert_eq!(labels[6], "Reinicie o jogo para aplicar as mudanças");
    }

    #[test]
    fn keys_are_stable() {
        let keys: Vec<_> = ALL.iter().map(|m| m.key()).collect();
        assert_eq!(
            keys,
            [
                "module.generators",
                "module.tools",
                "module.additions",
                "module.more_machine",
                "module.unleashed",
                "module.evolved"
            ]
        );
    }
}
