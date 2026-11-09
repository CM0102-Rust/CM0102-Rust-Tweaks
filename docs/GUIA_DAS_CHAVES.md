# Guia das chaves do tweaks.cfg

*English version: [GUIA_DAS_CHAVES.en.md](GUIA_DAS_CHAVES.en.md). Gerado por `tweaks-config.exe --guia pt` dos mesmos textos da ajuda do configurador; não editar à mão.*

Cada opção: o que faz, quando usar e o que acontece no jogo. Sem a linha no `tweaks.cfg`, vale o padrão indicado (o da DLL). O comentário de cada linha do `tweaks.cfg` traz o detalhe técnico.

## Fornada de regens

A fornada de regens (o Youth Intake do Football Manager; hornada no espanhol). Com ela ligada, você joga de outro jeito: os regens chegam todos de uma vez, num dia do ano, e todo clube com vaga recebe o mesmo número deles. Regen é regen: venha de um aposentado ou criado pela DLL, todos nascem pela rotina do jogo e vão para os clubes pela curva da aba Disputa dos clubes. O máximo por ano e o teto do elenco da aba Disputa dos clubes deixam de valer, porque a cota da fornada é o limite. O seu clube recebe uma mensagem do treinador da base. Só vale numa carreira NOVA.

- **Regens criados**: quando as aposentadorias não bastam para a cota de todos os clubes, a DLL cria os que faltam: a pessoa vem de uma reserva criada junto com a carreira nova (até 50 mil, conforme a folga do banco), e a ficha é a mistura de duas fichas de aposentados do país na mesma posição (as posições e os pés de uma, cada atributo de uma das duas, o PA entre os dois). Depois, nascem pela mesma rotina do jogo que qualquer regen.
- **Cota**: quantos regens um clube recebe na fornada: o número escolhido, limitado às vagas do elenco (50) no dia.
- **Fim da temporada**: o dia em que o jogo fecha a temporada de cada nação (a mesma data das finanças e das obras do estádio). Na Inglaterra é perto de junho; no Brasil, de dezembro.

### Fornada — `intake`

Liga a fornada, o "Youth Intake" do Football Manager. Regen é regen: a fornada muda só duas coisas. A data: os regens só nascem no dia da fornada (quem se aposenta espera na fila do jogo até lá). A quantidade: todo clube com vaga no elenco recebe o número escolhido abaixo. Para qual clube vai cada regen, quem decide é a curva da aba Disputa dos clubes (nível, instalações, comissão, surpresa, temperatura); o máximo por ano e o teto do elenco da aba Disputa dos clubes não valem, porque a cota da fornada é o limite. Quando as aposentadorias não bastam para todos, a DLL cria os regens que faltam, com pessoas criadas junto com a carreira e a ficha misturada de dois aposentados do país na mesma posição. Só numa carreira nova.

Valores: `0` (Desligada (regens ao longo da temporada)), `1` (Ligada (só numa carreira nova)). Padrão: `0`.

### Regens por clube em cada fornada — `intake_per_club`

Quantos regens cada clube recebe em cada fornada, limitado às vagas do elenco (50): com 2 vagas, chegam 2; sem vaga, nenhum, e o seu clube recebe uma mensagem dizendo isso. A vaga é contada no próprio dia: dá para liberar jogadores até a véspera. O seu clube recebe pela mesma regra. Os regens de um país são distribuídos do maior PA para o menor, cada um sorteado pela curva entre os clubes que ainda têm vaga na cota; os que sobram vão para a disputa normal do país.

Valores: de 1 a 20 (inteiro). Padrão: `3`.

### Dia da fornada — `intake_day`

O dia em que os regens chegam. No fim da temporada (o padrão), cada país quando a temporada dele acaba (o Brasil no começo do ano, a Inglaterra e a Itália no meio); os países fora da lista dos 35 principais do jogo recebem no dia 181. Num dia fixo (contado de 0 = 1º de janeiro; 181 = 1º de julho), todos os países juntos. No tweaks.cfg vale qualquer dia de 0 a 365.

Valores: `season_end` (Fim da temporada de cada nação), `181` (1º de julho (dia 181), para todos), `0` (1º de janeiro (dia 0), para todos). Padrão: `season_end`.

### Clubes que recebem — `intake_clubs`

Quais clubes recebem a cota: os das ligas jogáveis, ou também os das ligas de fundo que você escolheu ao criar a carreira. Os países de fora seguem o caminho original do jogo.

Valores: `playable` (Ligas jogáveis), `selected` (Também as ligas de fundo). Padrão: `playable`.

### Dia das aposentadorias — `retire_day`

O dia do ano das aposentadorias de jogadores. No jogo original, cada um se aposenta numa data própria (em geral o fim da temporada do país). Com um dia (contado de 0 = 1º de janeiro), todas as aposentadorias marcadas a partir de agora acontecem nele; um pouco antes da fornada, quem se aposenta já nasce na fornada seguinte. Vale com ou sem a fornada. No tweaks.cfg vale qualquer dia de 0 a 365.

