// Function to add environment variable addons to the installer
// Path to addons: ../../template/addons/env

use std::fs;

use crate::prompts::packages_prompt::Packages;

pub fn install_env(_app_name: &str, packages: &Vec<Packages>) {
    println!("Installing environment variables...");
    let mut env_file = "env-".to_string();
    match packages {
        packages if packages.contains(&Packages::Prisma) => {
            println!("Installing Prisma environment variables...");
            env_file = format!("{}-prisma", env_file);
        }
        packages if packages.contains(&Packages::Trpc) => {
            println!("Installing Prisma environment variables...");
            env_file = format!("{}-trpc", env_file);
        }
        packages if packages.contains(&Packages::NextAuth) => {
            println!("Installing NextAuth environment variables...");
            env_file = format!("{}-nextauth", env_file);
        }
        _ => {}
    }

    env_file = format!("{}.mjs", env_file);
    // possible combinations ["env-prisma-trpc-nextauth", "env-prisma-trpc", "env-prisma-nextauth", "env-trpc-nextauth", "env-prisma", "env-trpc", "env-nextauth", "env"]

    // get file from ../../template/addons/env/env_file with fs::read_to_string
    //fix this not reading
    let file = fs::read_to_string("./../../template/addons/env/auth-prisma-schema.mjs")
        .expect("reading failed");
    print!("{}", file);
}
