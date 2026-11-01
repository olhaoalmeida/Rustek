# Mekanism Rust: desenho (rascunho)

Nome do projeto: **Mekanism Rust**.

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

1. Reinício: o jogo deve fechar e reabrir sozinho, ou só fechar? O desenho assume reabrir sozinho.

Respostas já dadas:
- Padrão de mundo novo: os módulos ligados no momento da criação (seção 3).
- Nome: Mekanism Rust.
- Publicação: pública.
- iglee42: será contatado quando o desenho sair do papel e começar o código.
- Bloqueio de entrada: confirmado (seção 4).

## 10. Plano para a IA que vai programar

Este plano vale para qualquer IA ou pessoa que escreva código neste repositório. Os mesmos passos
estão em `AGENTS.md`, que é a versão curta para ferramentas.

### 10.1 Regras de trabalho

1. **Não achar nada.** Se o código, a documentação ou o mod original não respondem, a IA pergunta ao
   dono do projeto antes de seguir. Vale também para licença e para nomes de blocos e itens.
2. **Ler antes de escrever.** Para cada tarefa, ler o código-fonte do Horizon que ela toca e o
   comportamento do mod Java original (o jar ou o código publicado). Anotar o que foi lido.
3. **O Mekanism original é a especificação.** Valores, receitas, tempos e consumo de energia vêm do
   original. Onde o original tem um número, o port usa o mesmo número; onde difere, vai para
   `DIFERENCAS.md` com o motivo.
4. **Passos pequenos.** Uma tarefa por vez, um commit por tarefa, sem misturar assunto. Nunca em
   `main` direto: uma branch por fatia e um pull request.
5. **Licença sempre.** Código ou dado vindo de um mod MIT leva o aviso de copyright no arquivo ou em
   `NOTICE.md`. Nunca incluir jar, textura ou som de outro mod sem confirmar a licença do arquivo.
6. **Nada de dependência externa** além do Horizon (ver seção 6).
7. **Marcar o que é IA.** Commits com a linha `Co-Authored-By` do modelo usado.

### 10.2 Como cada tarefa termina

Uma tarefa só está pronta quando:
- o código compila e passa no formatador e no linter do Horizon;
- há um teste que verifica **comportamento observável** (por exemplo: a máquina converte o minério em
  pó com o tempo e a energia do original), com valor esperado vindo do original e não do próprio código;
- o dono rodou o jogo e viu funcionar, quando a tarefa tem parte visual (tela, textura, modelo);
- `PROGRESSO.md` registra o que foi feito, o que foi lido e o que ficou pendente.

A IA não declara "pronto" sem os dois primeiros itens e deve dizer claramente quando não pôde
testar algo (por exemplo, falta de placa de vídeo ou dos dados do Minecraft).

### 10.3 Tarefas por fatia (ver seção 7)

- **Fatia 0, antes de qualquer código:** verificar no código do Horizon o que existe para energia,
  fluidos, telas e receitas de mod nativo, e o que falta (gases, multiblocos, cabos). Entregável:
  uma tabela "existe / falta / como contornar", com o arquivo e a linha de cada item.
- **Fatia 1:** estrutura do mod nativo, tela de configurações com os interruptores, leitura do arquivo
  de configuração, reinício automático, uma máquina simples.
- **Fatia 2:** a lista de módulos gravada no mundo e o bloqueio de entrada (seção 4), com testes.
- **Fatia 3:** módulos, um por vez, do mais simples ao mais complexo.
- **Fatia 4:** Evolved (níveis extras registrados por módulo).

### 10.4 Quem faz cada parte

Modelos disponíveis ao dono: **Claude Sonnet 5.5**, **GPT-5.6 Terra** e **GPT-6 Luna**. O GPT-6 Sol e o
GPT-6 Astra não estão disponíveis e não entram no plano.

Aviso: esta divisão vem de **descrições públicas dos modelos**, não de um teste neste projeto. O
Luna é descrito como o de menor custo, para trabalho focado e de alto volume; o Terra, como faixa
intermediária que equilibra custo e capacidade; o Sonnet 5.5, como modelo eficiente para uso diário.
Quem escreveu este plano é o Claude, então o Sonnet 5.5 tem interesse nele. A divisão é provisória
até a calibração abaixo.

Regras de uso:
- **Quem escreve uma fatia não a revisa.** O revisor é outro modelo da lista.
- Dois modelos nunca escrevem na mesma fatia ao mesmo tempo.
- Todos recebem o mesmo contexto: `DESENHO.md` e `AGENTS.md`.

| Parte | Quem escreve | Quem revisa | Por quê |
|---|---|---|---|
| Fatia 0: tabela "existe / falta / como contornar" do Horizon | Sonnet 5.5 | Terra | Exige ler muito código com cuidado e registrar arquivo e linha, sem supor |
| Fatia 1: estrutura do mod, tela de configurações, config, reinício, primeira máquina | Sonnet 5.5 | Terra | Define a base que as outras partes usam |
| Fatia 2: lista de módulos no mundo e bloqueio de entrada | Terra | Sonnet 5.5 | Lógica delimitada, com muitos testes |
| Fatia 3: máquinas e telas de cada módulo | Terra | Sonnet 5.5 | Trabalho repetido em volume médio |
| Fatia 3: conversão em massa (receitas, textos, tabelas, modelos JSON) | Luna | Terra | Tarefa mecânica; o resultado é conferido por script e por teste |
| Fatia 4: níveis extras do Evolved | Sonnet 5.5 | Terra | Mexe no desenho do núcleo |
| Créditos, `NOTICE.md` e licenças | Sonnet 5.5 escreve | o dono confere | Decisão de licença é do dono |

Calibração (antes da Fatia 1): dar a **mesma tarefa pequena e real** aos três (por exemplo, uma máquina
simples com receita, energia e tela), rodar os testes e comparar resultado e custo. Se a ordem real for
diferente da tabela, a tabela muda. A tarefa e o resultado ficam registrados em `PROGRESSO.md`.

### 10.5 O que a IA nunca faz sozinha

- Publicar o mod, abrir pull request em repositório de outra pessoa ou falar com autores em nome do dono.
- Apagar arquivos do dono ou mexer em dados de jogador (mundos, configurações reais).
- Copiar código de outro mod sem registrar origem e licença.
- Afirmar que algo funciona sem ter testado.
