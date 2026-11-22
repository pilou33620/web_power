//! Le serveur de GridMaster : sert la page (index.html, css/, js/, graphismes/) au réseau local,
//! derrière la garde commune des web tools. La partie s'enregistre dans le navigateur : le serveur
//! n'a aucune route qui écrit.
//!
//! Lancement :  power [--port 8794] [--local] [--sans-navigateur] [--sans-pause] [--sans-maj]

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use web_tools_commun::appairage::{self, Garde};
use web_tools_commun::production;
use web_tools_commun::serveur::{decoder_chemin, Reponse, Requete, Serveur};

pub const PORT: u16 = 8794;

/// La racine du jeu (index.html, js/) : le dossier courant s'il la contient, sinon le premier
/// dossier au-dessus de l'exécutable qui la contient.
pub fn racine() -> &'static Path {
    static R: OnceLock<PathBuf> = OnceLock::new();
    R.get_or_init(|| {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let jeu = |d: &Path| d.join("index.html").is_file() && d.join("js").is_dir();
        if jeu(&cwd) {
            return cwd;
        }
        let exe = std::env::current_exe().ok().and_then(|e| std::fs::canonicalize(e).ok());
        exe.and_then(|e| e.ancestors().find(|d| jeu(d)).map(Path::to_path_buf)).unwrap_or(cwd)
    })
}

/// Liste blanche (le chemin déjà décodé) : la page, ses styles, ses scripts et ceux de la
/// bibliothèque graphique (moteur et three.js) ; ni .git/, ni les sources, ni le banc.
/// Rend le chemin relatif du fichier à servir.
fn statique(chemin: &str) -> Option<String> {
    let c = chemin.strip_prefix('/')?;
    if c.is_empty() {
        return Some("index.html".into());
    }
    // un nom de lettres, chiffres, _ et - (et . pour three.js), jamais commencé par un point
    let mot = |s: &str, point: bool| {
        !s.is_empty() && !s.starts_with('.') && s.chars().all(|x| x.is_alphanumeric() || x == '_' || x == '-' || (point && x == '.'))
    };
    let (dossier, nom) = c.rsplit_once('/').unwrap_or(("", c));
    let ok = match dossier {
        "" => nom == "index.html",
        "css" => nom.strip_suffix(".css").is_some_and(|b| mot(b, false)),
        "js" | "graphismes/rendu" => nom.strip_suffix(".js").is_some_and(|b| mot(b, false)),
        "graphismes/three" | "graphismes/three/addons/utils" => nom.strip_suffix(".js").is_some_and(|b| mot(b, true)),
        _ => false,
    };
    ok.then(|| c.to_string())
}

/// nosniff : sans le bon type, le navigateur refuse le fichier ; une extension hors de la table est un 404.
fn type_mime(rel: &str) -> Option<&'static str> {
    match rel.rsplit_once('.')?.1 {
        "html" => Some("text/html; charset=utf-8"),
        "css" => Some("text/css; charset=utf-8"),
        "js" => Some("text/javascript; charset=utf-8"),
        _ => None,
    }
}

fn fichier(chemin: &str) -> Reponse {
    let Some((rel, mime)) = statique(&decoder_chemin(chemin)).and_then(|r| type_mime(&r).map(|m| (r, m))) else {
        return Reponse::texte(404, "File not found");
    };
    match std::fs::read(racine().join(&rel)) {
        Ok(o) => Reponse::new(200, mime, o),
        Err(_) => Reponse::texte(404, "File not found"),
    }
}

fn garde() -> &'static Garde {
    GARDE.get().expect("garde non initialisée")
}
static GARDE: OnceLock<Garde> = OnceLock::new();

fn servir(r: &mut Requete) -> Reponse {
    // Host, Internet, TLS, jeton : avant tout traitement de la requête
    if let Some(rep) = garde().filtrer(r) {
        return rep;
    }
    if !r.lecture() {
        return Reponse::texte(501, "Unsupported method");
    }
    fichier(r.chemin())
}

/// Les en-têtes de toutes les réponses (X-Frame-Options : déjà posé par le serveur commun).
fn entetes(mut rep: Reponse) -> Reponse {
    rep.entetes.retain(|(k, _)| !k.eq_ignore_ascii_case("Cache-Control"));
    rep.entete("Cache-Control", "no-store, max-age=0")
        .entete("Content-Security-Policy", "frame-ancestors 'self'")
        .entete("X-Content-Type-Options", "nosniff")
}

