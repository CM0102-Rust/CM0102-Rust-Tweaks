//! A ajuda prática de cada chave (português, inglês): o que faz, quando usar e o que acontece no jogo. É o texto da dica
//! e do painel da direita do configurador e o do guia `docs/GUIA_DAS_CHAVES(.en).md` (`tweaks-config.exe --guia pt|en`).
//! O comentário do `tweaks.cfg` continua como o detalhe técnico.

pub fn ajuda(chave: &str) -> (&'static str, &'static str) {
    match chave {
        "save_over_2gb" => (
            "Permite gravar e carregar saves acima de 2 GB (até 4 GB). Ligue se o seu save é grande (muitas ligas, carreira longa) ou se o jogo dá erro ao carregar. Desligado, um save acima de 2 GB não carrega. Não muda nada na carreira.",
            "Lets the game save and load saves above 2 GB (up to 4 GB). Turn it on if your save is large (many leagues, a long career) or the game errors when loading. Off, a save above 2 GB won't load. It changes nothing in the career.",
        ),
        "multiple_instances" => (
            "Deixa abrir mais de um jogo ao mesmo tempo, desde que cada um esteja na sua pasta (por exemplo, o 3.9.68 e o CLZ, ou duas carreiras em duas cópias). O jogo tem uma trava que só deixa uma cópia aberta; com esta opção, a trava vale por pasta. Dois jogos da mesma pasta continuam barrados, porque usariam os mesmos arquivos temporários e o mesmo save. Ligue também a pasta portátil, para cada cópia usar a própria pasta. Cada jogo aberto ocupa a memória dele.",
            "Lets you open more than one game at the same time, as long as each is in its own folder (for example, 3.9.68 and CLZ, or two careers in two copies). The game has a lock that only allows one copy open; with this option, the lock is per folder. Two games from the same folder are still blocked, since they would use the same temporary files and the same save. Turn on the portable folder too, so each copy uses its own folder. Each open game uses its own memory.",
        ),
        "portable_folder" => (
            "O jogo usa a pasta onde está o exe, sem precisar do SetupReg nem do registro do Windows. Útil para ter várias instalações lado a lado ou uma cópia num pendrive. Nada é gravado no registro.",
            "The game uses the folder its exe is in, with no SetupReg and no Windows registry. Useful for several installs side by side or a copy on a USB stick. Nothing is written to the registry.",
        ),
        "clear_admin_flag" => (
            "Só para quem usa a NickDLL (CLZ e outros pacotes do NickPatcher): ela marca o exe para rodar como administrador a cada abertura. Esta opção tira essa marca (mantém o modo Windows 95). Vale a partir da abertura seguinte. Deixe desligado se você marcou isso de propósito.",
            "Only for NickDLL users (CLZ and other NickPatcher packs): it marks the exe to run as administrator every time. This removes that mark (keeps Windows 95 mode). It takes effect from the next start. Leave it off if you set that on purpose.",
        ),
        "override_exe_changes" => (
            "Quando o seu exe já tem uma modificação no mesmo ponto de um ajuste (o Regen Tweak, as idades do NickPatcher), o normal é ficar com a do exe. Ligado, os ajustes desta DLL substituem essas modificações. Use só se você quer as regras desta DLL no lugar das do seu pacote.",
            "When your exe already has a change where a tweak hooks in (the Regen Tweak, NickPatcher's ages), the exe's stays by default. On, this DLL's tweaks replace those changes. Use it only if you want this DLL's rules instead of your pack's.",
        ),
        "video" => (
            "Janela: o jogo desenha numa janela comum. O Print Screen e os gravadores de tela funcionam, a janela muda de tamanho com escala nítida, e o Windows nunca troca de resolução (sem piscar no Alt+Tab). Original: o DirectDraw do Windows, como sempre.",
            "Window: the game draws to an ordinary window. Print Screen and screen recorders work, the window resizes with sharp scaling, and Windows never changes resolution (no flicker on Alt+Tab). Original: Windows' DirectDraw, as always.",
        ),
        "resolution" => (
            "1920×1080: as telas do jogo em 1920×1080, aplicado em memória (o exe não é alterado). Liga o vídeo em janela junto e evita o erro do relatório de partidas antigas. Se o exe ou a NickDLL já têm resolução própria, fica com a deles.",
            "1920×1080: the game's screens at 1920×1080, applied in memory (the exe file isn't changed). It turns on the windowed video too and avoids the old match report error. If the exe or NickDLL already has its own resolution, theirs stays.",
        ),
        "no_cd" => (
            "O jogo roda sem o CD. Se você já usa um NoCD (a LaMOHatesCD do CLZ, um exe sem CD), fica com ele.",
            "The game runs without the CD. If you already use a NoCD (CLZ's LaMOHatesCD, a no-CD exe), that one stays.",
        ),
        "ages" => (
            "Tabela: a idade com que os regens nascem segue a tabela de idades (as barras). Original: o sorteio do jogo (quase todos de 16 a 20). Use a tabela para regens mais novos ou mais velhos. Vale para os regens que nascerem dali em diante, também num save em andamento.",
            "Table: the age regens are born with follows the age table (the bars). Original: the game's draw (almost all 16 to 20). Use the table for younger or older regens. It applies to regens born from then on, in a save in progress too.",
        ),
        "age_NN" => (
            "A chance de um regen nascer com cada idade (as chances não precisam somar 100) e o teto do CA naquela idade: o jovem não começa com CA acima do teto. Arraste a barra para mudar a chance; zero tira a idade.",
            "The chance of a regen being born at each age (the chances don't need to add up to 100) and the CA cap at that age: the youngster doesn't start with CA above the cap. Drag the bar to change the chance; zero removes the age.",
        ),
        "younger_from_age" => (
            "\"Regens mais novos\" (ideia do petermf23): os que sairiam com esta idade ou mais nascem mais novos. Funciona junto com \"anos a menos\".",
            "\"Younger regens\" (petermf23's idea): those who would come out at this age or older are born younger. Works with \"years less\".",
        ),
        "younger_by" => (
            "Quantos anos a menos para os regens de \"a partir de\" para cima. 0 desliga. Mais anos a menos = mais jovens de 16 a 20 e menos veteranos recém-gerados.",
            "How many years younger for regens from \"from age\" up. 0 turns it off. More years = more 16-to-20 youngsters and fewer newly generated veterans.",
        ),
        "ages_out_of_game" => (
            "As nações fora do jogo com poucas pessoas no banco sorteiam idades de um jeito próprio (quase sempre 23 a 27). \"A tabela também\" faz essas nações usarem a sua tabela. Só muda nações cuja liga você não escolheu.",
            "Nations not in the game with few people in the database draw ages their own way (mostly 23 to 27). \"The table too\" makes those nations use your table. It only changes nations whose league you didn't choose.",
        ),
        "ca_cap_skip" => (
            "Chance (%) de um jovem ignorar o teto do CA da idade dele: alguns nascem já prontos, como no jogo original (cerca de 1 em 200). Mais alto = mais jovens prontos cedo.",
            "Chance (%) that a youngster ignores the CA cap of his age: a few are born ready, as in the original game (about 1 in 200). Higher = more youngsters ready early.",
        ),
        "contest_max_age" => (
            "Até que idade o jovem vai à disputa dos clubes no jogo original (os mais velhos começam sem clube). Sem efeito com \"Sem clube só sem vaga\", que manda todos à disputa.",
            "Up to what age a youngster goes to the clubs' contest in the original game (older ones start without a club). No effect with \"Clubless only without room\", which sends everyone.",
        ),
        "unattached" => (
            "Sem clube só sem vaga: todo jovem de nação no jogo vai à disputa e só fica sem clube se nenhum clube do país tiver vaga. Original: só os de até 18 anos vão, e 1 em 4 é descartado (muitos começam sem clube e se perdem). Recomendado: menos talentos desperdiçados.",
            "Clubless only without room: every youngster from a nation in the game goes to the contest and is only left without a club if no club in his country has room. Original: only those up to 18 go, and 1 in 4 is dropped (many start clubless and get lost). Recommended: less wasted talent.",
        ),
        "retire_idle_players" => (
            "O jogador sem clube que ninguém quer se aposenta mais cedo, e a ficha dele vira um regen semanas depois. Resultado: mais jovens e menos veteranos parados no mercado. Num pacote que barra aposentadorias (o RetirePatchV4 do NickPatcher), quase não age.",
            "A clubless player nobody wants retires sooner, and his record becomes a regen a few weeks later. Result: more youngsters and fewer idle veterans on the market. In a pack that blocks retirements (NickPatcher's RetirePatchV4) it barely acts.",
        ),
        "idle_days" => (
            "Quantos dias sem clube antes de a regra dos inativos valer. Menos dias = aposentadorias mais rápidas.",
            "How many days without a club before the idle rule applies. Fewer days = faster retirements.",
        ),
        "idle_min_age" => (
            "A partir de que idade a regra dos inativos vale. Abaixo dela, fica a regra original do jogo.",
            "From what age the idle rule applies. Below it, the game's original rule stays.",
        ),
        "idle_rep" => (
            "A reputação até a qual o inativo se aposenta: quem tem reputação menor sai quase sempre; quem tem mais, raramente. Mais alto = mais aposentadorias.",
            "The reputation up to which an idle player retires: lower-reputation players almost always go; higher ones rarely. Higher = more retirements.",
        ),
        "idle_rep_per_year" => (
            "Quanto a reputação-limite sobe por ano acima da idade mínima: os mais velhos se aposentam mais fácil.",
            "How much the reputation threshold rises per year above the minimum age: older players retire more easily.",
        ),
        "protect_young_unemployed" => (
            "Abaixo desta idade, nenhum jogador sem clube se aposenta pela regra dos inativos. 0 = sem proteção (o padrão: o jogo já dá 365 dias ao jovem sem clube). Proteger os jovens diminui a reciclagem em regens.",
            "Below this age no clubless player retires by the idle rule. 0 = no protection (the default: the game already gives a clubless youngster 365 days). Protecting the young reduces recycling into regens.",
        ),
        "rule" => (
            "Sorteio proporcional: o jovem vai para um clube do tamanho dele, sorteado pelo nível, instalações, comissão e vaga, com no máximo 2 jovens por clube por ano. Original: os grandes recolhem quase todos (até 14 numa temporada). Recomendado para carreiras equilibradas.",
            "Proportional draw: the youngster goes to a club his size, drawn by level, facilities, staff and room, with at most 2 youngsters per club a year. Original: big clubs collect almost all of them (up to 14 in a season). Recommended for balanced careers.",
        ),
        "target_k" => (
            "Que reputação de clube combina com o jovem: o alvo é este valor vezes o PA (50 × PA 150 = clube de 7500). Mais alto = jovens bons indo para clubes maiores.",
            "What club reputation suits the youngster: the target is this times the PA (50 × PA 150 = a 7500 club). Higher = good youngsters going to bigger clubs.",
        ),
        "target_ca" => (
            "Quanto o alvo olha o CA (o que o jovem já sabe) em vez do PA (o que pode vir a ser). 0 = só o PA; 1 = só o CA.",
            "How much the target looks at the CA (what the youngster can do now) instead of the PA (what he may become). 0 = PA only; 1 = CA only.",
        ),
        "sigma" => (
            "Quanto um clube acima do alvo ainda tem chance. Maior = os jovens se espalham por clubes de níveis mais variados; menor = vão para clubes bem do tamanho deles.",
            "How much a club above the target still has a chance. Larger = youngsters spread over clubs of more varied levels; smaller = they go to clubs right their size.",
        ),
        "sigma_below" => (
            "O mesmo, para os clubes abaixo do alvo (0 = igual à largura de cima). Mantido menor porque as centenas de clubes pequenos de um país, somadas, levariam os jovens bons.",
            "The same for clubs below the target (0 = the same as above). Kept smaller because the hundreds of small clubs in a country, added up, would take the good youngsters.",
        ),
        "level_floor" => (
            "A chance mínima de um clube longe do alvo (0 a 1). Acima de 0, um clube pequeno ainda pode, raramente, receber um craque.",
            "The minimum chance of a club far from the target (0 to 1). Above 0, a small club can still, rarely, get a star.",
        ),
        "facilities_base" => (
            "Quanto as instalações de base (Training) pesam: o peso é esta base mais o valor por ponto. Clubes com boa base atraem mais jovens.",
            "How much youth facilities (Training) weigh: the weight is this base plus the per-point value. Clubs with good facilities attract more youngsters.",
        ),
        "facilities_k" => (
            "Quanto cada ponto de Training soma ao peso (com 0,4 e 0,06: Training 1 = 0,46; 10 = 1,00; 20 = 1,60).",
            "How much each Training point adds to the weight (with 0.4 and 0.06: Training 1 = 0.46; 10 = 1.00; 20 = 1.60).",
        ),
        "staff_min" => (
            "O peso do clube cuja comissão trabalha pior com jovens (\"Working with Youngsters\" 1).",
            "The weight of the club whose staff is worst with youngsters (\"Working with Youngsters\" 1).",
        ),
        "staff_max" => (
            "O peso do clube cuja comissão trabalha melhor com jovens (\"Working with Youngsters\" 20). Deve ser maior que o do pior.",
            "The weight of the club whose staff is best with youngsters (\"Working with Youngsters\" 20). It should be above the worst's.",
        ),
        "squad_limit" => (
            "Clube com este número de jogadores ou mais não recebe jovem. Mais baixo = os jovens vão para clubes com elenco menor.",
            "A club with this many players or more gets no youngster. Lower = youngsters go to clubs with smaller squads.",
        ),
        "squad_exp" => (
            "O quanto a vaga no elenco pesa: maior = clubes com elenco curto têm muito mais chance que os de elenco quase cheio.",
            "How much squad room weighs: larger = clubs with short squads get much more chance than nearly full ones.",
        ),
        "season_cap" => (
            "Quantos jovens um clube recebe no máximo a cada 365 dias pela disputa (0 = sem teto). É o que impede um clube de recolher dezenas de regens. Com 3, os elencos das ligas jogadas ficam um pouco mais cheios.",
            "How many youngsters a club gets at most every 365 days from the contest (0 = no cap). It's what stops a club collecting dozens of regens. With 3, the squads of the leagues you play stay a bit fuller.",
        ),
        "surprise" => (
            "Percentagem dos jovens sorteados sem olhar o nível do clube, só entre clubes das competições: o craque que surge num clube pequeno, ou o comum num grande. 0 = nunca.",
            "Percentage of youngsters drawn without looking at the club's level, only among competition clubs: the star who turns up at a small club, or the ordinary one at a big club. 0 = never.",
        ),
        "outside_competitions" => (
            "O peso dos clubes cuja divisão não roda na sua carreira (nem como liga de fundo). 1 = iguais aos outros; 0,25 = um quarto da chance; 0 = nunca (sem clube de competição com vaga, o jovem fica sem clube). Valores baixos mantêm cheios os elencos das ligas que você joga; com poucas ligas carregadas, 1 pode servir melhor.",
            "The weight of clubs whose division doesn't run in your career (not even as a background league). 1 = the same as the others; 0.25 = a quarter of the chance; 0 = never (with no competition club with room, the youngster stays clubless). Low values keep the squads of the leagues you play fuller; with few leagues loaded, 1 may suit better.",
        ),
        "temperature" => (
            "Acentua (maior que 1) ou suaviza (menor que 1) as diferenças de peso entre os clubes. 1 = o peso como calculado.",
            "Sharpens (above 1) or softens (below 1) the weight differences between clubs. 1 = the weight as calculated.",
        ),
        "regen_names" => (
            "Os da própria nação: os regens (e a comissão e os árbitros novos) ganham nomes do próprio país sempre que o banco tiver nomes suficientes dele (o mínimo abaixo). Original: o jogo só usa os nomes do país quando há mais de 100; com menos, pega emprestado de outro país da mesma língua ou de qualquer país do mundo, e daí vêm os regens com nomes de outras nações. A aba mostra, país por país do seu banco, de onde vêm os nomes hoje e o que muda. Vale para as pessoas criadas dali em diante, também num save em andamento. Os nomes vêm do banco: nada é acrescentado ao save.",
            "The nation's own: regens (and new staff and referees) get names from their own country whenever the database has enough of them (the minimum below). Original: the game only uses a country's names when it has more than 100; with fewer, it borrows from another country of the same language or from any country in the world, which is where regens with names from other nations come from. The tab shows, country by country in your database, where the names come from today and what changes. It applies to people created from then on, in a career already under way too. The names come from the database: nothing is added to the save.",
        ),
        "regen_names_min" => (
            "Quantos nomes do país o banco precisa ter para ele usar os próprios. Quanto menor, mais países usam os próprios nomes; a lista da aba mostra quais. Abaixo do mínimo fica o empréstimo do original. Vale separado para o primeiro nome e o sobrenome.",
            "How many of a country's names the database needs for it to use its own. The lower it is, the more countries use their own names; the tab's list shows which. Below the minimum, the original's borrowing stays. It applies separately to first names and surnames.",
        ),
        "grey_players" => (
            "No nível do clube: cada jogador cinzento (os virtuais que o jogo põe para completar o elenco antes da partida) tem o nível do próprio clube, com a variação escolhida abaixo. Original: a conta do jogo, que tira o nível da reputação e despenca nos clubes médios e pequenos (numa carreira de teste, 63% dos cinzentos saíram com CA negativo, o pior jogador possível). O mesmo clube continua recebendo os mesmos cinzentos: só o CA muda. Vale a partir da próxima partida, também num save em andamento.",
            "At the club's level: each grey player (the virtual ones the game adds to fill a squad before a match) gets the club's own level, within the variation chosen below. Original: the game's formula, which takes the level from the reputation and collapses at mid-sized and small clubs (in a test career, 63% of grey players got a negative CA, the weakest possible player). The same club still gets the same grey players: only the CA changes. It applies from the next match, in a career already under way too.",
        ),
        "grey_players_spread" => (
            "Quanto o CA de cada cinzento pode ficar acima ou abaixo do nível do clube. Com ± 5, num clube de nível 82 os cinzentos vêm de 77 a 87. ± 0 deixa todos iguais ao nível; mais variação, cinzentos mais diferentes entre si. No tweaks.cfg fica a largura total (± 5 = 10).",
            "How far each grey player's CA can be above or below the club's level. With ± 5, at a level-82 club the grey players come from 77 to 87. ± 0 makes them all equal to the level; more variation, grey players more different from each other. tweaks.cfg keeps the full width (± 5 = 10).",
        ),
        "intake_stock" => (
            "Liga a fornada (0 = desligada). É o estoque da garantia: quantas pessoas a DLL cria com a carreira nova, adormecidas, para completar os clubes que a disputa deixou com menos jovens que o mínimo. Os regens do próprio jogo não saem daqui: eles vêm das aposentadorias. Com 3.000, sobram uns 2.650 depois de três fornadas; o máximo é a folga das tabelas do jogo, umas 120.000. Só vale numa carreira nova.",
            "Turns the intake on (0 = off). It's the stock for the guarantee: how many people the DLL creates with a new career, kept asleep, to top up the clubs the contest left with fewer youngsters than the minimum. The game's own regens don't come from here: they come from retirements. With 3,000, about 2,650 are left after three intakes; the maximum is the room in the game's tables, about 120,000. It only works in a new career.",
        ),
        "intake_per_club" => (
            "O mínimo garantido por clube em cada fornada. Primeiro nascem os regens do jogo, pela disputa (a curva da aba Regens decide o clube de cada um); depois, todo clube com espaço no elenco que ficou abaixo desse número é completado: primeiro com os regens do jogo que a disputa deixou sem clube (até 21 anos, do país do clube), e só o que faltar com o estoque. Um clube que recebeu 5 pela disputa fica com 5; um que recebeu 1 ganha mais 2 do estoque. 0 = sem garantia.",
            "The minimum each club is guaranteed per intake. First the game's regens are born, through the contest (the Regens tab's curve picks each one's club); then every club with squad room that ended below this number is topped up: first with the game's regens the contest left without a club (up to 21, from the club's country), and only the rest from the stock. A club that got 5 from the contest keeps 5; one that got 1 gets 2 more from the stock. 0 = no guarantee.",
        ),
        "intake_day" => (
            "O dia em que os jovens chegam. Num dia fixo (contado de 0 = 1º de janeiro; 181 = 1º de julho), todos os países juntos. No fim da temporada, cada país quando a temporada dele acaba (o Brasil no começo do ano, a Inglaterra e a Itália no meio); os países fora da lista dos 35 principais do jogo recebem no dia 181. No tweaks.cfg vale qualquer dia de 0 a 365.",
            "The day the youngsters arrive. On a fixed day (counted from 0 = 1 January; 181 = 1 July), every country at once. At season end, each country when its season ends (Brazil early in the year, England and Italy mid-year); countries outside the game's list of 35 main ones receive on day 181. In tweaks.cfg any day from 0 to 365 works.",
        ),
        "intake_clubs" => (
            "Quais clubes têm a garantia do mínimo: os das ligas jogáveis, ou também os das ligas de fundo que você escolheu ao criar a carreira. Os regens do jogo continuam indo para onde a disputa mandar.",
            "Which clubs get the guaranteed minimum: those of the playable leagues, or also those of the background leagues you chose when creating the career. The game's regens still go wherever the contest sends them.",
        ),
        "intake_pa_min" => (
            "Só para os jovens do estoque (a garantia). O PA de um clube de reputação 0, na escala do jogo de 1 a 200. Entre 0 e 10.000 de reputação, o PA sobe em linha reta até o máximo: com 60 e 170, um clube de reputação 5.000 fica com 115. Os regens do jogo têm o PA da ficha do aposentado, como sempre.",
            "Only for stock youngsters (the guarantee). The PA of a reputation-0 club, on the game's 1 to 200 scale. Between reputation 0 and 10,000 the PA rises in a straight line to the maximum: with 60 and 170, a club of reputation 5,000 gets 115. The game's regens have the retired player's record PA, as always.",
        ),
        "intake_pa_max" => (
            "Só para os jovens do estoque. O PA de um clube de reputação 10.000, e o teto: nenhum jovem do estoque passa dele, nem com a variação nem com a surpresa.",
            "Only for stock youngsters. The PA of a reputation-10,000 club, and the cap: no stock youngster goes above it, not with the spread nor with the surprise.",
        ),
        "intake_pa_spread" => (
            "Só para os jovens do estoque. A variação sorteada em volta do PA do clube: 30 vai de 15 abaixo a 15 acima. 0 = todo jovem do estoque no clube com o mesmo PA. O resultado fica sempre entre o mínimo e o máximo.",
            "Only for stock youngsters. The random variation around the club's PA: 30 goes from 15 below to 15 above. 0 = every stock youngster at the club with the same PA. The result always stays between the minimum and the maximum.",
        ),
        "intake_surprise" => (
            "Só para os jovens do estoque. A chance, em %, de um deles ser a surpresa: o PA sorteado entre o do clube e o teto (intake_pa_max), em vez da variação de sempre. É o craque que aparece no clube pequeno. 10 = 1 em cada 10. 0 = nunca.",
            "Only for stock youngsters. The chance, in %, of one of them being the surprise: the PA drawn between the club's and the cap (intake_pa_max), instead of the usual spread. It's the gem that turns up at a small club. 10 = 1 in 10. 0 = never.",
        ),
        "intake_squad_max" => (
            "O clube com esse número de jogadores no elenco (o elenco tem 50 vagas) não recebe jovens do estoque naquele ano. Com 50, só o elenco lotado fica de fora. Os regens do jogo seguem o espaço no elenco da aba Regens.",
            "A club with this many players in the squad (the squad has 50 places) gets no stock youngsters that year. With 50, only a full squad is left out. The game's regens follow the squad room of the Regens tab.",
        ),
        "intake_only" => (
            "1 = os regens do jogo só nascem no dia da fornada: durante a temporada, quem se aposenta fica guardado na fila do jogo, e no dia da fornada todos nascem de uma vez, cada um pela disputa, com tudo da aba Regens (nível, instalações, comissão, espaço, teto, surpresa, temperatura), as idades e os nomes, e entram no regen_origins. Só a data muda. 0 = os regens do jogo nascem ao longo da temporada, como sempre, e o estoque dá intake_per_club a cada clube por cima.",
            "1 = the game's regens are only born on intake day: through the season, whoever retires waits in the game's queue, and on intake day they are all born at once, each through the contest, with everything on the Regens tab (level, facilities, staff, room, cap, surprise, temperature), the ages and the names, and they go into regen_origins. Only the date changes. 0 = the game's regens are born through the season, as always, and the stock gives every club intake_per_club on top.",
        ),
        "intake_news" => (
            "No dia da fornada, o seu clube recebe na caixa de entrada uma mensagem do treinador da base com todos os jovens que chegaram (os da disputa e os do estoque) e o parecer sobre cada um: se já está pronto para o time principal (comparado com o seu elenco) e até onde pode chegar. Sai na língua do jogo. As frases ficam no intake_news.txt, ao lado do exe, uma seção para cada uma das 19 línguas do jogo: dá para editar ou traduzir (português, espanhol, inglês, francês, alemão, italiano, holandês, dinamarquês, norueguês e sueco já vêm traduzidos). Sem a DLL, a mensagem aparece numa linha curta.",
            "On intake day your club gets an inbox message from the youth coach with every youngster who arrived (from the contest and from the stock) and his view of each one: whether he's ready for the first team (compared with your squad) and how far he can go. It comes in the game's language. The phrases are in intake_news.txt, next to the exe, one section for each of the game's 19 languages: you can edit or translate them (Portuguese, Spanish, English, French, German, Italian, Dutch, Danish, Norwegian and Swedish come translated). Without the DLL the message shows as one short line.",
        ),
        "intake_news_numbers" => (
            "Mostrar também o CA e o PA de cada jovem na mensagem. Desligado, fica só o parecer do treinador, e a graça é descobrir o jogador.",
            "Also show each youngster's CA and PA in the message. Off, there's only the coach's view, and the fun is finding out about the player.",
        ),
        "retire_day" => (
            "O dia do ano das aposentadorias de jogadores. No jogo original, cada um se aposenta numa data própria (em geral o fim da temporada do país). Com um dia (contado de 0 = 1º de janeiro), todas as aposentadorias marcadas a partir de agora acontecem nele; um pouco antes da fornada, quem se aposenta já nasce na fornada seguinte. Vale com ou sem a fornada. No tweaks.cfg vale qualquer dia de 0 a 365.",
            "The day of the year for player retirements. In the original game each one retires on a date of its own (usually the country's season end). With a day (counted from 0 = 1 January), every retirement set from then on happens on it; a little before the intake, whoever retires is born in the next intake. Works with or without the intake. In tweaks.cfg any day from 0 to 365 works.",
        ),
        "regen_origins" => (
            "Grava no regen_origins.csv, ao lado do exe, cada regen que nasce: o nome, a idade, a posição, o PA, o CA e o clube que ganhou a disputa (ou \"sem clube\"), e de quem ele herdou: a vaga (o id e a personalidade) e a ficha de jogador (o PA, as posições e os atributos), com o nome, a idade, o CA e o PA do dono no momento da aposentadoria. Cada linha leva a data do jogo, a data do computador e a carreira (o save). Assim você acha \"o regen do Ronaldo\". O jogo não guarda isso em lugar nenhum: só os regens criados com esta opção ligada ficam registrados. O arquivo abre no Excel com dois cliques (UTF-8, colunas separadas por ponto e vírgula; num Excel em inglês, use Dados > De Texto/CSV). Ao gravar o jogo, a DLL grava ao lado do save um X.regens com os donos das fichas que ainda vão virar regen; ao carregar, ela o lê. Copie os dois juntos: sem o .regens, alguns dos primeiros regens depois da carga podem sair com o dono desconhecido.",
            "Writes to regen_origins.csv, next to the exe, every regen born: his name, age, position, PA, CA and the club that won the contest (or \"no club\"), and whom he inherited from: the slot (the id and personality) and the player record (the PA, positions and attributes), with the owner's name, age, CA and PA at retirement. Each line has the game date, the computer's date and the career (the save). So you can find \"Ronaldo's regen\". The game keeps this nowhere: only regens created with this option on are recorded. The file opens in Excel with a double click (UTF-8, columns separated by semicolons; in an English Excel, use Data > From Text/CSV). When you save the game, the DLL writes an X.regens next to the save with the owners of the records still waiting to become regens; when you load, it reads it. Copy both together: without the .regens, some of the first regens after loading may show an unknown owner.",
        ),
        "log" => (
            "Grava o tweaks.log ao lado do exe: o que cada ajuste fez, cada jovem e o clube, as caixas de erro e, todo julho, o retrato da população. Para testar e para mandar quando algo der errado. Desligado no uso normal.",
            "Writes tweaks.log next to the exe: what each tweak did, each youngster and his club, error boxes and, every July, a picture of the population. For testing and to send when something goes wrong. Off in normal use.",
        ),
        _ => ("", ""),
    }
}