Valores: `original` (Datas do próprio jogo), `174` (24 de junho (dia 174, uma semana antes do 1º de julho)), `358` (24 de dezembro (dia 358)). Padrão: `original`.

### Mensagem da fornada — `intake_news`

No dia da fornada, o seu clube recebe na caixa de entrada uma mensagem do treinador da base com todos os regens que chegaram e o parecer sobre cada um (ou o aviso de que ninguém pôde chegar por falta de vaga): se já está pronto para o time principal (comparado com o seu elenco) e até onde pode chegar. Sai na língua do jogo. As frases ficam no intake_news.txt, ao lado do exe, uma seção para cada uma das 19 línguas do jogo: dá para editar ou traduzir (português, espanhol, inglês, francês, alemão, italiano, holandês, dinamarquês, norueguês e sueco já vêm traduzidos). Sem a DLL, a mensagem aparece numa linha curta.

Valores: `1` (Mensagem na caixa de entrada), `0` (Sem mensagem). Padrão: `1`.

### Na mensagem — `intake_news_numbers`

Mostrar também o CA e o PA de cada jovem na mensagem. Desligado, fica só o parecer do treinador, e a graça é descobrir o jogador.

Valores: `0` (Só o parecer do treinador), `1` (Parecer, CA e PA). Padrão: `0`.

## Idades dos regens

Com que idade nasce cada regen, e com quanto CA. As barras são as idades de 14 a 35: a altura é a chance de nascer com aquela idade, e o número embaixo é o teto do CA. Clique numa barra para escolhê-la e arraste para mudar a chance. Os ajustes valem para os regens que nascerem dali em diante, também num save em andamento.

- **Regen**: o jogador novo que o jogo cria quando alguém se aposenta. O jogo não cria uma pessoa do zero: reaproveita a vaga de um aposentado e a ficha de jogador de outro.
- **CA**: Current Ability, a habilidade atual (1 a 200): o quanto o jogador joga hoje.
- **PA**: Potential Ability, o potencial (1 a 200): até onde o CA pode chegar com o tempo. O regen herda o PA da ficha de jogador que recebeu.
- **Chance**: o peso de cada idade no sorteio. Não precisa somar 100: o que vale é a proporção entre as idades (a porcentagem em cima de cada barra). 0 tira a idade.
- **Teto do CA**: o CA máximo com que um jovem daquela idade nasce, o número embaixo da barra. Com teto 80 aos 17 anos, nenhum regen de 17 nasce com CA acima de 80, mesmo que o PA seja 180: ele precisa crescer. Sem teto, o CA sai só da fórmula do jogo.
- **age_18 no tweaks.cfg**: é como a idade 18 fica escrita no arquivo: "age_18 = 29.11, 100" quer dizer chance 29,11 e teto do CA 100. Cada barra é uma linha dessas, de age_14 a age_35.
- **Disputa**: a escolha do clube que recebe o jovem (aba Disputa dos clubes). Aqui ficam as duas opções que decidem quem entra nela.

### Idades dos regens — `ages`

Tabela: a idade com que os regens nascem segue a tabela de idades (as barras). Original: o sorteio do jogo (quase todos de 16 a 20). Use a tabela para regens mais novos ou mais velhos. Vale para os regens que nascerem dali em diante, também num save em andamento.

Valores: `original` (Original), `custom` (Tabela). Padrão: `original`.

### Tabela de idades — `age_14` … `age_35`

A chance de um regen nascer com cada idade (as chances não precisam somar 100) e o teto do CA naquela idade: o jovem não começa com CA acima do teto. Arraste a barra para mudar a chance; zero tira a idade.

Formato: `age_NN = chance, teto do CA`.

### Mais novos: a partir de — `younger_from_age`

"Regens mais novos" (ideia do petermf23): os que sairiam com esta idade ou mais nascem mais novos. Funciona junto com "anos a menos".

Valores: de 0 a 126 (inteiro). Padrão: `22`.

### Mais novos: anos a menos — `younger_by`

Quantos anos a menos para os regens de "a partir de" para cima. 0 desliga. Mais anos a menos = mais jovens de 16 a 20 e menos veteranos recém-gerados.

Valores: de 0 a 126 (inteiro). Padrão: `0`.

### Nações fora do jogo — `ages_out_of_game`

As nações fora do jogo com poucas pessoas no banco sorteiam idades de um jeito próprio (quase sempre 23 a 27). "A tabela também" faz essas nações usarem a sua tabela. Só muda nações cuja liga você não escolheu.

Valores: `original` (Sorteio original), `table` (A tabela também). Padrão: `original`.

### Jovens sem o teto do CA (%) — `ca_cap_skip`

