## Parsiranje argumenata
  - `init` `-n[ame]` ime projekta, default [DIR_NAME]; -p[ath] path do roota projekta, default [CWD]; -l[ocal] kreiraj jari projekat lokalno umesto globalno u $XDG_DATA_HOME
  - task
  
      show -> (default ako se ne upise nista nakon task) prikazi task i informacije vezane za njega; -n[ame] [NAME ili project_prefix_num] prikazi task i informacije vezane za njega; -s[tatus] "STATUS" promeni status taska; -e[dit] izmeni task, moras mu proslediti bar jedan od [-n[ame], -d[escription], -s[tatus]]; -ei (interaktivni edit, da li moze?)
      
      new -> pravi novi task; -n[ame] sa imenom; -i[nteractive] pravi ga interaktivno, prvo uneses name pa onda description, pa status
      
      list -> izlistaj sve taskove u projektu; -s[tyle] specificiraj kako da se prikaze lista
        moguce vrednosti = [i[nteractive], b[rowser], t[erm]], gde i[nteractive] kao tui; b[rowser] otvori kao html stranicu u $BROWSER; t[erm] pukni sve na stdout (pager? pagination?);
        default = term, podesivo u konfig fajlu; -f[ilter] filtriraj taskove (pogledaj export)
        
      extract -> izvuci sve taskove iz projekta rekurzivno na dole; -f specificiraj fajl za ekstrakciju;
      
      delete -> prima name i brise task iz baze ako postoji
  - export -> eksportuj taskove u neki format; -f[ormat] = [json, yaml, csv, neki treci kurac]; -p[roject] ime projekta iz kog se eksportuje, default [current_proj]; --filter (includes str, od-do po brojevima, od-do po datumu kreacije, od-do po datumu promene statusa, status, bice posla)
  
  
## Config i prikljucenija
  - treba nam konfig fajl u rootu projekta
    ### [project]
        - name: ime projekta
        - prefix_style: prefiks za task name, moguce vrednosti ["prefix_num", "prefix", "num", "off"] default "PROJECT_NAME-NUM:"
        - prefix: koji prefiks koristiti ako se koristi za taskove
        - numbering: {start, step} -> odakle da krene i kako da inkrementira NUM, defaults: 0, 1
        - status: [TODO, IN_PROGRESS, TESTING, DONE], podesivo ofc, pomaze kod autocomplete, mozda se implementira status toggling da ide napred nazad
        - extraction_keywords: keywordi koje trazimo da bismo izvukli taskove iz fajlova (default TODO, FIXME)
        - comment_symbol: simboli kojima pocinju komentari; default = infer iz ekstenzije fajla
    ### [ignores] 
        - ignorovanje foldera/fajlova, ako ima .gitignore onda ignorisi ta mesta, ako nema treba imati sane defaults
        - template? (mozda imati template za jezike/frejmvorke [spring boot, react, .idea, drugi kurci])
    ### [display] 
        - style = kako prikazati taskove kada se pozove list [tui, html, cli], default: cli za pocetak, posle tui
        - context = {pre, post} koliko linija konteksta prikazati prilikom pozivanja list ili task, default npr. {5, 5}
        - empty_lines = da li prikazati prazne linije u kontekstu, true/false
  - treba nam db fajl (sqlite3) u $XDG_DATA_HOME/.jari
  - treba nam globalni konfig u $XDG_CONFIG_HOME/.jari koji ce biti overwrite-ovan lokalnim

## DB
  - project tabela: id, name, path;
  - task tabela: id, proj_id, name, path, file_name, created_at, status, status_change_date, context -> (par linija pre i posle mesta sa kog je izvucen task)
    
