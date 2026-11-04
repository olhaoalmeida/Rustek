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

## Como validar (comandos exatos, repetíveis)
Checkout fixo do Horizon: `/home/claude/deps/horizon` em `f40d4df781963ed0a6e1e0fd41bf740f652e2e46`.
```sh
# lógica pura dos interruptores (rápido, sem o jogo)
cd mod/api/src && rustc --edition 2024 --test modules.rs -o /tmp/modules_test && /tmp/modules_test
# o pacote contra o Horizon de verdade (~2 min na primeira vez; sem áudio: falta libasound2-dev na sessão)
cd mod/api && CARGO_TARGET_DIR=<fora do repo> cargo check --config .cargo-check-patch.toml
cargo clippy --config .cargo-check-patch.toml -- -D warnings && cargo fmt --check
```
`.cargo-check-patch.toml` é local (ignorado pelo git). Conteúdo: `[patch."https://github.com/horizon-rust/horizon"]`
`horizon-client = { path = "<checkout>/crates/horizon-client" }`, mais os três `[patch.crates-io]` do `Cargo.toml` raiz do
Horizon (`wgpu-hal`, `naga`, `libmimalloc-sys`, caminhos em `third_party/`). Sem eles o cargo não resolve `wgpu-hal`.
A trait `Hooks` gerada fica em `<target>/debug/build/horizon-client-*/out/mod_api_generated.rs` (e `..._host.rs`).

## Feito
- `mod/horizon-mixin.toml`, `mod/api/` (pacote `mod-rustek`). Id provisório `rustek` (pergunta ao dono: o id vira
  prefixo de blocos e itens e de pastas de dados, então precisa ser confirmado antes da primeira máquina).
- `mod/api/src/modules.rs`: os 6 módulos, estado "ativo" (retrato da abertura) x "desejado", rótulos dos botões,
  aviso de reinício. 6 testes, todos com valores escritos à mão. Passam.
- `mod/api/src/lib.rs`: liga isso a `Hooks::settings` (lista de rótulos) e `Hooks::settings_click` (vira o módulo e
  grava com `Host::set_setting("rustek", "module.<nome>", "true"|"false")`). O estado inicial vem de `Host::setting`.
  Compila e passa `clippy -D warnings` e `fmt --check` contra o Horizon `f40d4df`.
- O "arquivo de configuração" do DESENHO.md é, na prática, um arquivo por chave na pasta de dados do mod
  (`native_data_dir`, o próprio Horizon grava assim). Vai para `DIFERENCAS.md`.

## O que NÃO foi testado
- `horizon-launcher native-build` (empacotar e instalar): não rodei. O launcher não está nesta sessão.
- O jogo com o mod: nunca abriu. Não sei como o Horizon mostra a página de configurações de um mod; só sei que ele
  chama `settings`/`settings_click` (docs e catálogo). Teste no Windows do dono pendente.
- `horizon_revision` no `horizon-mixin.toml` e `rev` no `Cargo.toml` apontam para `f40d4df`; o Horizon muda rápido.

## Decisões provisórias (confirmar com o dono)
1. Valor inicial dos módulos na primeira execução: **todos desligados** (`DEFAULT_ENABLED` em `modules.rs`).
   O DESENHO.md diz só que o padrão de mundo novo é "o que o usuário decidir".
2. Rótulos em português escritos direto no código: a API de configurações recebe `String`, não chave de tradução.
3. Id do mod `rustek` (acima).

## Próximos passos (para a IA que continuar)
- [ ] Primeira máquina: energia + uma receita + uma tela. Ler antes `native_block_entity*`, `native_screen*`,
      `native_recipes_of_type` no catálogo (`native-hooks.json`) e `docs/native-mods.md` (blocos declarados por
      `native_blocks`). Os números de energia do Mekanism original precisam ser lidos em `mekanism/mekanism`
      (clone raso em `commit 53a328e`) antes de escolher `i32`.
- [ ] Testar no Windows com `horizon-launcher native-build mod --instance <nome> --trust-native-code`.
- [ ] Pedir revisão ao GPT-5.6 Terra (regra do projeto: quem escreve não revisa).
