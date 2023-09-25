extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};

fn main() {
    // Initialise GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Crée une fenêtre
    let window = Window::new(WindowType::Toplevel);

    // Configure la fenêtre
    window.set_title("Ma première application GTK en Rust");
    window.set_default_size(400, 200);

    // Crée une étiquette (label) pour la fenêtre
    let label = Label::new(Some("Bonjour, Rust avec GTK!"));

    // Crée un bouton
    let button = Button::with_label("Cliquez-moi!");

    // let button = Button::with_label("Cliquez-moi!");

    // Ajoute un gestionnaire d'événements au bouton
    button.connect_clicked(|_| {
        println!("Le bouton a été cliqué !");
    });

    // Crée une boîte de conteneur pour organiser les widgets
    let box_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    
    // Ajoute la label et le bouton à la boîte de conteneur
    box_container.pack_start(&label, true, true, 0);
    box_container.pack_start(&button, true, true, 0);

    // Ajoute la boîte de conteneur à la fenêtre
    window.add(&box_container);

    // Gère l'événement de fermeture de la fenêtre
    window.connect_delete_event(|_, _| {
        // Ferme l'application lorsque la fenêtre est fermée
        gtk::main_quit();
        Inhibit(false)
    });

    // Affiche tout
    window.show_all();

    // Exécute la boucle principale de GTK
    gtk::main();
}
