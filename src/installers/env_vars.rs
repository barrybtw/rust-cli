// Function to add environment variable addons to the installer
// Path to addons: ../../template/addons/env

use fs_extra::dir::remove;

use crate::{helpers::copy::copy_files, prompts::packages_prompt::Packages};

pub fn install_env(_app_name: &str, packages: &Vec<Packages>) {
    println!("Installing environment variables...");
    let mut env_file = "env-".to_string();
    match packages {
        packages if packages.contains(&Packages::Prisma) => {
            println!("Installing Prisma environment variables...");
            env_file = format!("{}prisma-", env_file);
        }
        packages if packages.contains(&Packages::NextAuth) => {
            println!("Installing NextAuth environment variables...");
            env_file = format!("{}auth-", env_file);
        }
        _ => {}
    }

    env_file = format!("{}.mjs", env_file);
    // possible combinations ["env-prisma-trpc-nextauth", "env-prisma-trpc", "env-prisma-nextauth", "env-trpc-nextauth", "env-prisma", "env-trpc", "env-nextauth", "env"]

    // get file from ../../template/addons/env/env_file with fs::read_to_string
    //fix this not reading
    env_file = format!("./src/template/addons/env/{}", env_file);
    let destination = format!("{}/src/env/schema.mjs", _app_name);
    remove(&destination).expect("Failed to remove directory.");
    print!("{} {}", &env_file, &destination);
    copy_files(&env_file, &destination)
}
