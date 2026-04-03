
# 📦 Projet : egui Modules & Structure (Contact Book)

[Rust Module System in egui — Separate UI from Logic | GUI Tutorial - YouTube](https://www.youtube.com/watch?v=S-6fSktXgPA)



L'objectif de ce tutoriel est d'apprendre à organiser une application Rust en séparant l'interface utilisateur (UI) de la logique métier grâce au système de modules.

---

## 🎥 Résumé de la Vidéo

L'application exemple est un gestionnaire de contacts ("Contact Book") qui permet d'ajouter, visualiser et supprimer des fiches contacts.

### Concepts Clés abordés :
- **Organisation en modules** : Séparer le code dans différents fichiers pour éviter d'avoir un fichier `main.rs` trop volumineux.
- **Séparation UI / Logique** :
    - Un module pour les données (la structure `Contact`).
    - Un module pour l'affichage (le panneau latéral, la zone de création, la zone de visualisation).
- **Visibilité (`pub`)** : Utilisation du mot-clé `pub` pour permettre aux modules de communiquer entre eux.
- **Déclaration des modules** : Utilisation de `mod nom_du_module;` dans `main.rs` pour inclure les fichiers `.rs` correspondants.

---

## 💻 Structure du Projet (GitHub)

Le code est décomposé en plusieurs fichiers, chacun ayant une responsabilité précise.

### 1. Arborescence des Fichiers

#### Structure du projet

```bash
.
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── app.rs
    ├── main.rs
    ├── models
    │   ├── contact.rs
    │   └── mod.rs
    └── ui
        ├── contact_form.rs
        ├── contact_list.rs
        └── mod.rs
```

| Dossier / Fichier | Contenu                              | Rôle                                                           |
| :---------------- | :----------------------------------- | :------------------------------------------------------------- |
| `main.rs`         | `mod models; mod ui;`                | Point d'entrée et déclaration de la hiérarchie.                |
| **`models/`**     | `contact.rs`, `mod.rs`               | Définit la structure `Contact` (nom, email, téléphone).        |
| **`ui/`**         | `sidebar.rs`, `details.rs`, `mod.rs` | Contient les fonctions de rendu pour chaque partie de l'écran. |
| `app.rs`          | `struct ContactBookApp`              | Gère le vecteur de contacts et la logique métier.              |

##### Le module `models`
Ce dossier contient la définition pure des données.
* **`contact.rs`** : Définit la structure `Contact` avec des champs publics pour qu'ils soient lisibles par l'interface.
* **`mod.rs`** : Expose le fichier `contact.rs` au reste de l'application via `pub mod contact;`.

##### Le module `ui`
Ce dossier regroupe tout ce qui concerne le dessin des widgets **egui**.
* **`sidebar.rs`** : Affiche la liste cliquable des noms à gauche.
* **`details.rs`** : Affiche les informations complètes ou le formulaire d'ajout à droite.
* **`mod.rs`** : Centralise et expose les fonctions de dessin pour que `app.rs` puisse les appeler.


### 2. Le module de données (`models.rs`)
La structure `Contact` est définie de manière simple et publique :
```rust
pub struct Contact {
    pub name:  String,
    pub email: String,
    pub phone: String,
}
```

### 3. Logique de l'application (`app.rs`)
Ce fichier gère l'état global, notamment la liste des contacts et l'index du contact sélectionné. Il appelle ensuite les fonctions de rendu définies dans les modules de vue.

---

## 🛠️ Implémentation de la Structure

### Enchaînement des actions (Workflow)
1.  **Saisie** : L'utilisateur remplit les champs Nom, Email et Téléphone dans le formulaire "Add New Contact".
2.  **Ajout** : Au clic sur "Add Contact", un nouvel objet `Contact` est créé et ajouté au vecteur de l'application.
3.  **Sélection** : Cliquer sur un nom dans la barre latérale ("Contacts") met à jour l'index de sélection.
4.  **Affichage/Suppression** : La zone centrale affiche les détails du contact sélectionné ou permet sa suppression.


---

## 🔗 Liens et Fonctionnalités Clés
- **[[00:06](http://www.youtube.com/watch?v=S-6fSktXgPA&t=6)]** : Aperçu de l'interface finale avec la liste des contacts à gauche et le formulaire à droite.
- **[[00:11](http://www.youtube.com/watch?v=S-6fSktXgPA&t=11)]** : Démonstration de la sélection d'un contact existant (Alice Smith).
- **[[00:21](http://www.youtube.com/watch?v=S-6fSktXgPA&t=21)]** : Vue du formulaire vide prêt pour l'ajout d'un nouveau contact.
- **[[00:30](http://www.youtube.com/watch?v=S-6fSktXgPA&t=30)]** : Saisie d'un nouveau contact (Carol Davis).
- **[[00:43](http://www.youtube.com/watch?v=S-6fSktXgPA&t=43)]** : Validation et apparition instantanée du nouveau contact dans la liste latérale.

**Conclusion :** Adopter une structure modulaire dès le début d'un projet **egui** est essentiel. Cela permet de tester la logique métier indépendamment de l'interface et facilite grandement la collaboration ou l'évolution du code.



http://googleusercontent.com/youtube_content/2