use crate::chaves::Aba;

/// Um termo da aba: (nome pt, nome en, explicação pt, explicação en).
pub type Termo = (&'static str, &'static str, &'static str, &'static str);

/// O que a aba faz, no topo dela.
pub fn intro(aba: Aba) -> (&'static str, &'static str) {
    match aba {
        Aba::Inicio => (
            "Este programa escreve o tweaks.cfg, o arquivo que liga e regula os ajustes da ddraw.dll. Escolha a pasta do jogo no topo, comece por uma predefinição (no topo, à direita) e ajuste o que quiser nas abas da esquerda. Nada vale até você clicar em Gravar; o jogo lê o arquivo ao abrir. Aqui embaixo está o que cada ajuste vai fazer no exe desta pasta: verde liga, cinza está desligado, azul fica com a versão do exe ou de outra DLL, amarelo substitui a versão do exe.",
            "This program writes tweaks.cfg, the file that turns on and tunes the ddraw.dll tweaks. Pick the game folder at the top, start from a preset (top right) and change what you like in the tabs on the left. Nothing applies until you click Save; the game reads the file when it starts. Below is what each tweak will do on this folder's exe: green turns on, grey is off, blue keeps the exe's or another DLL's version, yellow replaces the exe's version.",
        ),
        Aba::Idades => (
            "Com que idade nasce cada regen, e com quanto CA. As barras são as idades de 14 a 35: a altura é a chance de nascer com aquela idade, e o número embaixo é o teto do CA. Clique numa barra para escolhê-la e arraste para mudar a chance. Os ajustes valem para os regens que nascerem dali em diante, também num save em andamento.",
            "At what age each regen is born, and with how much CA. The bars are the ages 14 to 35: the height is the chance of being born at that age, and the number below is the CA cap. Click a bar to pick it and drag to change the chance. The settings apply to regens born from then on, in a save in progress too.",
        ),
        Aba::Disputa => (
            "Para qual clube vai o regen jovem. Quando nasce um jovem de uma nação no jogo, os clubes do país dele disputam o jogador. Aqui você escolhe a regra dessa disputa e regula o sorteio. O gráfico e a lista de chances usam um jovem de exemplo e clubes de exemplo: mexa no PA e no CA dele para ver para onde a regra o mandaria.",
            "Which club the young regen goes to. When a youngster from a nation in the game is born, the clubs of his country compete for him. Here you choose the rule of that contest and tune the draw. The chart and the list of chances use an example youngster and example clubs: move his PA and CA to see where the rule would send him.",
        ),
        Aba::Inativos => (
            "A aposentadoria dos jogadores sem clube que ninguém quer. Todo regen nasce da ficha de um aposentado: aposentar mais cedo o veterano parado faz nascer mais jovens, sem mudar o total de pessoas nem o formato do save. O gráfico mostra a chance de aposentar por idade, para um jogador de exemplo.",
            "Retiring clubless players nobody wants. Every regen is born from a retired player's record: retiring the idle veteran sooner means more youngsters are born, with the same total number of people and no change to the save format. The chart shows the chance of retiring by age, for an example player.",
        ),
        Aba::Registro => (
            "De quem é cada regen. O jogo não guarda isso em lugar nenhum: com a opção ligada, a DLL anota cada regen que nasce, de quem ele herdou a vaga e de quem herdou a ficha de jogador, num arquivo regen_origins.csv ao lado do exe. Aqui você busca nesse histórico.",
            "Where each regen comes from. The game keeps this nowhere: with the option on, the DLL notes each regen born, whom he inherited the slot from and whom he inherited the player record from, in a regen_origins.csv file next to the exe. Here you search that history.",
        ),
        Aba::Jogo => (
            "O save, a instalação, a tela e o CD. Estes ajustes não mudam as regras do jogo: só como ele grava, onde procura os arquivos e como desenha.",
            "The save, the install, the screen and the CD. These tweaks don't change the game's rules: only how it saves, where it looks for its files and how it draws.",
        ),
        Aba::Nomes => (
            "Os nomes dos regens. Os nomes vêm do banco de dados (first_names.dat e second_names.dat, cada nome com a nação e o quanto é comum). A regra de quando o jogo usa os nomes do próprio país está no exe: só acima de 100 nomes. Com menos, ele pega emprestado de outro país, e o regen de Serra Leoa pode sair com nome de outro lugar. Aqui o país usa os próprios nomes a partir de um mínimo.",
            "Regen names. Names come from the database (first_names.dat and second_names.dat, each name with its nation and how common it is). The rule for when the game uses the country's own names is in the exe: only above 100 names. With fewer, it borrows from another country, and a Sierra Leone regen may get a name from somewhere else. Here the country uses its own names from a minimum.",
        ),
        Aba::Cinzentos => (
            "Os jogadores cinzentos. Antes de cada partida (e quando você abre um elenco), o jogo completa os elencos curtos com jogadores virtuais, que aparecem em cinza e ninguém pode contratar. O original dá a eles uma habilidade tirada da reputação do clube, por uma conta que despenca: nos clubes médios e pequenos ela sai negativa, e o motor da partida trata isso como o pior jogador possível. Aqui eles podem vir no nível do próprio clube.",
            "Grey players. Before every match (and when you open a squad), the game fills short squads with virtual players, shown in grey, whom nobody can sign. The original gives them an ability taken from the club's reputation, by a formula that collapses: at mid-sized and small clubs it comes out negative, and the match engine treats that as the weakest possible player. Here they can come at the club's own level.",
        ),
        Aba::Fornada => (
            "EXPERIMENTAL. A fornada de regens (o Youth Intake do Football Manager; hornada no espanhol): os regens chegam todos de uma vez, num dia do ano, em vez de nascerem um a um ao longo da temporada. O que muda é só a data: cada regen do jogo continua passando pela disputa dos clubes, com toda a curva da aba Regens, e continua no regen_origins. Por cima disso, a garantia: todo clube com espaço no elenco fica com pelo menos intake_per_club jovens, completados com um estoque de pessoas criado com a carreira. E o seu clube recebe uma mensagem do treinador da base. Só vale numa carreira NOVA.",
            "EXPERIMENTAL. The regen intake (Football Manager's Youth Intake): regens arrive all at once, on one day of the year, instead of being born one by one through the season. Only the date changes: each game regen still goes through the clubs' contest, with the whole curve of the Regens tab, and still goes into regen_origins. On top of that, the guarantee: every club with squad room ends with at least intake_per_club youngsters, topped up from a stock of people created with the career. And your club gets a message from the youth coach. It only works in a NEW career.",
        ),
        Aba::Cfg => (
            "O tweaks.cfg como vai ficar quando você clicar em Gravar. Em amarelo, as linhas mudadas ou acrescentadas. Nenhuma linha é apagada: o que você escreveu à mão, e os comentários, ficam.",
            "tweaks.cfg as it will be when you click Save. In yellow, the lines changed or added. No line is removed: what you wrote by hand, and the comments, stay.",
        ),
        Aba::Log => (
            "O tweaks.log é o diário da DLL: o exe que ela encontrou, o que cada ajuste fez e, com o registro ligado, cada jovem e o clube dele. É o arquivo para mandar quando algo der errado.",
            "tweaks.log is the DLL's diary: the exe it found, what each tweak did and, with logging on, each youngster and his club. It's the file to send when something goes wrong.",
        ),
    }
}

