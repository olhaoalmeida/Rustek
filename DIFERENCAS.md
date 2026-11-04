# Diferenças em relação ao Mekanism original

O Mekanism original é a especificação (`AGENTS.md`, regra 3). Toda diferença de valor, receita, tempo,
energia ou comportamento vai aqui, com o motivo.

## Modelo de entrada

```
### Nome da máquina, item ou regra
- Original: valor ou comportamento (versão do mod e arquivo)
- Mekanism Rust: valor ou comportamento
- Motivo: limitação do Horizon, decisão do dono, correção de bug
- Decidido por:
```

## Entradas

Nenhuma ainda.

## 2026-11-04 (Fatia 1)
- **Reinício automático**: o Horizon não tem hook para reiniciar o jogo (f40d4df). O mod mostra "Reinicie o jogo" e o jogador reinicia. Muda DESENHO.md seção 3 e 9.
- **Arquivo de configuração**: um arquivo por chave na pasta de dados do mod (`Host::setting`/`set_setting`), não um `.toml` único.
