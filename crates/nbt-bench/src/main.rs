use std::time::Instant;
use nbt_compression::*;
use nbt_core::*;

fn extract_palette_from_nbt(nbt_data: &NbtTag) -> Vec<String> {
    let mut palette = Vec::new();
    
    if let NbtTag::Compound(root) = nbt_data {
        if let Some(NbtTag::List { items, .. }) = root.get("palette") {
            for item in items {
                if let NbtTag::Compound(block) = item {
                    if let Some(NbtTag::String(name)) = block.get("Name") {
                        palette.push(name.clone());
                    }
                }
            }
        }
    }
    
    palette
}

fn main() {
    println!("=== CONDITION RÉELLE : Extraction des blocs de cube.nbt ===\n");
    
    // Charger le fichier cube.nbt
    let cube_data = include_bytes!("../bench/mock/cube.nbt");
    println!("📁 Fichier cube.nbt chargé: {} bytes ({:.1} KB)", cube_data.len(), cube_data.len() as f64 / 1024.0);
    
    // Mesurer le temps de parsing
    let start_parse = Instant::now();
    let nbt_file = NbtFile::read(cube_data, Endian::Big).unwrap();
    let parse_duration = start_parse.elapsed();
    println!("⚡ Temps de parsing NBT: {:?}", parse_duration);
    
    // Mesurer le temps d'extraction de la palette
    let start_extract = Instant::now();
    let palette = extract_palette_from_nbt(&nbt_file.root);
    let extract_duration = start_extract.elapsed();
    
    println!("⚡ Temps d'extraction palette: {:?}", extract_duration);
    println!("⚡ Temps total (parse + extract): {:?}\n", parse_duration + extract_duration);
    
    // Afficher les résultats
    println!("🎯 LISTE DES BLOCS TROUVÉS ({} types):", palette.len());
    println!("{}", "=".repeat(60));
    
    for (i, block) in palette.iter().enumerate() {
        println!("{:3}. {}", i + 1, block);
    }
    
    println!("{}", "=".repeat(60));
    println!("\n📊 STATISTIQUES:");
    println!("  • Nombre de types de blocs: {}", palette.len());
    println!("  • Taille du fichier: {:.1} KB", cube_data.len() as f64 / 1024.0);
    println!("  • Performance parsing: {:.2} ms", parse_duration.as_micros() as f64 / 1000.0);
    println!("  • Performance extraction: {:.2} μs", extract_duration.as_micros());
    println!("  • Performance totale: {:.2} ms", (parse_duration + extract_duration).as_micros() as f64 / 1000.0);
    
    // Simuler plusieurs extractions pour voir la performance en condition réelle
    println!("\n🔄 TEST DE PERFORMANCE (simulation utilisation réelle):");
    
    let start_multiple = Instant::now();
    let mut all_palettes = Vec::new();
    for i in 0..10 {
        let file = NbtFile::read(cube_data, Endian::Big).unwrap();
        let p = extract_palette_from_nbt(&file.root);
        all_palettes.push(p);
        if i == 0 {
            print!("Processing");
        }
        print!(".");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
    let multiple_duration = start_multiple.elapsed();
    
    println!("\n  • 10 extractions complètes: {:?}", multiple_duration);
    println!("  • Moyenne par extraction: {:.2} ms", multiple_duration.as_micros() as f64 / 10.0 / 1000.0);
    println!("  • Débit: {:.0} fichiers/seconde", 10.0 / multiple_duration.as_secs_f64());
    
    println!("\n✅ Test terminé avec succès!");
} 