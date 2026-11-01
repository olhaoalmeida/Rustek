# Fatia 0: o que o Horizon oferece para este projeto

Status: **preenchida, aguardando revisão** (escreveu: Claude Sonnet 5.5; revisor previsto: GPT-5.6 Terra, ver DESENHO.md 10.4).

Horizon analisado: commit `eb0d9e5805f40f7ee92c82700484ee09c26d5704` (2026-11-01), `NATIVE_HOOKS_API = 153`
(`crates/horizon-client/src/game/native_hooks.rs:519`). Os caminhos abaixo são do repositório `horizon-rust/horizon`;
`hooks.rs` abrevia `crates/horizon-client/src/game/native_hooks.rs`.

Como ler: **Existe** = li o código e o recurso está lá. **Não encontrei** = busquei no código e na documentação
e não achei; isso não prova que não exista. Nada foi executado: tudo é leitura de código. O que é ideia minha
está marcado "ideia, não testada".

| Item | Situação | Arquivo e linha | Como contornar se falta |
|---|---|---|---|
| Energia de máquina | **Existe**, com limite: todos os valores são `i32` | `hooks.rs:1942` (`native_block_entity_energy`); `crates/horizon-client-render/src/chunk/block_entity.rs:230` (`ModEnergyBuffer`: capacidade, máx. de entrada e de saída, faces); `crates/horizon-client/src/game/machines/native_energy.rs:137,161,175,191` (ler, inserir, extrair, mover entre dois blocos); `docs/native-mods.md`, API 142 | Falta saber se o `i32` basta: **não verifiquei o tipo numérico que o Mekanism usa**. Se for maior, ideia (não testada): guardar numa unidade menor ou pedir um hook ao Horizon |
| Fluidos de máquina | **Existe, mas só água e lava** | `hooks.rs:1926` (`native_block_entity_fluids`); `block_entity.rs:215` (`ModFluidTanks`); `machines/native_fluids.rs:16,199`; `docs/native-mods.md:565` ("Water and lava ... are the fluids"); recipientes em itens: API 147 (`native_item_fluid_tank`) | Fluidos próprios do mod **não encontrei**. Ideia (não testada): guardar o tipo e a quantidade no compound do próprio bloco e fazer a transferência na lógica do mod |
| Gases e químicos | **Não encontrei** | Busca por `chemical`, `gas`, `slurry`, `infus` em `crates/` e `docs/`: sem suporte; os poucos resultados são dados de teste (`hooks.rs:6194`) e uma receita de exemplo (`horizon-port-pack/src/carry.rs:1382`) | Ideia (não testada): tratar como um tipo de dado próprio do mod, guardado no compound do bloco, com transferência escrita no mod |
| Multiblocos | **Não encontrei** | Busca por `multiblock` em `crates/` e `docs/`: nenhum resultado | Ideia (não testada): lógica própria no tique do bloco (`hooks.rs:1970`) que confere os blocos vizinhos |
| Cabos e redes de energia | **Não encontrei rede**; existe o passo básico | `machines/native_energy.rs:191` (`native_move_energy`, move energia entre dois blocos); tique do servidor `hooks.rs:1491` | A rede (vários cabos, divisão, prioridade) teria de ser escrita no mod, em cima de `native_move_energy` |
| Entidades de bloco com estado salvo e tique | **Existe** | `hooks.rs:1870` (`native_block_entity`), `1880` (`native_block_entity_ticks`), `1911` (`native_block_entity_container`), `1970` (`native_block_entity_tick`); `docs/native-mods.md`, API 17 | n/a |
| Telas próprias de bloco (botões, dicas, slots) | **Existe** | `hooks.rs:2123` (`native_screen`), `2133` (`native_screen_draw`), `2514` (`native_screen_click`), `2527` (`native_screen_tooltip`), `2540` (`native_menu_button`); tela de mod sem bloco: `4630`, `4653`, `4660` (`native_mod_screen_open/close`), desenho em `2158` (`native_screen_render_post`) | n/a. Pelo comentário do código, a tela de mod "não pausa" o jogo e o gamepad não clica nela |
| Receitas de tipos próprios | **Existe** | `hooks.rs:4871` (`native_recipes_of_type`), `4933` (`native_recipes`), `1403` (`native_datapack_sync`, API 120, envia as receitas do servidor ao cliente) | n/a |
| Geração de minérios | **Existe, por dados** | `docs/mods.md:309-325` (features próprias entram em biomas pelo `neoforge:add_features` em `data/<ns>/neoforge/biome_modifier/`) | n/a |
| Página de configurações do mod | **Existe, só botões e teclas** | `hooks.rs:1844` (`native_mods`), `1850` (`native_settings`: lista de rótulos), `1854` (`native_settings_click`), `4786` (`native_mod_key`) | Não há controle com valor (interruptor, lista). Cada interruptor seria um botão cujo rótulo mostra "Ligado/Desligado" |
| Reiniciar o jogo a partir de um mod | **Não encontrei** | Nenhum hook de reinício. Só avisos no log que mandam o jogador reiniciar: `crates/horizon-client/src/game/mods.rs:182,202,221` | Falta. Opções: pedir ao Horizon um hook (decisão do dono), ou o aviso "reinicie o jogo" em vez de reinício automático |
| Gravar dados próprios por mundo (lista de módulos) | **Não encontrei** | Existe só a pasta global do mod (`hooks.rs:4807`, `native_data_dir`) e o compound de cada bloco. `NativeLifecycleContext` (`crates/horizon-client/src/game/native_lifecycle.rs:65`) não traz nome nem pasta do mundo | Falta. Sem saber qual mundo está aberto, não dá para guardar a lista por mundo. Precisa de hook novo ou outra fonte do mundo atual |
| Impedir a entrada em um mundo | **Não encontrei** | `WorldPhase` (`native_lifecycle.rs:43`) só tem `WorldOpenReady`, `WorldOpenFailed`, `WorldCloseFinal`, `WorldCloseFailed`, e o hook é "read-only" (`hooks.rs`, comentário de `native_world_lifecycle`): o mod vê, mas não pode vetar | Falta. Não verifiquei se existe jeito de um mod fechar o mundo depois de aberto. Provável hook novo no Horizon |
| Níveis (tiers) extras vindos de um módulo | **Não é uma lacuna do Horizon** | Busca por `tier` nos hooks e docs: nada, e não precisa | O Mekanism Rust será dono do próprio código de níveis, então o requisito é interno: o núcleo guarda uma lista de níveis e os módulos acrescentam entradas |

## Observações que afetam o desenho

1. **Ligar e desligar mods nativos**: na lista de mods do Horizon, um mod nativo desligado "descansa" até o
   launcher gerar o próximo build (`crates/horizon-client-render/src/mod_list.rs`, campo `disabled`). Isso vale
   para o mod inteiro, não para os módulos dentro dele. Os interruptores por módulo do DESENHO.md seriam um
   mecanismo nosso, lido na inicialização.
2. **Itens da seção 4 do DESENHO.md** (lista de módulos no mundo e bloqueio de entrada) dependem de duas
   lacunas desta tabela: dados por mundo e veto de entrada. Sem hook novo no Horizon, a regra "bloquear a
   entrada" não pode ser cumprida como está escrita. É uma decisão para o dono: pedir os hooks ao Horizon,
   ou mudar a regra.
3. **O Horizon muda rápido.** A API de hooks estava em 80 hoje cedo e está em 153 agora. A tabela vale para o
   commit acima e precisa ser conferida de novo antes de cada fatia.
4. Falta ainda verificar, no Mekanism original, os números que definem se o `i32` de energia basta e quais
   fluidos e gases a primeira máquina precisa. Isso exige o jar ou o código do Mekanism (não foi enviado).
