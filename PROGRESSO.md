# Progresso

Registro de cada tarefa feita por uma IA ou por uma pessoa. Uma entrada por tarefa, a mais nova no topo.

## Modelo de entrada

```
### AAAA-MM-DD, Fatia N: título da tarefa
- Quem fez: modelo ou pessoa
- Quem revisou: modelo ou pessoa
- O que foi lido (arquivos do Horizon e do mod original, com versão):
- O que foi feito:
- Testes: o que rodou e o resultado
- O que NÃO foi testado e por quê:
- Pendências e perguntas ao dono:
```

## Entradas

### 2026-11-04, Fatia 1 (parcial): estrutura do mod e interruptores de módulo
- Quem fez: Claude Sonnet 5.5
- Quem revisou: pendente (previsto: GPT-5.6 Terra)
- O que foi lido: Horizon `f40d4df` (`docs/native-mods.md` "Trait mods", `native-hooks.json`, trait `Hooks` gerada, `crates/cargo-horizon-mod/template/native/`); Mekanism `53a328e` (README/LICENSE, contagem de classes)
- O que foi feito: `mod/` (bundle `rustek` provisório, pacote `api`), interruptores por módulo com aviso de reinício; licença do Mekanism registrada em `NOTICE.md`; diário em `docs/fatia-1.md`
- Testes: 6 testes de `modules.rs` passam (`rustc --test`); `cargo check`, `clippy -D warnings` e `fmt --check` do pacote contra o Horizon `f40d4df` passam
- O que NÃO foi testado e por quê: empacotar/instalar com o launcher e abrir no jogo (sem launcher nem janela nesta sessão). Reinício automático não existe (sem hook). Primeira máquina não começou. Calibração dos três modelos (DESENHO.md 10.4) não feita: o dono mandou começar
- Pendências e perguntas ao dono: id do mod, valor inicial dos módulos, licença dos assets; ver `docs/fatia-1.md`

Nenhuma ainda.