// ---------------------------------------------------------------------- lancement

#[derive(Debug, Default, PartialEq)]
pub struct Args {
    pub port: u16,
    pub local: bool,
    pub sans_navigateur: bool,
    /// Sans effet ici : demarrer_WEB_POWER.cmd garde la fenêtre ouverte sur une erreur sous Windows.
    pub sans_pause: bool,
    pub sans_maj: bool,
    pub aide: bool,
}

pub const USAGE: &str = "usage: power [--port 8794] [--local] [--sans-navigateur] [--sans-pause] [--sans-maj]";

pub fn lire_args(argv: &[String]) -> Result<Args, String> {
    let mut a = Args { port: PORT, ..Default::default() };
    let mut i = 0;
    while i < argv.len() {
        let x = argv[i].as_str();
        let (cle, val) = match x.split_once('=') {
            Some((k, v)) => (k, Some(v.to_string())),
            None => (x, None),
        };
        match cle {
            "--port" => {
                let v = match val {
                    Some(v) => v,
                    None => {
                        i += 1;
                        argv.get(i).cloned().ok_or("--port attend un nombre")?
                    }
                };
                a.port = v.parse().map_err(|_| format!("port invalide : {v}"))?;
            }
            "--local" if val.is_none() => a.local = true,
            "--sans-navigateur" if val.is_none() => a.sans_navigateur = true,
            "--sans-pause" if val.is_none() => a.sans_pause = true,
            "--sans-maj" if val.is_none() => a.sans_maj = true,
            "-h" | "--help" if val.is_none() => a.aide = true,
            _ => return Err(format!("option inconnue : {x}")),
        }
        i += 1;
    }
    Ok(a)
}

fn ouvrir_navigateur(url: &str) {
    let r = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", "", url]).stdout(Stdio::null()).stderr(Stdio::null()).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(url).spawn()
    } else {
        Command::new("xdg-open").arg(url).stdout(Stdio::null()).stderr(Stdio::null()).spawn()
    };
    if r.is_err() {
        println!("Ouvrez {url} dans votre navigateur.");
    }
}

/// git dans le dossier du jeu, jamais d'invite, tué au bout de `delai` secondes.
fn git(args: &[&str], delai: u64) -> Result<String, String> {
    let mut proc = Command::new("git")
        .args(args)
        .current_dir(racine())
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    // lus à part : un tuyau plein bloquerait git
    let lire = |flux: Option<Box<dyn Read + Send>>| {
        std::thread::spawn(move || {
            let mut tout = Vec::new();
            if let Some(mut f) = flux {
                let _ = f.read_to_end(&mut tout);
            }
            String::from_utf8_lossy(&tout).trim().to_string()
        })
    };
    let sortie = lire(proc.stdout.take().map(|f| Box::new(f) as Box<dyn Read + Send>));
    let erreur = lire(proc.stderr.take().map(|f| Box::new(f) as Box<dyn Read + Send>));
    let fin = Instant::now() + Duration::from_secs(delai);
    let statut = loop {
        match proc.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) if Instant::now() < fin => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = proc.kill();
                let _ = proc.wait();
                // (les fils de lecture ne sont pas attendus : un sous-processus de git peut tenir le tuyau)
                return Err(format!("délai de {delai} s dépassé"));
            }
        }
    };
    let (o, e) = (sortie.join().unwrap_or_default(), erreur.join().unwrap_or_default());
    if statut.success() {
        Ok(o)
    } else {
        Err(if e.is_empty() { o } else { e })
    }
}

