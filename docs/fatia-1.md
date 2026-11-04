# Fatia 1: estrutura do mod nativo (diário de trabalho)

Status: **em andamento**. Este arquivo existe para outra IA continuar de onde esta parou. Atualize-o a cada passo.
Quem escreve: Claude Sonnet 5.5. Revisor previsto: GPT-5.6 Terra (DESENHO.md 10.4).
Branch: `fatia-1` (criada de `origin/main` em `dbb5ce4`).

## Como o dono pediu (2026-11-04)
"pode fazer todos ... comece a parte 1 e deixe registrando o que você estava fazendo e como estava fazendo".
Leitura adotada: parte 1 = Fatia 1 do DESENHO.md seção 7. O dono disse que renomeou o repositório para `rustek`
(não verificado: o acesso da sessão ao GitHub só cobre o nome antigo `mekanism-evolved-rust`).

## Fatos verificados sobre o Horizon (commit `f40d4df`, API 359, lido em 2026-11-04)
- Um mod nativo com código próprio é um **"trait mod"**: `docs/native-mods.md`, seção "Trait mods (draft)".
  Pasta com `horizon-mixin.toml` (com `[api] path = "api"`) e um pacote Rust `api/` que exporta
  `pub fn hooks() -> Box<dyn horizon_client::mod_api::Hooks>`. Molde: `crates/cargo-horizon-mod/template/native/`
  no repositório do Horizon, ou `cargo horizon-mod new <pasta> --template native --horizon <commit>`.
- A trait `Hooks` é **gerada na compilação** a partir de `native-hooks.json` (na raiz do Horizon) e das assinaturas
  em `crates/horizon-client/src/game/native_hooks.rs` (`crates/horizon-mod-api-gen`). Por isso os nomes exatos dos
  métodos e eventos só se leem no código gerado (`OUT_DIR/mod_api_generated.rs`). Não escreva assinaturas de memória.
- Hooks que a Fatia 1 usa (nome no catálogo, contrato): `settings` (collect, devolve botões), `settings_click`
  (listen), `world_lifecycle` (listen), `block_entity_energy` (replace), `mods` (collect).
- Pacotes extras só podem ser crates e versões que já estão no `Cargo.lock` do Horizon.
- O arquivo do pacote (`.hnmod`) só aceita `.rs`, `.toml`, `data/`, README/LICENSE/NOTICE (docs/api/mod-files.md).

## O que NÃO existe no Horizon (de `docs/fatia-0.md`, reconferido em f40d4df)
Gases/químicos, multiblocos, rede de cabos, veto de entrada em mundo, reinício pelo mod. Por isso a Fatia 1 **não
entrega reinício automático**: mostra o aviso "reinicie o jogo". Isso muda o DESENHO.md (seção 9, pergunta 1) e
deve ir para `DIFERENCAS.md` quando o código existir.

## Como estou validando
`cargo check -p horizon-client --lib` no pinado `/home/claude/deps/horizon` (sem git; é um checkout fixo),
com `CARGO_TARGET_DIR` fora dele, para gerar a trait. Depois, compilar o pacote do mod contra esse Horizon.
Montar o jogo inteiro leva ~16 min (ver o repositório Molt, `tools/horizon.py`).

## Pendências
- [ ] Gerar a trait `Hooks` e anotar aqui os nomes reais dos métodos usados.
- [ ] Pacote `api/` com módulos: `config` (lê `rustek.toml`), `modules` (lista de módulos e estado), `settings`
      (botões "Ligado/Desligado"), `machine` (primeira máquina de energia e itens).
- [ ] Testes de comportamento da config (valores esperados escritos à mão, não derivados do código).
- [ ] Atualizar `PROGRESSO.md`, `DIFERENCAS.md`, `docs/fatia-0.md` (API 359).
- [ ] A calibração dos três modelos (DESENHO.md 10.4) **não foi feita**; o dono mandou começar. Registrado.