Chance (%) de um jovem ignorar o teto do CA da idade dele: alguns nascem já prontos, como no jogo original (cerca de 1 em 200). Mais alto = mais jovens prontos cedo.

Valores: de 0 a 100. Padrão: `0.5`.

### Idade máxima da disputa — `contest_max_age`

Até que idade o jovem vai à disputa dos clubes no jogo original (os mais velhos começam sem clube). Sem efeito com "Sem clube só sem vaga", que manda todos à disputa.

Valores: de 0 a 126 (inteiro). Padrão: `18`.

### Quem vai à disputa — `unattached`

Sem clube só sem vaga: todo jovem de nação no jogo vai à disputa e só fica sem clube se nenhum clube do país tiver vaga. Original: só os de até 18 anos vão, e 1 em 4 é descartado (muitos começam sem clube e se perdem). Recomendado: menos talentos desperdiçados.

Valores: `original` (Original), `only_if_no_room` (Sem clube só sem vaga). Padrão: `original`.

## Disputa dos clubes

Para qual clube vai o regen jovem. Quando nasce um jovem de uma nação no jogo, os clubes do país dele disputam o jogador. Aqui você escolhe a regra dessa disputa e regula o sorteio. O gráfico e a lista de chances usam um jovem de exemplo e clubes de exemplo: mexa no PA e no CA dele para ver para onde a regra o mandaria.

- **Original**: a regra do 3.9.68: os clubes de maior reputação e melhores instalações ganham quase sempre. Um grande chega a juntar mais de 10 regens numa temporada, e os pequenos quase nunca recebem.
- **Sorteio proporcional**: a regra desta DLL: cada clube do país ganha um peso, e o clube é sorteado na proporção do peso. Um clube com o dobro do peso tem o dobro da chance; ninguém tem garantia e ninguém fica excluído. O peso é nível × instalações × comissão × vaga.
- **Reputação do clube**: o tamanho do clube no jogo, de 0 a 10000 (um gigante europeu passa de 9000; um clube de bairro fica perto de 1000).
- **Alvo**: a reputação de clube que combina com o jovem: o PA dele vezes a regra do alvo (com 50, um PA 150 mira um clube de 7500). A linha amarela do gráfico.
- **Nível**: o quanto o clube está perto do alvo, de 0 a 1: a curva em sino do gráfico. No alvo vale 1; longe dele, cai.
- **Jovem de exemplo**: um jovem imaginário, só para os gráficos: mexa no PA e no CA dele para ver para onde a regra mandaria um jovem assim. Não muda o jogo nem o tweaks.cfg.
- **Clubes de exemplo**: clubes imaginários de vários tamanhos (os pontos azuis do gráfico e a lista de chances), para comparar. No jogo entram os clubes de verdade do país do jovem.
- **Surpresa**: uma parte dos jovens é sorteada sem olhar o nível do clube: o craque que surge num clube pequeno, como o Ronaldo no São Cristóvão.

### A disputa dos clubes — `rule`

Sorteio proporcional: o jovem vai para um clube do tamanho dele, sorteado pelo nível, instalações, comissão e vaga, com no máximo 2 jovens por clube por ano. Original: os grandes recolhem quase todos (até 14 numa temporada). Recomendado para carreiras equilibradas.

Valores: `original` (Original), `new` (Sorteio proporcional). Padrão: `original`.

### Alvo: reputação por ponto de PA — `target_k`

Que reputação de clube combina com o jovem: o alvo é este valor vezes o PA (50 × PA 150 = clube de 7500). Mais alto = jovens bons indo para clubes maiores.

Valores: de … a …. Padrão: `50`.

### Alvo: potencial ou o que já sabe — `target_ca`

Quanto o alvo olha o CA (o que o jovem já sabe) em vez do PA (o que pode vir a ser). 0 = só o PA; 1 = só o CA.

Valores: de … a …. Padrão: `0`.

### Largura acima do alvo — `sigma`

Quanto um clube acima do alvo ainda tem chance. Maior = os jovens se espalham por clubes de níveis mais variados; menor = vão para clubes bem do tamanho deles.

Valores: de 1 a …. Padrão: `2500`.

### Largura abaixo do alvo — `sigma_below`

O mesmo, para os clubes abaixo do alvo (0 = igual à largura de cima). Mantido menor porque as centenas de clubes pequenos de um país, somadas, levariam os jovens bons.

Valores: de 0 a …. Padrão: `1200`.

### Piso do nível — `level_floor`

A chance mínima de um clube longe do alvo (0 a 1). Acima de 0, um clube pequeno ainda pode, raramente, receber um craque.

Valores: de … a …. Padrão: `0.002`.

### Instalações: base — `facilities_base`

Quanto as instalações de base (Training) pesam: o peso é esta base mais o valor por ponto. Clubes com boa base atraem mais jovens.

Valores: de … a …. Padrão: `0.4`.

### Instalações: por ponto de Training — `facilities_k`