/// Os termos que a aba usa, explicados.
pub fn termos(aba: Aba) -> &'static [Termo] {
    match aba {
        Aba::Inicio => &[
            ("Nação no jogo", "Nation in the game", "a nação cuja liga você escolheu ao criar a carreira, jogável ou de fundo (para os regens as duas contam igual). Fora do jogo é a nação cuja liga não foi escolhida.", "a nation whose league you chose when creating the career, playable or background (for regens both count the same). Not in the game is a nation whose league wasn't chosen."),
            ("NickPatcher", "NickPatcher", "um programa da comunidade que altera o exe. Se ele já mexeu no mesmo ponto de um ajuste, o ajuste fica com a versão do exe e avisa aqui.", "a community program that changes the exe. If it already changed the same place as a tweak, the tweak keeps the exe's version and says so here."),
        ],
        Aba::Idades => &[
            ("Regen", "Regen", "o jogador novo que o jogo cria quando alguém se aposenta. O jogo não cria uma pessoa do zero: reaproveita a vaga de um aposentado e a ficha de jogador de outro.", "the new player the game creates when someone retires. The game doesn't create a person from scratch: it reuses one retiree's slot and another's player record."),
            ("CA", "CA", "Current Ability, a habilidade atual (1 a 200): o quanto o jogador joga hoje.", "Current Ability (1 to 200): how well the player plays today."),
            ("PA", "PA", "Potential Ability, o potencial (1 a 200): até onde o CA pode chegar com o tempo. O regen herda o PA da ficha de jogador que recebeu.", "Potential Ability (1 to 200): how far the CA can grow over time. The regen inherits the PA of the player record he got."),
            ("Chance", "Chance", "o peso de cada idade no sorteio. Não precisa somar 100: o que vale é a proporção entre as idades (a porcentagem em cima de cada barra). 0 tira a idade.", "each age's weight in the draw. It doesn't need to add up to 100: what counts is the proportion between ages (the percentage above each bar). 0 removes the age."),
            ("Teto do CA", "CA cap", "o CA máximo com que um jovem daquela idade nasce, o número embaixo da barra. Com teto 80 aos 17 anos, nenhum regen de 17 nasce com CA acima de 80, mesmo que o PA seja 180: ele precisa crescer. Sem teto, o CA sai só da fórmula do jogo.", "the highest CA a youngster of that age is born with, the number below the bar. With a cap of 80 at 17, no 17-year-old regen is born with CA above 80, even with PA 180: he has to grow. With no cap, the CA comes only from the game's formula."),
            ("age_18 no tweaks.cfg", "age_18 in tweaks.cfg", "é como a idade 18 fica escrita no arquivo: \"age_18 = 29.11, 100\" quer dizer chance 29,11 e teto do CA 100. Cada barra é uma linha dessas, de age_14 a age_35.", "it's how age 18 is written in the file: \"age_18 = 29.11, 100\" means chance 29.11 and CA cap 100. Each bar is one of these lines, from age_14 to age_35."),
            ("Disputa", "Contest", "a escolha do clube que recebe o jovem (aba Disputa dos clubes). Aqui ficam as duas opções que decidem quem entra nela.", "the choice of the club that gets the youngster (Clubs' contest tab). The two options that decide who enters it are here."),
        ],
        Aba::Disputa => &[
            ("Original", "Original", "a regra do 3.9.68: os clubes de maior reputação e melhores instalações ganham quase sempre. Um grande chega a juntar mais de 10 regens numa temporada, e os pequenos quase nunca recebem.", "the 3.9.68 rule: clubs with the highest reputation and best facilities almost always win. A big club can collect over 10 regens in a season, and small ones almost never get any."),
            ("Sorteio proporcional", "Proportional draw", "a regra desta DLL: cada clube do país ganha um peso, e o clube é sorteado na proporção do peso. Um clube com o dobro do peso tem o dobro da chance; ninguém tem garantia e ninguém fica excluído. O peso é nível × instalações × comissão × vaga.", "this DLL's rule: each club in the country gets a weight, and the club is drawn in proportion to its weight. A club with twice the weight has twice the chance; nobody is guaranteed and nobody is left out. The weight is level × facilities × staff × room."),
            ("Reputação do clube", "Club reputation", "o tamanho do clube no jogo, de 0 a 10000 (um gigante europeu passa de 9000; um clube de bairro fica perto de 1000).", "the club's size in the game, from 0 to 10000 (a European giant is above 9000; a local club is near 1000)."),
            ("Alvo", "Target", "a reputação de clube que combina com o jovem: o PA dele vezes a regra do alvo (com 50, um PA 150 mira um clube de 7500). A linha amarela do gráfico.", "the club reputation that suits the youngster: his PA times the target rule (with 50, a PA 150 aims at a 7500 club). The yellow line on the chart."),
            ("Nível", "Level", "o quanto o clube está perto do alvo, de 0 a 1: a curva em sino do gráfico. No alvo vale 1; longe dele, cai.", "how close the club is to the target, from 0 to 1: the bell curve on the chart. At the target it's 1; far from it, it drops."),
            ("Jovem de exemplo", "Example youngster", "um jovem imaginário, só para os gráficos: mexa no PA e no CA dele para ver para onde a regra mandaria um jovem assim. Não muda o jogo nem o tweaks.cfg.", "an imaginary youngster, only for the charts: move his PA and CA to see where the rule would send a youngster like him. It changes neither the game nor tweaks.cfg."),
            ("Clubes de exemplo", "Example clubs", "clubes imaginários de vários tamanhos (os pontos azuis do gráfico e a lista de chances), para comparar. No jogo entram os clubes de verdade do país do jovem.", "imaginary clubs of several sizes (the blue dots on the chart and the list of chances), for comparison. In the game, the real clubs of the youngster's country take part."),
            ("Surpresa", "Surprise", "uma parte dos jovens é sorteada sem olhar o nível do clube: o craque que surge num clube pequeno, como o Ronaldo no São Cristóvão.", "a share of youngsters is drawn without looking at the club's level: the star who turns up at a small club, like Ronaldo at São Cristóvão."),
        ],
        Aba::Inativos => &[
            ("Inativo", "Idle player", "jogador sem clube há muito tempo, que nenhum clube contrata.", "a player without a club for a long time, whom no club signs."),
            ("Verificação", "Check", "o jogo testa as aposentadorias quatro vezes por ano (dias 0, 80, 173 e 250). A chance do gráfico é por verificação.", "the game tests retirements four times a year (days 0, 80, 173 and 250). The chart's chance is per check."),
            ("Reputação", "Reputation", "a fama do jogador, de 0 a 10000. Quem tem reputação baixa se aposenta mais fácil.", "the player's fame, from 0 to 10000. Players with low reputation retire more easily."),
            ("Reputação de exemplo", "Example reputation", "um jogador imaginário, só para o gráfico: mude a reputação para ver a chance de aposentar por idade. Não muda o jogo.", "an imaginary player, only for the chart: change the reputation to see the chance of retiring by age. It doesn't change the game."),
            ("Regra original", "Original rule", "a linha tracejada: no 3.9.68, depois de 365 dias sem clube, a chance é 750 / reputação, em qualquer idade.", "the dashed line: in 3.9.68, after 365 days without a club, the chance is 750 / reputation, at any age."),
        ],
        Aba::Registro => &[
            ("Vaga", "Slot", "a pessoa aposentada que o jogo reaproveita para o regen: dela vêm o id e a personalidade. O nome, a idade e a nação mudam.", "the retired person the game reuses for the regen: the id and personality come from him. The name, age and nation change."),
            ("Ficha de jogador", "Player record", "a ficha de outro aposentado, guardada numa reserva: dela vêm o PA, as posições, os pés e os atributos. É o \"de quem é\" o regen: o regen do Ronaldo é quem recebe a ficha do Ronaldo.", "another retiree's record, kept in a pool: the PA, positions, feet and attributes come from it. It's \"whose\" the regen is: Ronaldo's regen is whoever gets Ronaldo's record."),
            ("regen_origins.csv", "regen_origins.csv", "o histórico, ao lado do exe: uma linha por regen, separada por ponto e vírgula. Abre no Excel com dois cliques e no Bloco de Notas. Para compartilhar, mande o arquivo.", "the history, next to the exe: one line per regen, separated by semicolons. It opens in Excel with a double click and in Notepad. To share it, send the file."),
            ("X.regens", "X.regens", "ao gravar o jogo, a DLL grava ao lado do save (X.sav) os donos das fichas que ainda estão na reserva, e o lê ao carregar. Copie o save e o .regens juntos. Se o .regens não for dessa gravação (o save regravado com o registro desligado, ou um .regens de outro save), a DLL o ignora, para não dar dono errado.", "when you save the game, the DLL writes next to the save (X.sav) the owners of the records still in the pool, and reads it on load. Copy the save and the .regens together. If the .regens isn't from that save (the save re-saved with recording off, or a .regens from another save), the DLL ignores it, so no owner comes out wrong."),
        ],
        Aba::Jogo => &[
            ("Sobrescrever as mudanças do exe", "Override the exe's changes", "quando o exe já tem uma alteração (do NickPatcher, do Regen Tweak) no mesmo ponto de um ajuste, o normal é ficar com a do exe. Ligado, vale a desta DLL, e aquela opção do exe deixa de valer.", "when the exe already has a change (from NickPatcher, the Regen Tweak) where a tweak hooks in, the exe's stays by default. On, this DLL's applies, and that exe option stops working."),
            ("SetupReg", "SetupReg", "o programa que grava no registro do Windows onde o jogo está instalado. Com a pasta portátil, o jogo não precisa dele.", "the program that writes in the Windows registry where the game is installed. With the portable folder, the game doesn't need it."),
        ],
        Aba::Nomes => &[
            ("Pegar emprestado", "Borrowing", "quando o país tem 100 nomes ou menos no banco, o jogo não usa os dele: tira o nome de um país de referência da mesma língua (por exemplo, da Espanha para os de língua espanhola). Alguns países não têm país de referência: o jogo sorteia 40 vezes entre todos os países atrás de um da mesma língua e, se não achar, fica com um país qualquer. A lista abaixo mostra, país por país, de onde vêm hoje.", "when a country has 100 names or fewer in the database, the game doesn't use its own: it takes the name from a reference country of the same language (for example, from Spain for Spanish-speaking ones). Some countries have no reference country: the game draws 40 times among all countries looking for one of the same language and, if it finds none, keeps any country. The list below shows, country by country, where they come from today."),
            ("Pessoas novas", "New people", "os regens, e também a comissão e os árbitros que o jogo cria: todos passam pela mesma regra de nomes.", "regens, and also the staff and referees the game creates: they all go through the same name rule."),
        ],
        Aba::Cinzentos => &[
            ("Jogador cinzento", "Grey player", "um jogador virtual que o jogo cria só para completar o elenco: aparece em cinza, não tem contrato e não pode ser contratado. Some quando o clube volta a ter jogadores.", "a virtual player the game creates only to fill the squad: shown in grey, with no contract, and nobody can sign him. He goes away when the club has players again."),
            ("CA", "CA", "Current Ability, a habilidade atual (1 a 200): o quanto o jogador joga hoje.", "Current Ability (1 to 200): how well the player plays today."),
            ("Nível do clube", "Club's level", "o maior entre dois valores: a média de CA dos 16 melhores jogadores de verdade do elenco (com pelo menos 5), e uma conta pela reputação que sobe com ela e nunca fica negativa (reputação × 0,016 + 20; seleção, reputação / 100 + 40). A reputação segura o clube grande de uma liga que você não carregou, que tem poucos jogadores no banco.", "the higher of two values: the average CA of the squad's 16 best real players (with at least 5), and a formula from the reputation that rises with it and is never negative (reputation × 0.016 + 20; national team, reputation / 100 + 40). The reputation keeps a big club from a league you didn't load strong, since it has few players in the database."),
            ("Variação", "Variation", "quanto o CA de cada cinzento pode ficar acima ou abaixo do nível do clube, sorteado para cada um.", "how far each grey player's CA can be above or below the club's level, drawn for each one."),
        ],
        Aba::Fornada => &[
            ("Estoque", "Stock", "as pessoas a mais que a DLL cria com a carreira nova, adormecidas até a fornada. Cada fornada gasta umas 750 (3 por clube, 278 clubes); com o estoque vazio, a fornada para e o jogo segue normal.", "the extra people the DLL creates with a new career, asleep until the intake. Each intake uses about 750 (3 per club, 278 clubs); with the stock empty, the intake stops and the game goes on as normal."),
            ("Fim da temporada", "Season end", "o dia em que o jogo fecha a temporada de cada nação (a mesma data das finanças e das obras do estádio). Na Inglaterra é perto de junho; no Brasil, de dezembro.", "the day the game closes each nation's season (the same date as finances and stadium works). In England it's around June; in Brazil, around December."),
        ],
        Aba::Cfg | Aba::Log => &[],
    }
}
