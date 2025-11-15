fn main() {
    let year = 2024;
    let month = 1;
    let day = 15;

    println!("📅 {year}-{month:02}-{day:02}"); // 📅 2024-01-15

    let amount = 142.9765;

    println!("💵 {amount:.2} ₽"); // 💵 142.98 ₽

    let r = 255;
    let g = 128;
    let b = 0;

    println!("🎨 #{r:02X}{g:02X}{b:02X}"); // 🎨 #FF8000

    let name = "Alice";
    let age = 25;
    let score = 95.543;

    println!("| {name:<8} | {age:^8} | {score:>8.1} |"); // | Alice    |    25    |     95.5 |

    
}