Quanto cada ponto de Training soma ao peso (com 0,4 e 0,06: Training 1 = 0,46; 10 = 1,00; 20 = 1,60).

Valores: de … a …. Padrão: `0.06`.

### Comissão: o pior — `staff_min`

O peso do clube cuja comissão trabalha pior com jovens ("Working with Youngsters" 1).

Valores: de … a …. Padrão: `0.8`.

### Comissão: o melhor — `staff_max`

O peso do clube cuja comissão trabalha melhor com jovens ("Working with Youngsters" 20). Deve ser maior que o do pior.

Valores: de … a …. Padrão: `1.2`.

### Elenco: teto — `squad_limit`

Clube com este número de jogadores ou mais não recebe jovem. Mais baixo = os jovens vão para clubes com elenco menor.

Valores: de 1 a 50. Padrão: `48`.

### Quanto pesa a vaga no elenco — `squad_exp`

O quanto a vaga no elenco pesa: maior = clubes com elenco curto têm muito mais chance que os de elenco quase cheio.

Valores: de … a …. Padrão: `0.5`.

### Máximo de jovens por clube por ano — `season_cap`

Quantos jovens um clube recebe no máximo a cada 365 dias pela disputa (0 = sem teto). É o que impede um clube de recolher dezenas de regens. Com 3, os elencos das ligas jogadas ficam um pouco mais cheios.

Valores: de … a …. Padrão: `2`.

### Surpresa (%) — `surprise`

Percentagem dos jovens sorteados sem olhar o nível do clube, só entre clubes das competições: o craque que surge num clube pequeno, ou o comum num grande. 0 = nunca.

Valores: de 0 a 100. Padrão: `15`.

### Clubes de divisões que não rodam — `outside_competitions`

O peso dos clubes cuja divisão não roda na sua carreira (nem como liga de fundo). 1 = iguais aos outros; 0,25 = um quarto da chance; 0 = nunca (sem clube de competição com vaga, o jovem fica sem clube). Valores baixos mantêm cheios os elencos das ligas que você joga; com poucas ligas carregadas, 1 pode servir melhor.

Valores: de 0 a 1. Padrão: `1`.

### Diferença entre os clubes (temperatura) — `temperature`

Acentua (maior que 1) ou suaviza (menor que 1) as diferenças de peso entre os clubes. 1 = o peso como calculado.

Valores: de … a …. Padrão: `1`.

## Aposentadoria

A aposentadoria dos jogadores sem clube que ninguém quer. Todo regen nasce da ficha de um aposentado: aposentar mais cedo o veterano parado faz nascer mais jovens, sem mudar o total de pessoas nem o formato do save. O gráfico mostra a chance de aposentar por idade, para um jogador de exemplo.

- **Inativo**: jogador sem clube há muito tempo, que nenhum clube contrata.
- **Verificação**: o jogo testa as aposentadorias quatro vezes por ano (dias 0, 80, 173 e 250). A chance do gráfico é por verificação.
- **Reputação**: a fama do jogador, de 0 a 10000. Quem tem reputação baixa se aposenta mais fácil.
- **Reputação de exemplo**: um jogador imaginário, só para o gráfico: mude a reputação para ver a chance de aposentar por idade. Não muda o jogo.
- **Regra original**: a linha tracejada: no 3.9.68, depois de 365 dias sem clube, a chance é 750 / reputação, em qualquer idade.

### Aposentar os inativos — `retire_idle_players`

O jogador sem clube que ninguém quer se aposenta mais cedo, e a ficha dele vira um regen semanas depois. Resultado: mais jovens e menos veteranos parados no mercado. Num pacote que barra aposentadorias (o RetirePatchV4 do NickPatcher), quase não age.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Dias sem clube — `idle_days`

Quantos dias sem clube antes de a regra dos inativos valer. Menos dias = aposentadorias mais rápidas.

Valores: de 0 a … (inteiro). Padrão: `180`.

### Idade mínima — `idle_min_age`

A partir de que idade a regra dos inativos vale. Abaixo dela, fica a regra original do jogo.

Valores: de 0 a 126 (inteiro). Padrão: `28`.

### Reputação-limite — `idle_rep`

A reputação até a qual o inativo se aposenta: quem tem reputação menor sai quase sempre; quem tem mais, raramente. Mais alto = mais aposentadorias.

Valores: de 0 a …. Padrão: `1500`.

### A mais por ano de idade — `idle_rep_per_year`

Quanto a reputação-limite sobe por ano acima da idade mínima: os mais velhos se aposentam mais fácil.

Valores: de 0 a …. Padrão: `250`.

### Proteger os jovens: menores de — `protect_young_unemployed`

Abaixo desta idade, nenhum jogador sem clube se aposenta pela regra dos inativos. 0 = sem proteção (o padrão: o jogo já dá 365 dias ao jovem sem clube). Proteger os jovens diminui a reciclagem em regens.

