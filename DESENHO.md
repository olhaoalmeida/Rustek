# Mekanism Horizon: desenho (rascunho)

Nome do projeto: a definir entre "Mekanism Rust" e "Mekanism Horizon" (ver seção 9).

Status: rascunho para revisão. Nenhum código foi escrito. Será retomado quando o Horizon tiver os
recursos necessários (ver "Dependências do Horizon").

## 1. Objetivo

Um único mod para o Horizon que reúne o Mekanism e seus addons. O jogador escolhe, nas
configurações do mod, quais partes ficam ligadas. A escolha vale para o jogo inteiro.

## 2. Escopo

Entram (um mod só):

| Módulo | Origem | Depende de |
|---|---|---|
| Núcleo (Mekanism) | Mekanism principal | nada |
| Generators | Mekanism Generators | núcleo |
| Tools | Mekanism Tools | núcleo |
| Additions | Mekanism Additions | núcleo |
| More Machine | Mekanism More Machine | núcleo |
| Unleashed | Mekanism Unleashed | núcleo |
| Evolved | Evolved Mekanism | núcleo |

Fora do escopo por enquanto:
- A iglee-library: o projeto não depende dela (ver seção 6).
- Portar qualquer código Java agora. O trabalho de porte começa só depois que o Horizon estiver pronto.

## 3. Como o jogador escolhe

- Um interruptor por módulo, na página de configurações do mod (a página "Mods" do Horizon, botão
  de configurações, como os mods nativos já têm).
- A mesma escolha é gravada em um arquivo na pasta de configuração do jogo, para quem prefere editar
  à mão. Se o arquivo e a tela divergirem, vale o arquivo, lido na inicialização.
- Mudar um interruptor **não aplica na hora**. Ao salvar, aparece o aviso "Reiniciando", e o jogo
  fecha e reabre sozinho para aplicar. A escolha fica salva.
- Regra de dependência: todos os módulos dependem do núcleo, que é o próprio mod e está sempre
  ligado. Nenhum módulo depende de outro módulo, nem de mod externo.
- **Padrão de mundo novo:** os módulos que estiverem ligados no momento em que o mundo é criado. Se o
  jogador deixar só um addon ligado, esse é o padrão; se deixar dois, são esses dois. Não há padrão fixo.

## 4. Mundos que precisam de um módulo desligado (o ponto mais delicado)

Um mundo "precisa" de um módulo quando contém blocos ou itens dele. Regras:

1. **Entrar é bloqueado.** Se o jogador tenta abrir um mundo que precisa de um módulo desligado, o
   jogo não entra. Mostra a mensagem dizendo quais módulos estão desligados e que por isso não é
   possível entrar, com um botão para voltar à configuração.
2. Nada é apagado do mundo. Os dados dos módulos desligados continuam no arquivo do mundo, e o
   mundo abre normalmente quando o módulo é ligado de novo.
3. Mundos que não usam o módulo desligado abrem sem aviso.
4. Ao criar um mundo, o jogo usa os módulos ligados naquele momento (seção 3).

Detalhe a confirmar: a checagem é feita pelo que está salvo no mundo (blocos e itens) ou por uma
lista de módulos gravada no próprio mundo quando ele foi criado. A primeira é mais precisa; a
segunda é mais barata de checar ao abrir.

Esse é o ponto que mais precisa de teste antes de qualquer release.

## 5. Como o Horizon entra

Hoje o Horizon separa dois tipos de mod:
- **Mod de conteúdo** (`.hmod`): dados e recursos, sem código. Serve para blocos, itens, receitas
  e texturas, mas não para máquinas com lógica própria.
- **Mod nativo** (`.hnmod`): código Rust compilado pelo launcher, com ganchos (hooks) no jogo.

O Mekanism precisa de lógica de máquina, energia, fluidos e telas. Isso exige mod nativo. As partes
de dados (receitas, modelos, textos) podem ir para um mod de conteúdo, se for mais simples.

