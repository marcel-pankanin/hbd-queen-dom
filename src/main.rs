use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    loves: Vec<String>,
    hates: Vec<String>,
    achievements: Vec<String>,
    breakfast: String,
    brain_type: String,
}

#[derive(Debug)]
struct TeamExperience {
    old_team: String,
    new_team: String,
    old_manager_sins: Vec<String>,
}

fn main() {
    let dominika = Person {
        name: "Dominika".to_string(),
        loves: vec![
            "movies".to_string(),
            "frogs".to_string(),
            "dramas".to_string(),
            "plush toys".to_string(),
            "fourth wave feminism".to_string(),
            "growth".to_string(),
            "painting".to_string(),
            "coffee".to_string(),
            "chess".to_string(),
            "plants".to_string(),
            "coding in Rust".to_string(),
            "policies Team".to_string()
        ],
        hates: vec![
            "bad managers".to_string(),
            "selfish leadership".to_string(),
            "workplace isolation policies".to_string(),
            "working without a mouse and a keyboard".to_string(),
            "crowds".to_string(),
            "man".to_string(),
            "children".to_string(),
            "dogs".to_string()
        ],
        achievements: vec![
            "learned Rust like a boss".to_string(),
            "joined the kernel team".to_string(),
            "learned Kotlin in less than a year".to_string(),
            "became a perfect leader in less than a year".to_string(),
            "showed the boys adult conflict resolution strategies".to_string(),
            "escaped toxic management".to_string(),
        ],
        breakfast: "energy bars".to_string(),
        brain_type: "AuDHD superpower".to_string(),
    };


    let team_journey = TeamExperience {
        old_team: "policies team (amazing people, sad to leave)".to_string(),
        new_team: "kernel team (new adventures await!)".to_string(),
        old_manager_sins: vec![
            "terrible team management".to_string(),
            "selfish decision making".to_string(),
            "forbidding workplace friendships".to_string(),
            "lack of empathy".to_string(),
        ],
    };

    let wish_senders = vec!["Marcel", "Kira"];

    println!("🎉 SPECIAL WISHES FOR {} 🎉", dominika.name);
    println!("From: {} & {}", wish_senders[0], wish_senders[1]);
    println!();

    println!("// Your passion for {} fuels your creativity! 🎬", dominika.loves[0]);
    println!("// May your love for {} bring you joy and luck! 🐸", dominika.loves[1]);
    println!("// Your {} keep life interesting and full of plot twists! 📺", dominika.loves[2]);
    println!();

    println!("// Rust achievement unlocked! 🦀");
    for achievement in &dominika.achievements {
        println!("// ✅ {}", achievement);
    }
    println!();

    println!("// Team transition status:");
    println!("// Leaving: {} 💔", team_journey.old_team);
    println!("// Joining: {} 🚀", team_journey.new_team);
    println!();

    println!("// Old manager.exe has stopped working due to:");
    for sin in &team_journey.old_manager_sins {
        println!("// ❌ {}", sin);
    }
    println!("// Good riddance! You deserve better leadership! 👋");
    println!();

    println!("// Your {} brain is your superpower! 🧠⚡", dominika.brain_type);
    println!("// May your {} always give you the energy you need! 🍫", dominika.breakfast);
    println!();

    let mut wishes: HashMap<&str, &str> = HashMap::new();
    wishes.insert("career", "May your kernel adventures be bug-free and fulfilling!");
    wishes.insert("movies", "May you discover amazing films and have cozy movie nights!");
    wishes.insert("frogs", "May cute frogs bring smiles to your face every day!");
    wishes.insert("plush toys", "May you always have a plush toy with you!");
    wishes.insert("concentration", "May you always have a Medikinet to listen to boring meetings!");
    wishes.insert("patience", "May you always have a patience to imperfect world around you!");
    wishes.insert("health", "May your AuDHD be your creative superpower!");
    wishes.insert("happiness", "May drama exist only in your favorite shows, not in real life!");
    wishes.insert("future", "May your new team appreciate your awesomeness!");
    wishes.insert("chess", "May you never lose a game of chess to kids!");

    println!("🌟 OUR WISHES FOR YOU 🌟");
    for (category, wish) in wishes {
        println!("// {}: {}", category.to_uppercase(), wish);
    }

    println!();
    println!("// Compiled with love by Marcel & Kira 💝");
    println!("// Version: 2024.friendship.release 🎊");
}