/// Mise à jour par git (avance rapide seulement, modifications locales mises en réserve puis
/// réappliquées). Rend vrai si le code a changé ; tout échec mène au démarrage normal.
fn mise_a_jour() -> bool {
    // copie de production (archive sans git ni sources) : mise à jour depuis la publication « binaires »
    if production::est_copie_prod(racine()) {
        return production::mettre_a_jour("https://github.com/pilou33620/web_power", "power", racine(), &|t| println!("[*] {t}"))
            .unwrap_or_else(|e| {
                println!("[X] {e}");
                false
            });
    }
    if git(&["rev-parse", "--is-inside-work-tree"], 5).is_err() {
        return false; // pas de git, ou pas un dépôt
    }
    println!("[*] Recherche de mises à jour sur GitHub...");
    if let Err(e) = git(&["fetch", "--quiet", "origin"], 10) {
        println!("[!] GitHub injoignable ({e}).\n[*] Démarrage normal de l'outil...");
        return false;
    }
    let amont = if git(&["rev-parse", "--verify", "@{u}"], 5).is_ok() {
        "@{u}"
    } else if git(&["rev-parse", "--verify", "origin/main"], 5).is_ok() {
        "origin/main"
    } else {
        return false;
    };
    let plage = format!("HEAD..{amont}");
    let n: u32 = match git(&["rev-list", plage.as_str(), "--count"], 5).ok().and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return false,
    };
    if n == 0 {
        println!("[*] Le logiciel est à jour.");
        return false;
    }
    println!("[*] Mise à jour disponible ({n} nouveau(x) commit(s) sur GitHub) : application...");
    let modifie = git(&["status", "--porcelain", "-uno"], 5).is_ok_and(|s| !s.is_empty());
    let reserve = modifie && {
        println!("  Modifications locales détectées : mise en réserve temporaire...");
        git(&["stash", "push", "-m", "Auto-stash WEB_POWER avant mise a jour"], 10).is_ok()
    };
    // --ff-only : un historique distant réécrit ou divergent est refusé, jamais fusionné en silence
    let fusion = git(&["merge", "--ff-only", amont], 120);
    if reserve {
        if let Err(e) = git(&["stash", "pop"], 10) {
            println!("[!] Vos modifications locales n'ont pas pu être réappliquées automatiquement :\n    {e}");
            println!("    Elles sont conservées dans la réserve Git : voir « git stash list », puis « git stash pop » une fois le conflit résolu.");
        }
    }
    match fusion {
        Ok(_) => {
            println!("[*] Mise à jour appliquée.");
            true
        }
        Err(e) => {
            println!("[!] Échec de l'application de la mise à jour : {e}\n[*] Poursuite avec la version actuelle...");
            false
        }
    }
}

pub fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match lire_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{USAGE}\npower: {e}");
            std::process::exit(2);
        }
    };
    if args.aide {
        println!("{USAGE}");
        return;
    }
    let _ = std::env::set_current_dir(racine());
    let reseau = !args.local;
    let srv = match Serveur::ouvrir(args.port, reseau, 10) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Aucun port libre entre {} et {} : {e}", args.port, args.port.saturating_add(9));
            std::process::exit(1);
        }
    };
    if !args.sans_maj && mise_a_jour() {
        // demarrer_WEB_POWER pose cette variable : il reprend l'exécutable de la nouvelle version et relance
        if std::env::var_os("WEB_POWER_RELANCEUR").is_some() {
            println!("[*] Redémarrage avec la nouvelle version...");
            std::process::exit(75);
        }
        println!("[*] Nouvelle version téléchargée : relancez GridMaster pour en profiter.");
    }
    let _ = GARDE.set(Garde::new("WEB_POWER", reseau, None, None));
    garde().brancher(&srv); // en mode réseau : HTTPS sur le même port
    let port = srv.port();
    let url = format!("http://127.0.0.1:{port}/");
    println!("GridMaster");
    println!("  adresse : {url}");
    if port != args.port {
        println!("  (le port {} était occupé)", args.port);
    }
    if reseau {
        for ip in appairage::adresses_poste().iter().filter(|ip| !ip.starts_with("127.")) {
            println!("  réseau  : {}   (tablette, autre poste)", garde().adresse(ip, port));
        }
        println!("            l'autre appareil demande une fois le code d'appairage ci-dessous");
        garde().annoncer();
    }
    println!("  Ctrl+C pour arrêter.\n");
    let _ = std::io::stdout().flush();
    if !args.sans_navigateur {
        ouvrir_navigateur(&url);
    }
    srv.servir(Arc::new(|r: &mut Requete| entetes(servir(r))));
}

#[cfg(test)]
mod essais {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    fn a(l: &[&str]) -> Vec<String> {
        l.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn ligne_de_commande() {
        // la commande du catalogue de web_launcher
        let x = lire_args(&a(&["--port", "9000", "--sans-navigateur", "--sans-pause", "--sans-maj"])).unwrap();
        assert_eq!(x, Args { port: 9000, sans_navigateur: true, sans_pause: true, sans_maj: true, ..Default::default() });
        assert_eq!(lire_args(&a(&["--port=8800", "--local"])).unwrap(), Args { port: 8800, local: true, ..Default::default() });
        assert_eq!(lire_args(&[]).unwrap().port, PORT);
        assert!(lire_args(&a(&["--help"])).unwrap().aide);
        for refuse in [&["--port"][..], &["--port", "x"], &["--inconnu"], &["--local=1"]] {
            assert!(lire_args(&a(refuse)).is_err(), "{refuse:?}");
        }
    }

