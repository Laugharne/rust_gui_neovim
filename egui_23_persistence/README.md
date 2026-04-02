
# 💾 Projet : egui Persistence (Paramètres & Sauvegarde)

Ce tutoriel (épisode 23) explique comment rendre une application capable de "se souvenir" de l'état de l'utilisateur (mode sombre, nom d'utilisateur, volume, etc.) même après avoir été fermée et rouverte.

---

## 🎥 Résumé de la Vidéo

L'objectif est de créer une application de réglages dont les valeurs sont automatiquement écrites sur le disque dur et rechargées au démarrage.

### Concepts Clés de la Persistance :
- **Sérialisation & Désérialisation** : Utilisation de `serde` pour transformer la structure Rust en un format stockable (comme du JSON ou du binaire) [[00:23](http://www.youtube.com/watch?v=DoIvFrc13A4&t=23)].
- **Trait `serde::Serialize` / `Deserialize`** : Macros à dériver sur votre structure d'application pour activer la sauvegarde [[07:29](http://www.youtube.com/watch?v=DoIvFrc13A4&t=449)].
- **`eframe::get_value`** : Fonction utilisée au démarrage pour tenter de lire les données sauvegardées dans le stockage local [[03:01](http://www.youtube.com/watch?v=DoIvFrc13A4&t=181)].
- **`eframe::set_value`** : Fonction qui écrit les données actuelles vers le stockage [[03:07](http://www.youtube.com/watch?v=DoIvFrc13A4&t=187)].
- **Méthode `save()`** : Une fonction du trait `App` qui est appelée automatiquement par eframe (souvent lors de la fermeture ou à intervalles réguliers) pour persister l'état [[03:23](http://www.youtube.com/watch?v=DoIvFrc13A4&t=203)].

---

## 💻 Structure du Code (GitHub)

Le code montre comment intégrer `serde` au cycle de vie d'une application `eframe`.

### 1. Dépendances nécessaires (`Cargo.toml`)
Il est crucial d'activer les "features" spécifiques pour que la persistance fonctionne :
```toml
[dependencies]
eframe = { version = "0.31", features = ["persistence"] }
serde  = { version = "1",    features = ["derive"] }
```

### 2. La Structure de Données (`app.rs`)
La structure `MyApp` contient tous les réglages de l'utilisateur. Notez l'utilisation des attributs `derive`.

```rust
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // Utilise les valeurs par défaut si un champ est manquant dans la sauvegarde
pub struct MyApp {
    username     : String,
    dark_mode    : bool,
    font_size    : f32,
    notifications: bool,
    volume       : f32,
}
```

### 3. Cycle de Vie : Chargement et Sauvegarde

Le flux de données suit un cycle précis pour garantir que l'utilisateur retrouve ses réglages.


| Étape           | Moment                    | Fonction utilisée                                                                                                              |
| :-------------- | :------------------------ | :----------------------------------------------------------------------------------------------------------------------------- |
| **Chargement**  | Au démarrage (`new`)      | `eframe::get_value(storage, eframe::APP_KEY)` [[03:01](http://www.youtube.com/watch?v=DoIvFrc13A4&t=181)]                      |
| **Mise à jour** | À chaque frame (`update`) | Les widgets modifient directement les champs de `self`.                                                                        |
| **Sauvegarde**  | Fermeture / Auto-save     | `storage.set_string(eframe::APP_KEY, ...)` via la méthode `save()` [[03:33](http://www.youtube.com/watch?v=DoIvFrc13A4&t=213)] |


---

## 🛠️ Implémentation Technique

### Initialisation (`fn new`)
Dans la fonction `new`, on utilise le `CreationContext` (`cc`) pour accéder au stockage :
- Si des données existent, on les charge.
- Sinon, on utilise `MyApp::default()` [[03:13](http://www.youtube.com/watch?v=DoIvFrc13A4&t=193)].

### Interface Utilisateur
Le tutoriel utilise un **`egui::Grid`** pour aligner proprement les labels et les contrôles (sliders, checkboxes, text inputs) [[04:10](http://www.youtube.com/watch?v=DoIvFrc13A4&t=250)].

### Bouton de Réinitialisation
Le code inclut un bouton "Reset" qui remplace simplement l'état actuel par les valeurs par défaut :
`*self = MyApp::default();` [[05:27](http://www.youtube.com/watch?v=DoIvFrc13A4&t=327)].

### 🔍 Détails techniques sur le stockage

* **Format du fichier :** Par défaut, `eframe` avec `persistence` utilise souvent le format **JSON** ou un format binaire compact (via `ron` ou `bincode`), selon la configuration. Dans la vidéo, comme `serde` est utilisé, c'est lisible si c'est du JSON.
* **La Clé d'accès :** Dans le code, vous voyez souvent `eframe::APP_KEY`. C'est l'identifiant (le nom du fichier ou de la clé) utilisé pour retrouver vos données spécifiques parmi d'autres applications.
* **Le rôle de `NativeOptions` :** Le dossier parent porte généralement le nom que vous avez défini dans `NativeOptions { viewport: ViewportBuilder::default().with_title("Mon App"), .. }`.

### 📂 Emplacements par Système d'Exploitation

| Système d'Exploitation | Chemin Standard (Généralement)                                |
| :--------------------- | :------------------------------------------------------------ |
| **Windows**            | `%USERPROFILE%\AppData\Roaming\[Nom_App]\data.json`           |
| **macOS**              | `~/Library/Application Support/[Nom_App]/data.json`           |
| **Linux**              | `~/.local/share/[Nom_App]/data.json`                          |
| **Web (WASM)**         | Stocké dans le **Local Storage** du navigateur (lié à l'URL). |

---

### 💡 Astuce pour le développement
Si vous voulez "réinitialiser" totalement votre application pendant vos tests :
1.  Localisez le dossier mentionné ci-dessus.
2.  Supprimez le fichier de données (souvent dans un sous-dossier `storage` ou portant le nom de votre binaire).
3.  Relancez l'application : elle chargera alors les valeurs de `MyApp::default()`.

---

## 🔗 Liens Utiles avec Timestamps
- **[[01:19](http://www.youtube.com/watch?v=DoIvFrc13A4&t=79)]** : Configuration de `Cargo.toml` avec les features de persistance.
- **[[02:19](http://www.youtube.com/watch?v=DoIvFrc13A4&t=139)]** : Déclaration de la structure avec `Serialize` et `Deserialize`.
- **[[03:01](http://www.youtube.com/watch?v=DoIvFrc13A4&t=181)]** : Logique de récupération des données au lancement.
- **[[03:33](http://www.youtube.com/watch?v=DoIvFrc13A4&t=213)]** : Implémentation de la méthode `save` pour l'auto-sauvegarde.
- **[[06:44](http://www.youtube.com/watch?v=DoIvFrc13A4&t=404)]** : Démonstration du changement de thème (Dark Mode) persistant.

**Conclusion :** Grâce à `eframe` et `serde`, la persistance devient presque transparente. Il suffit de définir ses données et de laisser le framework gérer l'écriture sur le disque (souvent dans un dossier caché spécifique à l'OS).