Valores: de 0 a 126 (inteiro). Padrão: `0`.

## Nomes dos regens

Os nomes dos regens. Os nomes vêm do banco de dados (first_names.dat e second_names.dat, cada nome com a nação e o quanto é comum). A regra de quando o jogo usa os nomes do próprio país está no exe: só acima de 100 nomes. Com menos, ele pega emprestado de outro país, e o regen de Serra Leoa pode sair com nome de outro lugar. Aqui o país usa os próprios nomes a partir de um mínimo.

- **Pegar emprestado**: quando o país tem 100 nomes ou menos no banco, o jogo não usa os dele: tira o nome de um país de referência da mesma língua (por exemplo, da Espanha para os de língua espanhola). Alguns países não têm país de referência: o jogo sorteia 40 vezes entre todos os países atrás de um da mesma língua e, se não achar, fica com um país qualquer. A lista abaixo mostra, país por país, de onde vêm hoje.
- **Pessoas novas**: os regens, e também a comissão e os árbitros que o jogo cria: todos passam pela mesma regra de nomes.

### Os nomes das pessoas novas — `regen_names`

Os da própria nação: os regens (e a comissão e os árbitros novos) ganham nomes do próprio país sempre que o banco tiver nomes suficientes dele (o mínimo abaixo). Original: o jogo só usa os nomes do país quando há mais de 100; com menos, pega emprestado de outro país da mesma língua ou de qualquer país do mundo, e daí vêm os regens com nomes de outras nações. A aba mostra, país por país do seu banco, de onde vêm os nomes hoje e o que muda. Vale para as pessoas criadas dali em diante, também num save em andamento. Os nomes vêm do banco: nada é acrescentado ao save.

Valores: `original` (Original (só acima de 100 nomes)), `own` (Os da própria nação). Padrão: `original`.

### Nomes próprios a partir de — `regen_names_min`

Quantos nomes do país o banco precisa ter para ele usar os próprios. Quanto menor, mais países usam os próprios nomes; a lista da aba mostra quais. Abaixo do mínimo fica o empréstimo do original. Vale separado para o primeiro nome e o sobrenome.

Valores: de 1 a 101 (inteiro). Padrão: `20`.

## Jogadores cinzentos

Os jogadores cinzentos. Antes de cada partida (e quando você abre um elenco), o jogo completa os elencos curtos com jogadores virtuais, que aparecem em cinza e ninguém pode contratar. O original dá a eles uma habilidade tirada da reputação do clube, por uma conta que despenca: nos clubes médios e pequenos ela sai negativa, e o motor da partida trata isso como o pior jogador possível. Aqui eles podem vir no nível do próprio clube.

- **Jogador cinzento**: um jogador virtual que o jogo cria só para completar o elenco: aparece em cinza, não tem contrato e não pode ser contratado. Some quando o clube volta a ter jogadores.
- **CA**: Current Ability, a habilidade atual (1 a 200): o quanto o jogador joga hoje.
- **Nível do clube**: o maior entre dois valores: a média de CA dos 16 melhores jogadores de verdade do elenco (com pelo menos 5), e uma conta pela reputação que sobe com ela e nunca fica negativa (reputação × 0,016 + 20; seleção, reputação / 100 + 40). A reputação segura o clube grande de uma liga que você não carregou, que tem poucos jogadores no banco.
- **Variação**: quanto o CA de cada cinzento pode ficar acima ou abaixo do nível do clube, sorteado para cada um.

### O nível dos cinzentos — `grey_players`

No nível do clube: cada jogador cinzento (os virtuais que o jogo põe para completar o elenco antes da partida) tem o nível do próprio clube, com a variação escolhida abaixo. Original: a conta do jogo, que tira o nível da reputação e despenca nos clubes médios e pequenos (numa carreira de teste, 63% dos cinzentos saíram com CA negativo, o pior jogador possível). O mesmo clube continua recebendo os mesmos cinzentos: só o CA muda. Vale a partir da próxima partida, também num save em andamento.

Valores: `original` (Original (a conta do jogo)), `squad` (No nível do clube). Padrão: `original`.

### Variação em volta do nível — `grey_players_spread`

Quanto o CA de cada cinzento pode ficar acima ou abaixo do nível do clube. Com ± 5, num clube de nível 82 os cinzentos vêm de 77 a 87. ± 0 deixa todos iguais ao nível; mais variação, cinzentos mais diferentes entre si. No tweaks.cfg fica a largura total (± 5 = 10).

Valores: de 0 a 100 (inteiro). Padrão: `10`.

## Público e estádio

Quanta gente vai ao estádio e quando o estádio cresce. O público é a torcida × o momento × o jogo: cresce pelo que acontece em campo e esfria quando o time volta ao esperado. A parte de cima é o público de cada jogo e da temporada; a de baixo, as obras do estádio. Cada parte tem o seu interruptor: desligado, aquela parte fica como no jogo original. O clube de exemplo no fim mostra o que os ajustes fazem com números.

