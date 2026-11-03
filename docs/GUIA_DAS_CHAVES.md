# Guia das chaves do tweaks.cfg

*English version: [GUIA_DAS_CHAVES.en.md](GUIA_DAS_CHAVES.en.md). Gerado por `tweaks-config.exe --guia pt` dos mesmos textos da ajuda do configurador; não editar à mão.*

Cada opção: o que faz, quando usar e o que acontece no jogo. Sem a linha no `tweaks.cfg`, vale o padrão indicado (o da DLL). O comentário de cada linha do `tweaks.cfg` traz o detalhe técnico.

## Idades

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

### Mais novos: anos a menos (0 = desligado) — `younger_by`

Quantos anos a menos para os regens de "a partir de" para cima. 0 desliga. Mais anos a menos = mais jovens de 16 a 20 e menos veteranos recém-gerados.

Valores: de 0 a 126 (inteiro). Padrão: `0`.

### Nações fora do jogo — `ages_out_of_game`

As nações fora do jogo com poucas pessoas no banco sorteiam idades de um jeito próprio (quase sempre 23 a 27). "A tabela também" faz essas nações usarem a sua tabela. Só muda nações cuja liga você não escolheu.

Valores: `original` (Sorteio original), `table` (A tabela também). Padrão: `original`.

### Pular o teto do CA (%) — `ca_cap_skip`

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

### Alvo: peso do CA (0 = PA, 1 = CA) — `target_ca`

Quanto o alvo olha o CA (o que o jovem já sabe) em vez do PA (o que pode vir a ser). 0 = só o PA; 1 = só o CA.

Valores: de … a …. Padrão: `0`.

### Largura acima do alvo — `sigma`

Quanto um clube acima do alvo ainda tem chance. Maior = os jovens se espalham por clubes de níveis mais variados; menor = vão para clubes bem do tamanho deles.

Valores: de 1 a …. Padrão: `2500`.

### Largura abaixo do alvo (0 = igual) — `sigma_below`

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

### Elenco: expoente da vaga — `squad_exp`

O quanto a vaga no elenco pesa: maior = clubes com elenco curto têm muito mais chance que os de elenco quase cheio.

Valores: de … a …. Padrão: `0.5`.

### Teto por clube a cada 365 dias (0 = sem) — `season_cap`

Quantos jovens um clube recebe no máximo a cada 365 dias pela disputa (0 = sem teto). É o que impede um clube de recolher dezenas de regens. Com 3, os elencos das ligas jogadas ficam um pouco mais cheios.

Valores: de … a …. Padrão: `2`.

### Surpresa (%) — `surprise`

Percentagem dos jovens sorteados sem olhar o nível do clube, só entre clubes das competições: o craque que surge num clube pequeno, ou o comum num grande. 0 = nunca.

Valores: de 0 a 100. Padrão: `15`.

### Clubes fora das competições (peso) — `outside_competitions`

O peso dos clubes cuja divisão não roda na sua carreira (nem como liga de fundo). 1 = iguais aos outros; 0,25 = um quarto da chance; 0 = nunca (sem clube de competição com vaga, o jovem fica sem clube). Valores baixos mantêm cheios os elencos das ligas que você joga; com poucas ligas carregadas, 1 pode servir melhor.

Valores: de 0 a 1. Padrão: `1`.

### Temperatura — `temperature`

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

### Proteger os menores de (0 = não) — `protect_young_unemployed`

Abaixo desta idade, nenhum jogador sem clube se aposenta pela regra dos inativos. 0 = sem proteção (o padrão: o jogo já dá 365 dias ao jovem sem clube). Proteger os jovens diminui a reciclagem em regens.

Valores: de 0 a 126 (inteiro). Padrão: `0`.

## De quem é cada regen

De quem é cada regen. O jogo não guarda isso em lugar nenhum: com a opção ligada, a DLL anota cada regen que nasce, de quem ele herdou a vaga e de quem herdou a ficha de jogador, num arquivo regen_origins.csv ao lado do exe. Aqui você busca nesse histórico.

- **Vaga**: a pessoa aposentada que o jogo reaproveita para o regen: dela vêm o id e a personalidade. O nome, a idade e a nação mudam.
- **Ficha de jogador**: a ficha de outro aposentado, guardada numa reserva: dela vêm o PA, as posições, os pés e os atributos. É o "de quem é" o regen: o regen do Ronaldo é quem recebe a ficha do Ronaldo.
- **regen_origins.csv**: o histórico, ao lado do exe: uma linha por regen, separada por ponto e vírgula. Abre no Excel com dois cliques e no Bloco de Notas. Para compartilhar, mande o arquivo.
- **X.regens**: ao gravar o jogo, a DLL grava ao lado do save (X.sav) os donos das fichas que ainda estão na reserva, e o lê ao carregar. Copie o save e o .regens juntos.

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

## tweaks.log

O tweaks.log é o diário da DLL: o exe que ela encontrou, o que cada ajuste fez e, com o registro ligado, cada jovem e o clube dele. É o arquivo para mandar quando algo der errado.

### Registro (tweaks.log) — `log`

Grava o tweaks.log ao lado do exe: o que cada ajuste fez, cada jovem e o clube, as caixas de erro e, todo julho, o retrato da população. Para testar e para mandar quando algo der errado. Desligado no uso normal.

Valores: `1` liga, `0` desliga. Padrão: `0`.

