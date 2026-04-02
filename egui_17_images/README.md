
# 🖼️ Projet : Galerie d'Images avec egui

 [Display & Scale Images in egui — Rust GUI #17 - YouTube](https://www.youtube.com/watch?v=mEpJRgYSYWE)

![](assets/2026-04-02-13-27-56.png)

Ce tutoriel (épisode 17 de la série) enseigne comment intégrer, afficher et manipuler des images dans une application Rust en utilisant le framework **egui**.

---

## 🎥 Résumé de la Vidéo

L'objectif est de construire une application de galerie fonctionnelle comprenant une barre latérale de vignettes (thumbnails) et une zone de prévisualisation avec zoom.

### Concepts Clés abordés :
- **Installation des loaders** : Utilisation de `egui_extras::install_image_loaders` pour permettre au framework de décoder des formats comme le PNG ou le JPEG.
- **Inclusion à la compilation** : Utilisation de la macro `include_image!` pour intégrer les fichiers images directement dans le binaire de l'application.
- **Manipulation d'images** :
    - `max_width` et `corner_radius` pour le style des vignettes.
    - `sense(Sense::click())` pour rendre une image interactive.
    - `fit_to_exact_size` pour gérer dynamiquement le zoom de l'image principale.

---

## 💻 Structure du Code (GitHub)

Le projet adopte une structure propre en séparant la logique de données de la logique d'affichage. Voici comment le code est organisé pour rester maintenable :


### 1. Architecture des Modules

Le fichier `main.rs` n'est qu'un chef d'orchestre. Il déclare les modules et lance l'application. La véritable intelligence se trouve ici :

- **`src/app.rs`** : C'est le "Cœur". Il contient la structure `MyApp` qui stocke les données (chemins des images) et l'état (quelle image est sélectionnée, quel est le zoom).
- **`src/sidebar.rs`** : Dédié uniquement au rendu de la colonne de gauche.
- **`src/preview.rs`** : Dédié uniquement au rendu de la zone d'affichage principale.


| Fichier      | Rôle Principal                                                                                      |
| :----------- | :-------------------------------------------------------------------------------------------------- |
| `main.rs`    | Point d'entrée, configuration de la fenêtre et initialisation des loaders d'images.                 |
| `app.rs`     | Définit la structure de données `MyApp` et gère l'état global (image sélectionnée, niveau de zoom). |
| `sidebar.rs` | Gère l'affichage de la colonne de gauche contenant les vignettes cliquables.                        |
| `preview.rs` | Gère la zone centrale avec le curseur de zoom et l'affichage de l'image en grand.                   |


### 2. Le concept de `pub(crate)`
Une particularité intéressante de ce code est l'utilisation de `pub(crate)` sur les champs de la structure `MyApp`.

```rust
pub struct MyApp {
    pub(crate) selected_index: usize,
    pub(crate) zoom_level:     f32,
}
```
- **Pourquoi ?** Cela permet aux fichiers `sidebar.rs` et `preview.rs` de lire et modifier directement l'index de l'image ou le zoom sans avoir à passer par des fonctions complexes (getters/setters), tout en gardant ces champs invisibles pour l'extérieur du projet.


### 3. Flux de données (Data Flow)
Le projet suit un modèle de mise à jour simple :

1.  **Entrée** : L'utilisateur clique sur une vignette dans `sidebar.rs`.
2.  **Mutation** : La variable `selected_index` dans `MyApp` est mise à jour.
3.  **Réaction** : Au cycle suivant (60 FPS), `preview.rs` lit le nouvel index et affiche instantanément la nouvelle image correspondante via `include_image!`.


### 4. Gestion de la mémoire et compilation
L'utilisation de la macro `include_image!` est cruciale dans la structure :
- Les images ne sont pas chargées depuis le disque dur au moment du clic (ce qui pourrait causer un lag).
- Elles sont **intégrées directement dans l'exécutable** lors de la compilation. Le binaire est plus lourd, mais l'affichage est instantané et l'application est "portable" (pas besoin de dossier `assets` séparé pour qu'elle fonctionne).


### 5. Gestion de l'État (`MyApp`)
La structure utilise le mot-clé `pub(crate)` pour permettre aux différents modules d'accéder aux champs sans les rendre totalement publics :
- `selected_index`: `usize` (pour savoir quelle image afficher).
- `zoom_level`: `f32` (pour multiplier la taille de l'image).


### 6. Logique d'affichage
- **Sidebar** : Boucle sur un tableau de chemins d'accès aux images. Chaque image est affichée comme un bouton. Si l'utilisateur clique, l'index dans `MyApp` est mis à jour.
- **Preview** :
    - Un `Slider` contrôle la valeur de `zoom_level`.
    - L'image sélectionnée est rendue dans le `CentralPanel`. Sa taille affichée est calculée en multipliant sa taille d'origine par le facteur de zoom.

---

## 🛠️ Dépendances Requises (`Cargo.toml`)

Pour reproduire ce projet, les dépendances suivantes sont nécessaires :
```toml
[dependencies]
eframe = "0.31"
egui_extras = { version = "0.31", features = ["all_loaders"] }
image = { version = "0.25", default-features = false, features = ["png"] } # Pour le support du format PNG
```

### Étapes d'implémentation rapide :
1.  **Initialiser** les loaders dans le `main` avec `egui_extras::install_image_loaders(&cc.egui_ctx);`.
2.  **Charger** l'image avec `egui::Image::new(egui::include_image!("../assets/mon_image.png"))`.
3.  **Appliquer** les transformations (zoom, arrondis) via les méthodes chaînées sur l'objet `Image`.


**Verdict :** Le projet montre que l'affichage d'images dans **egui** est devenu extrêmement simple grâce aux nouveaux outils de chargement intégrés, évitant ainsi la gestion manuelle complexe des textures GPU qui était nécessaire dans les versions plus anciennes.