- **Média de público**: quantas pessoas o clube leva por jogo, em média. É a base de cada jogo e muda no fim de cada temporada.
- **Esperado**: a posição que o próprio jogo prevê para cada clube no começo da temporada, pela força dos elencos. Ir melhor ou pior que o previsto é o que move o público.
- **Recorde**: o teto do público de um jogo comum. Clássicos, mata-matas e jogos grandes podem passar dele.
- **Piso do original**: a média que o clube teria no jogo original, calculada temporada a temporada. Quem vai mal perde o que ganhou pela nossa regra, mas nunca fica abaixo dela. Ela é guardada num arquivo ao lado do save (.publico).
- **Demanda**: quanta gente quis vir a um jogo, antes de o recorde e o estádio cortarem. É a medida de quem ficou de fora.
- **Ocupação**: o público dividido pela capacidade do estádio. 92% ou mais por duas temporadas deixam o clube apto a ampliar.

### Público — `attendance`

Liga a nossa regra do público: ele cresce com o que acontece em campo e esfria quando o time volta ao esperado. Quem termina onde se previa mantém o público; quem vai melhor cresce; quem vai pior perde só o que ganhou por esta regra, nunca ficando abaixo do que o jogo original daria. Desligado, o público é o do jogo original, qualquer que seja o resto desta parte.

Valores: `off` (Desligado (o jogo original)), `on` (Ligado (a nossa regra)). Padrão: `on`.

### Tetos pela divisão — `attendance_league_caps`

O jogo original segura a média de público de cada clube abaixo de um teto pela divisão, e nas divisões mais fracas corta o público de cada jogo acima de um limite (fica só 20% do que passa). É isso que impede um clube popular de divisão baixa de encher o estádio. Sem os tetos, quem decide é o desempenho.

Valores: `on` (Com os tetos do jogo), `off` (Sem os tetos (a nossa regra)). Padrão: `off`.

### Recorde do clube — `attendance_record`

O recorde do clube é o teto do público de um jogo comum. No original ele é o do banco de dados e só sobe com título ou acesso: o clube que faz uma ótima temporada sem ganhar nada fica preso nele. Com "o maior público que de fato veio", o recorde acompanha os jogos de verdade (clássicos, mata-matas e jogos grandes passam do recorde e o levantam).

Valores: `fixed` (O do banco (só título ou acesso o sobem)), `real` (O maior público que de fato veio). Padrão: `real`.

### Gente que ficou de fora vira recorde — `pent_up_demand`

Quando mais gente quis vir do que o recorde deixou, parte dessa demanda vira recorde novo. É assim que um clube em ascensão vai abrindo espaço jogo a jogo, sem crescer do nada: só cresce quem tem gente ficando de fora.

Valores: de 0 a 1. Padrão: `0.1`.

### Peso da tradição — `tradition_weight`

O recorde histórico do banco mostra a torcida que o clube já teve. Quando um clube de tradição volta a ir bem (2 posições ou mais acima do previsto), parte dessa torcida volta mais rápido. Um clube sem passado grande quase não muda.

Valores: de 0 a 2. Padrão: `0.25`.

### Teto do mata-mata — `knockout_ceiling`

Num mata-mata (copa nacional, fases finais, continental) o interesse é maior. "Só o estádio" deixa o público ir até a capacidade; com "o recorde do clube", o mata-mata fica preso ao mesmo teto de um jogo comum, como no original.

Valores: `record` (O recorde do clube (como num jogo comum)), `stadium` (Só o estádio). Padrão: `stadium`.

### Jogo grande pela tabela — `title_race`

O jogo de liga entre dois clubes que ainda podem ser campeões leva mais gente, cada vez mais perto do fim da temporada. Se o jogo também for clássico, os dois efeitos se somam. Ninguém precisa configurar rivais: quem decide é a tabela.

Valores: `off` (Desligado), `on` (Ligado). Padrão: `on`.

### Estádio — `stadium`

Liga a nossa regra do estádio: só amplia quem lota de forma sustentada e tem dinheiro, do tamanho que a demanda pede e pagando um preço de verdade. Desligado, o estádio é o do jogo original.

Valores: `off` (Desligado (o jogo original)), `on` (Ligado (a nossa regra)). Padrão: `on`.

### Lugares sentados somam na capacidade — `stadium_seating_growth`

No original, quando um clube não amplia, ele converte lugares em pé em sentados, e o jogo soma os lugares novos também na capacidade: o estádio com espaço em pé cresce todo ano sozinho, mesmo vazio (nas nossas carreiras, o do Bahia passou de 115 mil para 134 mil com o público caindo). "Não cresce sozinho" tira esse crescimento.

Valores: `on` (Cresce sozinho, mesmo vazio (original)), `off` (Não cresce sozinho). Padrão: `off`.

