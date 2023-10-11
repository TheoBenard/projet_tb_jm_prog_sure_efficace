mod game;

fn main() {
    println!("Welcome to Minesweeper!\n");
    game::play();
}

/*
    TIMER à ajouter par la suite

    let mut start_time = Instant::now();

    loop {
        let elapsed_time = start_time.elapsed();
        let seconds = elapsed_time.as_secs();
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;

        // Effacez l'écran (pour éviter que le temps précédent ne soit affiché)
        print!("\x1B[2J\x1B[1;1H");

        // Affichez le temps écoulé au format mm:ss
        println!("{:02}:{:02}", minutes, remaining_seconds);

        // Attendez une seconde avant la prochaine mise à jour
        thread::sleep(Duration::from_secs(1));
    }
    
*/