Ganchos que o upstream já oferece e que este projeto usaria:
- energia e fluidos de máquina (`machines/native_energy.rs`, `machines/native_fluids.rs`)
- entidades de bloco com estado salvo e tique no servidor
- telas próprias de bloco (botões, dicas, slots)
- receitas de tipos próprios, em JSON
- configurações de mod (botões e teclas na página do mod)

Níveis (tiers) do Evolved: no original, o Evolved altera o código do Mekanism por mixin para acrescentar
níveis. O Horizon não tem mixin para isso. Por isso o núcleo precisa ser desenhado desde o início para
**aceitar níveis extras registrados por módulos**. O Evolved continua sendo módulo (liga e desliga);
desligado, simplesmente não registra seus níveis. Este ponto é um requisito de desenho; eu não li o código
dos níveis do Mekanism para confirmar como isso se encaixa.

Lacunas que eu não confirmei no código e que precisam de checagem:
- **Gases e químicos** (base de vários recursos do Mekanism). Não achei suporte específico.
- **Multiblocos** (estruturas de vários blocos). Não achei suporte específico.
- **Cabos e redes de energia** no estilo do Mekanism.
- **Geração de minérios** em escala.

## 6. Licença e dependência externa

- Todos os addons listados são **MIT**, segundo as páginas do CurseForge. Isso foi lido por resumo
  automático das páginas; confirmar nas páginas e no repositório de cada um antes de copiar código.
- MIT permite juntar o código, desde que o aviso de copyright e a licença de cada autor fiquem no
  código.
- **iglee-library** (iglee42): o projeto **não depende dela e não a inclui**. A análise do jar do
  Evolved Mekanism 1.3.0 (1.21.1) mostrou que ele usa uma só função dela, `ModsUtils.getUpperName`
  (transforma `some_name` em `SomeName`), e nenhum material, bloco, item ou dado. Essa função é
  reescrita dentro do projeto, a partir do comportamento. Os créditos citam a iglee-library e o
  iglee42 por ela ser usada pelo Evolved original. A página dela proíbe repostar o mod em outros
  sites; isso não se aplica, porque nada dela é redistribuído.

## 7. Fases (proposta)

1. **Pré-requisito:** o Horizon ter os recursos da seção 5 confirmados ou implementados.
2. **Fatia 1:** núcleo com uma máquina simples (energia, uma receita, uma tela), e a tela de
   configuração com os interruptores. Sem addons.
3. **Fatia 2:** a regra de mundos salvos da seção 4, com testes automatizados e um mundo de teste.
4. **Fatia 3:** os módulos, um por vez, começando pelos mais simples (Tools, depois Generators).
5. **Fatia 4:** Evolved, por último, porque acrescenta níveis (tiers) ao núcleo.

## 8. Testes

- Teste de ligar e desligar cada módulo e reiniciar.
- Teste de mundo salvo com cada módulo desligado: o bloqueio de entrada (seção 4) e o mundo que não
  usa o módulo, que deve abrir sem aviso.
- Teste do reinício automático: o aviso aparece, o jogo fecha e reabre com a configuração nova.
- Teste do Evolved ligado e desligado: os níveis dele aparecem e somem sem afetar o núcleo.
- Teste de conflito: dois módulos que mexem na mesma receita.

## 9. Perguntas em aberto

1. Nome do projeto: **Mekanism Rust** ou **Mekanism Horizon**? (Escolha ainda não feita.)
2. Reinício: o jogo deve fechar e reabrir sozinho, ou só fechar? O desenho assume reabrir sozinho.

Respostas já dadas:
- Padrão de mundo novo: os módulos ligados no momento da criação (seção 3).
- Publicação: pública.
- iglee42: será contatado quando o desenho sair do papel e começar o código.
- Bloqueio de entrada: confirmado (seção 4).
