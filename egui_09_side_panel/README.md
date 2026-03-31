
# Résumé : Création d'un "Note Viewer" avec SidePanel (`egui`)

[Build a Note Viewer with Side Panel in egui | Learn egui in Neovim Ep 9 - YouTube](https://www.youtube.com/watch?v=KIhsHRpZMcQ)




Cette vidéo (épisode 9 de la série "Learn egui in Neovim") explique comment structurer une interface utilisateur en deux parties : une barre latérale de navigation et un panneau central de contenu.


## 1. Concepts Clés d'Interface
L'application utilise deux composants majeurs de la bibliothèque `egui` pour diviser l'écran :

- **`SidePanel::left("id")`** : Crée une région fixe sur le côté gauche. L'identifiant doit être unique.
- **`CentralPanel::default()`** : Remplit automatiquement tout l'espace restant après le placement des panneaux (Side, Top, Bottom).
- **`selectable_label`** : Un widget qui agit comme un bouton cliquable restant "mis en surbrillance" si la condition (booléen) est vraie.


## 2. Structure du Code Rust
Le projet est divisé en deux fichiers principaux : `main.rs` et `app.rs`.

### A. Configuration de la Fenêtre (`main.rs`)
Le point d'entrée configure les options natives de la fenêtre (taille du viewport) et lance l'application.
```rust
// Extrait simplifié de la logique main.rs
let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
    ..Default::default()
};
```

### B. Logique de l'Application (`app.rs`)
C'est ici que réside la structure des données et l'UI.

| Composant            | Rôle Technique                                                      |
| :------------------- | :------------------------------------------------------------------ |
| **Struct `Note`**    | Contient `title` (String) et `body` (String).                       |
| **Struct `MyApp`**   | Stocke un `Vec<Note>` et un `selected_index: usize`.                |
| **Méthode `update`** | Gère le rendu à chaque frame (Sidebar à gauche, Contenu au centre). |


## 3. Détails de l'Implémentation UI
Le flux logique dans la fonction `update` est le suivant :

1.  **Sidebar (Navigation) :**
    * On utilise `ui.selectable_label(self.selected_index == i, &note.title)` à l'intérieur d'une boucle `for`.
    * Si l'utilisateur clique, `self.selected_index` est mis à jour avec l'index actuel.
2.  **Panneau Central (Lecture) :**
    * On récupère la note sélectionnée via l'index : `let note = &self.notes[self.selected_index]`.
    * On affiche le titre en grand (`Heading`) et le corps du texte.



## 4. Points à Retenir (Takeaways)
- **`min_width(150.0)`** : Utilisé sur le `SidePanel` pour garantir que la barre latérale ne disparaisse pas si le contenu est court.
- **Mise à jour d'état** : Le changement de note est instantané car `egui` est en "Immediate Mode" (l'interface est redessinée dès que la variable `selected_index` change).


## 5. Exemple de code (Logique SidePanel)
```rust
egui::SidePanel::left("sidebar").min_width(150.0).show(ctx, |ui| {
    for (i, note) in self.notes.iter().enumerate() {
        if ui.selectable_label(self.selected_index == i, &note.title).clicked() {
            self.selected_index = i;
        }
    }
});

egui::CentralPanel::default().show(ctx, |ui| {
    let note = &self.notes[self.selected_index];
    ui.heading(&note.title);
    ui.separator();
    ui.label(&note.body);
});
```

