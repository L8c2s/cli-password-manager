use directories::BaseDirs;
use sqlite;


if let Some(base_dirs) = BaseDirs::new() {
    const CONFIG_DIR: Path = base_dirs.config_local_dir();
}

const DB_FILE_PATH: Path = CONFIG_DIR.join("cli_cypher.db");

println!("{:#?}", DB_FILE_PATH);