    fn demander(port: u16, methode: &str, chemin: &str, hote: &str) -> String {
        let mut t = TcpStream::connect(("127.0.0.1", port)).unwrap();
        write!(t, "{methode} {chemin} HTTP/1.0\r\nHost: {hote}\r\nContent-Length: 0\r\n\r\n").unwrap();
        let mut rep = Vec::new();
        let _ = t.read_to_end(&mut rep);
        String::from_utf8_lossy(&rep).into_owned()
    }

    fn statut(rep: &str) -> u16 {
        rep.split(' ').nth(1).and_then(|c| c.parse().ok()).unwrap_or(0)
    }

    /// Le banc de l'ancien serveur Python (test/banc-serveur.py) : ce qu'il sert, ce qu'il refuse.
    #[test]
    fn fichiers_servis_et_refuses() {
        let _ = GARDE.set(Garde::new("WEB_POWER", false, None, None));
        let srv = Serveur::ouvrir(0, false, 1).unwrap();
        let port = srv.port();
        srv.servir_en_fond(Arc::new(|r: &mut Requete| entetes(servir(r))));
        let hote = format!("127.0.0.1:{port}");
        let s = |m: &str, c: &str| statut(&demander(port, m, c, &hote));
        for c in [
            "/", "/index.html", "/css/style.css", "/js/00-appareil.js", "/js/25-boucle.js", "/js/01-moteur.js",
            "/graphismes/rendu/scene.js", "/graphismes/rendu/modernes.js", "/graphismes/three/three.module.js",
            "/graphismes/three/three.core.js", "/graphismes/three/addons/utils/BufferGeometryUtils.js",
        ] {
            assert_eq!(s("GET", c), 200, "{c}");
        }
        for c in [
            "/.git/config", "/.git/HEAD", "/serveur.py", "/appairage.py", "/README.md", "/banc/banc.js", "/.gitignore",
            "/../README.md", "/%2e%2e/README.md", "/index.html/", "/banc/", "/js/", "/js/../serveur.py",
            "/js/%2e%2e/serveur.py", "/css/../appairage.py", "/js/inconnu.js", "/test/banc-serveur.py",
            "/graphismes/VERSION.json", "/graphismes/README.md", "/graphismes/royaumes/scene.js",
            "/graphismes/power/01-outils.js", "/graphismes/rendu/../../serveur.py", "/graphismes/rendu/rendu.css",
            "/graphismes/three/LICENSE", "/graphismes/three/VERSION", "/graphismes/three/../../serveur.py",
            "/graphismes/rendu/sous/x.js", "/Cargo.toml", "/rust/power/src/serveur.rs", "/js/.cache.js",
        ] {
            assert_eq!(s("GET", c), 404, "{c}");
        }
        assert_eq!(statut(&demander(port, "GET", "/", "piege.example")), 403, "Host inconnu");
        assert_eq!(s("POST", "/"), 501, "POST");
        assert_eq!(s("PUT", "/index.html"), 501, "PUT");
        let rep = demander(port, "HEAD", "/js/01-moteur.js", &hote);
        assert_eq!(statut(&rep), 200);
        for e in [
            "Content-Type: text/javascript; charset=utf-8",
            "Cache-Control: no-store, max-age=0",
            "Content-Security-Policy: frame-ancestors 'self'",
            "X-Content-Type-Options: nosniff",
            "X-Frame-Options: SAMEORIGIN",
        ] {
            assert!(rep.contains(e), "{e}");
        }
        assert!(!rep.contains("Cache-Control: no-store\r\n"), "un seul Cache-Control");
        // chaque type de la table : 200 avec son Content-Type
        for (c, t) in [("/", "text/html"), ("/css/style.css", "text/css"), ("/js/01-moteur.js", "text/javascript")] {
            let rep = demander(port, "GET", c, &hote);
            assert_eq!(statut(&rep), 200, "{c}");
            assert!(rep.contains(&format!("Content-Type: {t}; charset=utf-8\r\n")), "{c}");
        }
        for inconnu in ["LICENSE", "VERSION.json", "rendu.png", "a.exe", "a.JS"] {
            assert_eq!(type_mime(inconnu), None, "{inconnu}");
        }
        srv.arreter();
    }
}
