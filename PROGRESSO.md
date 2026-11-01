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

### 2026-11-01, Fatia 0: o que o Horizon oferece
- Quem fez: Claude Sonnet 5.5
- Quem revisou: pendente (previsto: GPT-5.6 Terra)
- O que foi lido: Horizon `eb0d9e5` (`native_hooks.rs`, `machines/native_energy.rs`, `machines/native_fluids.rs`, `native_lifecycle.rs`, `block_entity.rs`, `docs/native-mods.md`, `docs/mods.md`)
- O que foi feito: `docs/fatia-0.md` preenchida (14 itens)
- Testes: nenhum; leitura de código apenas
- O que NÃO foi testado e por quê: nada foi executado nem compilado; não há código do projeto ainda. "Não encontrei" vem de busca textual e não prova ausência
- Pendências e perguntas ao dono: ver "Observações" em `docs/fatia-0.md`, em especial o bloqueio de entrada em mundo e o reinício automático