### Ampliar pela reputação — `stadium_reputation_trigger`

No original, um clube de reputação alta amplia o estádio mesmo sem público para isso. "Só por ocupação" deixa a ampliação depender só do estádio cheio.

Valores: `on` (Amplia só pela reputação (original)), `off` (Só por ocupação). Padrão: `off`.

### Temporadas lotado para poder ampliar — `stadium_full_seasons`

Quantas temporadas seguidas o estádio precisa ficar acima de 92% de ocupação para o clube poder ampliar. Com 2, uma temporada boa não basta: é preciso sustentar. Poder ampliar não é ampliar: ainda é preciso dinheiro.

Valores: de 1 a 2 (inteiro). Padrão: `2`.

### O que conta na ocupação — `stadium_occupancy`

O que entra na conta da ocupação. "Todos os jogos em casa" conta também copas e competições continentais: o clube que lota o mata-mata da Libertadores conta isso a favor do estádio novo.

Valores: `league` (Só os jogos da liga (original)), `all` (Todos os jogos em casa). Padrão: `all`.

### Tamanho da obra — `stadium_size`

Quantos lugares a obra acrescenta. "O que a demanda medida pede": a média de gente que quis vir nos jogos em casa, até o teto do estádio. O tamanho fixo do jogo dá 21 mil lugares a quem tem dinheiro, precise ou não.

Valores: `original` (O tamanho fixo do jogo), `demand` (O que a demanda medida pede). Padrão: `demand`.

### Custo por lugar novo — `stadium_seat_cost`

Quanto custa cada lugar novo, no dinheiro do banco que você joga. Automático: 1.500 por lugar no banco de 2001 (uma obra real da época custava de 1.000 a 3.500 libras por lugar), na proporção do caixa dos clubes grandes do banco carregado; num banco com o triplo de dinheiro, a obra custa o triplo. Um número fixa o valor por lugar. O do jogo (uns 100 por lugar) é tão barato que qualquer clube rico amplia sem sentir.

Valores: `original` (O do jogo (uns 100 por lugar)), `auto` (Automático, pelo caixa do banco). Padrão: `auto`.

### Estádio sem teto no banco — `stadium_ceiling_zero`

Cada estádio tem no banco um teto de expansão (o que a cidade comporta), mas a maioria vem com zero. No original, esse teto sobe 1.500 por ano sem critério. "Pela demanda medida": o teto passa a ser a capacidade mais o que a demanda justifica.

Valores: `original` (Sobe 1.500 por ano (original)), `demand` (Pela demanda medida). Padrão: `demand`.

### Maior estádio que ainda amplia — `stadium_max`

Acima desta capacidade, o estádio não amplia mais. O próprio jogo usa 125 mil.

Valores: de 1000 a 10000000 (inteiro). Padrão: `125000`.

## De quem é cada regen

De quem é cada regen. O jogo não guarda isso em lugar nenhum: com a opção ligada, a DLL anota cada regen que nasce, de quem ele herdou a vaga e de quem herdou a ficha de jogador, num arquivo regen_origins.csv ao lado do exe. Aqui você busca nesse histórico.

- **Vaga**: a pessoa aposentada que o jogo reaproveita para o regen: dela vêm o id e a personalidade. O nome, a idade e a nação mudam.
- **Ficha de jogador**: a ficha de outro aposentado, guardada numa reserva: dela vêm o PA, as posições, os pés e os atributos. É o "de quem é" o regen: o regen do Ronaldo é quem recebe a ficha do Ronaldo.
- **regen_origins.csv**: o histórico, ao lado do exe: uma linha por regen, separada por ponto e vírgula. Abre no Excel com dois cliques e no Bloco de Notas. Para compartilhar, mande o arquivo.
- **X.regens**: ao gravar o jogo, a DLL grava ao lado do save (X.sav) os donos das fichas que ainda estão na reserva, e o lê ao carregar. Copie o save e o .regens juntos. Se o .regens não for dessa gravação (o save regravado com o registro desligado, ou um .regens de outro save), a DLL o ignora, para não dar dono errado.

### Registrar de quem é cada regen — `regen_origins`

Grava no regen_origins.csv, ao lado do exe, cada regen que nasce: o nome, a idade, a posição, o PA, o CA e o clube que ganhou a disputa (ou "sem clube"), e de quem ele herdou: a vaga (o id e a personalidade) e a ficha de jogador (o PA, as posições e os atributos), com o nome, a idade, o CA e o PA do dono no momento da aposentadoria. Cada linha leva a data do jogo, a data do computador e a carreira (o save). Assim você acha "o regen do Ronaldo". O jogo não guarda isso em lugar nenhum: só os regens criados com esta opção ligada ficam registrados. O arquivo abre no Excel com dois cliques (UTF-8, colunas separadas por ponto e vírgula; num Excel em inglês, use Dados > De Texto/CSV). Ao gravar o jogo, a DLL grava ao lado do save um X.regens com os donos das fichas que ainda vão virar regen; ao carregar, ela o lê. Copie os dois juntos: sem o .regens, alguns dos primeiros regens depois da carga podem sair com o dono desconhecido.

