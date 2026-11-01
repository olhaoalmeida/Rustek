# Instruções para IAs neste repositório

Projeto: **Mekanism Rust**, um mod para o Horizon (Minecraft 26.3 em Rust). O plano completo está em
[DESENHO.md](DESENHO.md), seção 10. Resumo:

1. Não ache nada: se não souber, pergunte ao dono antes de seguir.
2. Leia o código do Horizon e o comportamento do Mekanism original antes de escrever.
3. O Mekanism original é a especificação. Diferenças vão para `DIFERENCAS.md`.
4. Uma tarefa por vez, um commit por tarefa, uma branch por fatia, nunca direto em `main`.
5. Código vindo de mod MIT leva o aviso de copyright (`NOTICE.md`). Sem jar, textura ou som de terceiros sem confirmar licença.
6. Sem dependência externa além do Horizon.
7. Pronto = compila, passa formatador e linter, tem teste de comportamento com valor esperado independente do código, e `PROGRESSO.md` atualizado. Diga o que não pôde testar.
8. Nunca publique, abra PR em repositório alheio, fale com autores nem mexa em dados reais de jogador sem pedido explícito.
9. Commits terminam com a linha `Co-Authored-By` do modelo usado.
