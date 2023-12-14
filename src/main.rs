use std::io;
use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f32> = HashMap::new();
    println!(r"

    /$$$$$$$$        /$$                       /$$      /$$                           /$$ /$$           /$$
    |__  $$__/       | $$                      | $$$    /$$$                          | $$|__/          |__/
       | $$  /$$$$$$ | $$   /$$  /$$$$$$       | $$$$  /$$$$  /$$$$$$  /$$$$$$$   /$$$$$$$ /$$  /$$$$$$  /$$
       | $$ /$$__  $$| $$  /$$/ /$$__  $$      | $$ $$/$$ $$ |____  $$| $$__  $$ /$$__  $$| $$ /$$__  $$| $$
       | $$| $$  \ $$| $$$$$$/ | $$  \ $$      | $$  $$$| $$  /$$$$$$$| $$  \ $$| $$  | $$| $$| $$  \__/| $$
       | $$| $$  | $$| $$_  $$ | $$  | $$      | $$\  $ | $$ /$$__  $$| $$  | $$| $$  | $$| $$| $$      | $$
       | $$|  $$$$$$/| $$ \  $$|  $$$$$$/      | $$ \/  | $$|  $$$$$$$| $$  | $$|  $$$$$$$| $$| $$      | $$
       |__/ \______/ |__/  \__/ \______/       |__/     |__/ \_______/|__/  |__/ \_______/|__/|__/      |__/                                                                                                                                                                                                       
    
    ");

    loop {
        println!("\nMenu:");
        println!("1. Tambah item");
        println!("2. Lihat menu");
        println!("3. Edit item");
        println!("4. Hapus item");
        println!("5. Keluar");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Gagal membaca baris");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Masukkan nama item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Gagal membaca baris");

                println!("Masukkan harga item:");
                let mut price = String::new();
                io::stdin().read_line(&mut price).expect("Gagal membaca baris");

                let price: f32 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Harga tidak valid");
                        continue;
                    }
                };

                menu.insert(name.trim().to_string(), price);
                println!("Item berhasil ditambahkan!");
            }
            2 => {
                println!("Daftar Menu:");
                for (item, price) in &menu {
                    println!("{} - Rp{}", item, price);
                }
            }
            3 => {
                println!("Masukkan nama item yang ingin diubah harganya:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Gagal membaca baris");

                println!("Masukkan harga baru:");
                let mut price = String::new();
                io::stdin().read_line(&mut price).expect("Gagal membaca baris");

                let price: f32 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Harga tidak valid");
                        continue;
                    }
                };

                if let Some(item) = menu.get_mut(&name.trim().to_string()) {
                    *item = price;
                    println!("Harga item berhasil diubah!");
                } else {
                    println!("Item tidak ditemukan");
                }
            }
            4 => {
                println!("Masukkan nama item yang ingin dihapus:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Gagal membaca baris");

                if let Some(_) = menu.remove(&name.trim().to_string()) {
                    println!("Item berhasil dihapus!");
                } else {
                    println!("Item tidak ditemukan");
                }
            }
            5 => {
                println!("Terima kasih!");
                break;
            }
            _ => {
                println!("Pilihan tidak valid");
            }
        }
    }
}