Valores: `1` liga, `0` desliga. Padrão: `0`.

## Tela, save e pasta

O save, a instalação, a tela e o CD. Estes ajustes não mudam as regras do jogo: só como ele grava, onde procura os arquivos e como desenha.

- **Sobrescrever as mudanças do exe**: quando o exe já tem uma alteração (do NickPatcher, do Regen Tweak) no mesmo ponto de um ajuste, o normal é ficar com a do exe. Ligado, vale a desta DLL, e aquela opção do exe deixa de valer.
- **SetupReg**: o programa que grava no registro do Windows onde o jogo está instalado. Com a pasta portátil, o jogo não precisa dele.

### Save acima de 2 GB — `save_over_2gb`

Permite gravar e carregar saves acima de 2 GB (até 4 GB). Ligue se o seu save é grande (muitas ligas, carreira longa) ou se o jogo dá erro ao carregar. Desligado, um save acima de 2 GB não carrega. Não muda nada na carreira.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Pasta portátil (sem SetupReg) — `portable_folder`

O jogo usa a pasta onde está o exe, sem precisar do SetupReg nem do registro do Windows. Útil para ter várias instalações lado a lado ou uma cópia num pendrive. Nada é gravado no registro.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Abrir mais de um jogo ao mesmo tempo — `multiple_instances`

Deixa abrir mais de um jogo ao mesmo tempo, desde que cada um esteja na sua pasta (por exemplo, o 3.9.68 e o CLZ, ou duas carreiras em duas cópias). O jogo tem uma trava que só deixa uma cópia aberta; com esta opção, a trava vale por pasta. Dois jogos da mesma pasta continuam barrados, porque usariam os mesmos arquivos temporários e o mesmo save. Ligue também a pasta portátil, para cada cópia usar a própria pasta. Cada jogo aberto ocupa a memória dele.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Tirar a marca de administrador da NickDLL — `clear_admin_flag`

Só para quem usa a NickDLL (CLZ e outros pacotes do NickPatcher): ela marca o exe para rodar como administrador a cada abertura. Esta opção tira essa marca (mantém o modo Windows 95). Vale a partir da abertura seguinte. Deixe desligado se você marcou isso de propósito.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Sobrescrever as mudanças do exe — `override_exe_changes`

Quando o seu exe já tem uma modificação no mesmo ponto de um ajuste (o Regen Tweak, as idades do NickPatcher), o normal é ficar com a do exe. Ligado, os ajustes desta DLL substituem essas modificações. Use só se você quer as regras desta DLL no lugar das do seu pacote.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Vídeo — `video`

Janela: o jogo desenha numa janela comum. O Print Screen e os gravadores de tela funcionam, a janela muda de tamanho com escala nítida, e o Windows nunca troca de resolução (sem piscar no Alt+Tab). Original: o DirectDraw do Windows, como sempre.

Valores: `original` (DirectDraw do Windows), `window` (Janela do projeto (Print Screen, escala)). Padrão: `original`.

### Resolução — `resolution`

1920×1080: as telas do jogo em 1920×1080, aplicado em memória (o exe não é alterado). Liga o vídeo em janela junto e evita o erro do relatório de partidas antigas. Se o exe ou a NickDLL já têm resolução própria, fica com a deles.

Valores: `original` (A do exe), `1080` (1920×1080). Padrão: `original`.

### Sem o CD — `no_cd`

O jogo roda sem o CD. Se você já usa um NoCD (a LaMOHatesCD do CLZ, um exe sem CD), fica com ele.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Proteção do elenco (jogador repetido) — `squad_guard`

Corrige um defeito do jogo: ele pode pôr no elenco um jogador que já está lá (visto ao mandar jogadores entre o clube e o time B). As cópias se acumulam até a escalação automática colocar o mesmo jogador em duas posições e mostrar a caixa "match_man.cpp 1225". A proteção apaga a cópia repetida na hora, antes de cada escalação e ao carregar um save (limpa também saves antigos). Cada cópia apagada vai para o tweaks.log.

Valores: `1` liga, `0` desliga. Padrão: `1`.

## tweaks.log

O tweaks.log é o diário da DLL: o exe que ela encontrou, o que cada ajuste fez e, com o registro ligado, cada jovem e o clube dele. É o arquivo para mandar quando algo der errado.

### Registro (tweaks.log) — `log`

Grava o tweaks.log ao lado do exe: o que cada ajuste fez, cada jovem e o clube, as caixas de erro e, todo julho, o retrato da população. Para testar e para mandar quando algo der errado. Desligado no uso normal.

Valores: `1` liga, `0` desliga. Padrão: `0`.

