use std::error::Error;

use rand::Rng;

pub fn patch_chromedriver(chromedriver_executable: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("Starting ChromeDriver executable patch...");
    let file_name = if cfg!(windows) {
        "chromedriver.exe"
    } else {
        "chromedriver"
    };
    let f = std::fs::read(file_name)?;
    let mut new_chromedriver_bytes = f.clone();
    let mut total_cdc = String::from("");
    let mut cdc_pos_list = Vec::new();
    let mut is_cdc_present = false;
    let mut patch_ct = 0;
    for i in 0..f.len() - 3 {
        if "cdc_"
            == format!(
                "{}{}{}{}",
                f[i] as char,
                f[i + 1] as char,
                f[i + 2] as char,
                f[i + 3] as char
            )
            .as_str()
        {
            for x in i + 4..i + 22 {
                total_cdc.push_str(&(f[x] as char).to_string());
            }
            is_cdc_present = true;
            cdc_pos_list.push(i);
            total_cdc = String::from("");
        }
    }
    if is_cdc_present {
        tracing::info!("Found cdcs");
    } else {
        tracing::warn!("No cdcs were found!");
    }
    let get_random_char = || -> char {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect::<Vec<char>>()[rand::rng().random_range(0..48)]
    };

    for i in cdc_pos_list {
        for x in i + 4..i + 22 {
            new_chromedriver_bytes[x] = get_random_char() as u8;
        }
        patch_ct += 1;
    }
    tracing::info!("Patched {} cdcs!", patch_ct);

    tracing::info!("Starting to write to binary file...");
    let _file = std::fs::File::create(chromedriver_executable)?;
    match std::fs::write(chromedriver_executable, new_chromedriver_bytes) {
        Ok(_res) => {
            tracing::info!("Successfully wrote patched executable to 'chromedriver_PATCHED'!",)
        }
        Err(err) => tracing::error!("Error when writing patch to file! Error: {}", err),
    };
    Ok(())
}
