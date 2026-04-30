use crate::{log_header, log_message, log_package_status, wait_for_confirmation};
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    env, fs,
    io::{Cursor, Write},
    path::PathBuf,
};
use stringcase::snake_case;
use toml::Table;
use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

#[derive(Serialize, Deserialize)]
struct PackageFile {
    data: Vec<u8>,
    name: String,
    alias: String,
    entry: PathBuf,
    version: String, // semantic format
    dependencies: Vec<PackageFile>,
    hash: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct PackageRegistry {
    pub packages: HashMap<String, HashMap<String, HashMap<String, String>>>, // {name: { version: { info } }
}

fn get_project_root_folder() -> PathBuf {
    env::current_dir().expect("Unable to get project root folder")
}

fn get_configuration_folder() -> PathBuf {
    let pkg_path = dirs::home_dir()
        .expect("Unable to retrieve user home directory")
        .join(".glang");

    pkg_path
}

fn get_package_registry() -> PathBuf {
    get_configuration_folder().join("packages.json")
}

pub fn get_latest_version(package: &str) -> Option<Version> {
    let registry = read_registry();

    let versions_map = registry.packages.get(package)?;

    versions_map
        .keys()
        .filter_map(|v| Version::parse(v).ok())
        .max()
}

fn create_configuration_folder() {
    let config_path = get_configuration_folder();
    let registry = config_path.join("packages.json");

    if !config_path.exists() {
        fs::create_dir_all(&config_path).expect("Unable to build '.glang' configuration directory");
    }

    if !registry.exists() {
        fs::write(config_path.join("packages.json"), "{\"packages\": {}}")
            .expect("Unable to write 'packages.json' file");
    }
}

pub fn create_package_folder() {
    let root = get_project_root_folder();
    let source_folder = root.join("src");
    let entry_file = root.join("lib.glang");
    let package_config = root.join("kennel.toml");
    let git_config = root.join(".gitignore");

    fs::create_dir(&source_folder).expect("Unable to create 'src' folder");
    fs::write(&entry_file, "# this is the entrypoint of your kennel\n# see https://george-language.github.io/docs/kennels/creating for more info").expect("Unable to create 'lib.glang' file");
    fs::write(
        &package_config,
        format!(
            r#"[package]
name = {:?}
version = "0.1.0"
entry = "lib.glang"

[requirements]
        "#,
            root.file_name().unwrap()
        ),
    )
    .expect("Unable to create 'kennel.toml' file");
    fs::write(&git_config, "*.kennel").expect("Unable to create '.gitignore' file");
}

pub fn create_project_folder() {
    let root = get_project_root_folder();
    let source_folder = root.join("src");
    let main_file = root.join("main.glang");

    fs::create_dir(&source_folder).expect("Unable to create 'src' folder");
    fs::write(
        &main_file,
        "func main() {\n    bark(\"Hello, World!\");\n }\n\n main();",
    )
    .expect("Unable to create 'main.glang' file");
}

fn verify_package_configuration_file(contents: &str) -> (String, Version, String, Vec<PathBuf>) {
    log_message("Parsing 'kennel.toml'");

    let package_toml = contents
        .parse::<Table>()
        .expect("Error parsing 'kennel.toml'");

    let package_field = package_toml["package"]
        .as_table()
        .expect("'package' field is an invalid table (error in kennel.toml)");
    let requirements_field = package_toml["requirements"]
        .as_table()
        .expect("'requirements' field is an invalid table (error in kennel.toml)");

    let name = package_field["name"]
        .as_str()
        .expect("'name' field is an invalid string (error in kennel.toml)");

    if snake_case(name) != name {
        panic!("'name' field must be snake-case (error in kennel.toml)")
    }

    if glang_attributes::BUILT_IN_FUNCTIONS.contains(&name) {
        panic!("'name' field cannot be the name of a built-in function (error in kennel.toml)")
    }

    let version = package_field["version"]
        .as_str()
        .expect("'version' field is an invalid string (error in kennel.toml)");
    let version = Version::parse(version)
        .expect("'version' field is not in semantic style versioning (error in kennel.toml)");

    let entry = package_field["entry"]
        .as_str()
        .expect("'entry' field is an invalid string (error in kennel.toml)");

    {
        // validate entry field path
        let entry_path = PathBuf::from(entry);

        if !entry_path.exists() {
            panic!("'entry' field leads to a file that does not exist (error in kennel.toml)")
        }

        if entry_path
            .extension()
            .expect("'entry' is not a valid file (error in kennel.toml)")
            != "glang"
        {
            panic!(
                "'entry' field leads to a file that is not a '.glang' file (error in kennel.toml)"
            )
        }
    }

    let mut requirements = Vec::new();

    for (name, version_value) in requirements_field {
        let version_str = version_value
            .as_str()
            .expect("Requirement version must be a string (error in kennel.toml)");

        let version = Version::parse(version_str)
            .expect("Invalid semantic version in requirements (error in kennel.toml)");

        // look it up in registry
        let registry = read_registry();

        let package_versions = registry.packages.get(name).unwrap_or_else(|| {
            panic!(
                "Requirement '{}' not found in registry (error in kennel.toml)",
                name
            )
        });

        let package_info = package_versions
            .get(&version.to_string())
            .unwrap_or_else(|| {
                panic!(
                    "Version '{}' of requirement '{}' not found in registry (error in kennel.toml)",
                    version, name
                )
            });

        let entry_path = package_info.get("location").unwrap();

        requirements.push(PathBuf::from(entry_path));
    }

    (name.to_string(), version, entry.to_string(), requirements)
}

pub fn read_registry() -> PackageRegistry {
    create_configuration_folder();

    let registry: PackageRegistry = serde_json::from_str(
        &fs::read_to_string(get_package_registry()).expect("Unable to read package registry file"),
    )
    .expect("Unable to deserialize package registry");

    registry
}

pub fn write_registry(registry: PackageRegistry) {
    create_configuration_folder();

    fs::write(
        get_package_registry(),
        serde_json::to_string_pretty(&registry).expect("Unable to serialize registry data"),
    )
    .expect("Unable to write package registry file");
}

fn create_package_file(root: Option<PathBuf>) -> (PackageFile, PathBuf) {
    let root = root.unwrap_or(env::current_dir().expect("Unable to get root"));
    let package_config_file = root.join("kennel.toml");
    let source_folder = root.join("src");

    if !package_config_file.exists() {
        panic!("Missing 'kennel.toml' configuration file")
    }

    let (name, version, entry, requirements) = verify_package_configuration_file(
        &fs::read_to_string(&package_config_file).expect("Unable to read 'kennel.toml' file"),
    );

    let entry_file = root.join(&entry);

    log_message(&format!("Compressing {name} {version}"));

    let mut cursor = std::io::Cursor::new(Vec::new());
    let mut archive = ZipWriter::new(&mut cursor);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    archive
        .start_file_from_path(&entry_file.strip_prefix(&root).unwrap(), options)
        .expect("Error zipping file");
    archive
        .write(
            &fs::read(&entry_file.strip_prefix(&root).unwrap()).expect("Error reading entry file"),
        )
        .expect("Error writing file to zip");
    archive
        .start_file_from_path(&package_config_file.strip_prefix(&root).unwrap(), options)
        .expect("Error zipping file");
    archive
        .write(
            &fs::read(&package_config_file.strip_prefix(&root).unwrap())
                .expect("Error reading kennel.toml file"),
        )
        .expect("Error writing file to zip");

    if source_folder.exists() {
        for entry in WalkDir::new(&source_folder) {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if path.is_file() {
                archive
                    .start_file_from_path(path.strip_prefix(&root).unwrap(), options)
                    .expect("Error starting file in zip");

                let data = fs::read(path).expect("Error reading file");
                archive.write_all(&data).expect("Error writing file to zip");
            }
        }
    }

    archive.finish().expect("Error finishing zip archive");

    let zip_bytes = cursor.into_inner();
    let hash = Sha256::digest(&zip_bytes);

    let path = root.join(format!("{}-{}.kennel", name, version));

    log_message(&format!("Resolving {name} {version} requirements"));

    let mut dependencies = Vec::new();

    for requirement in requirements {
        dependencies.push(create_package_file(Some(requirement)).0);
    }

    (
        PackageFile {
            data: zip_bytes,
            hash: hash.into(),
            name: path
                .file_name()
                .expect("Error getting file name of kennel")
                .to_string_lossy()
                .to_string(),
            alias: name,
            entry: entry_file.strip_prefix(&root).unwrap().to_owned(),
            version: version.to_string(),
            dependencies: dependencies,
        },
        path,
    )
}

pub fn write_package_file(root: Option<PathBuf>) {
    log_header("Bundling project into kennel");

    let (package_file, path) = create_package_file(root);

    let data = bincode::serialize(&package_file).expect("Error serializing kennel file");
    fs::write(&path, data).expect("Error writing kennel file");

    log_message(&format!(
        "Successfully bundled {}",
        path.file_name().unwrap().to_string_lossy().to_string()
    ));
}

fn add_package_from_file(package: &PackageFile, force: bool) {
    create_configuration_folder();

    let config_dir = get_configuration_folder();

    let package_path = config_dir.join(&package.name);
    let incoming_version = Version::parse(&package.version).expect("Error parsing kennel version");

    fs::create_dir_all(&package_path).expect("Unable to create kennel directory");

    log_message(&format!("Unzipping compressed {}", package.name));

    let cursor = Cursor::new(&package.data);
    let mut archive = ZipArchive::new(cursor).expect("Failed to read zip archive");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to access file");
        let out_path = package_path.join(file.name());

        if file.name().ends_with('/') {
            // directory
            fs::create_dir_all(&out_path).expect("Failed to create directory");
        } else {
            // ensure parent directories exist
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directories");
            }

            let mut outfile = fs::File::create(&out_path).expect("Failed to create file");

            std::io::copy(&mut file, &mut outfile).expect("Failed to write file");
        }
    }

    let mut registry = read_registry();

    if let Some(versions) = registry.packages.get(&package.alias) {
        if let Some(existing_info) = versions.get(&incoming_version.to_string()) {
            let existing_hash = existing_info.get("hash").unwrap();

            if *existing_hash == hex::encode(package.hash) {
                if !force {
                    if wait_for_confirmation(&format!(
                        "Kennel {} {} is already installed, overwrite it?",
                        package.alias, incoming_version
                    )) {
                        log_message(&format!(
                            "Overwriting {} {}",
                            package.alias, incoming_version
                        ));
                    } else {
                        log_message(&format!("Skipping {} {}", package.alias, incoming_version));

                        return;
                    }
                }
            } else {
                log_message(&format!(
                    "Skipping install of {} {} (potentially corrupted)",
                    package.alias, incoming_version
                ));

                return;
            }
        }
    }

    let versions = registry
        .packages
        .entry(package.alias.clone())
        .or_insert_with(HashMap::new);
    let mut info = HashMap::new();
    info.insert("hash".to_string(), hex::encode(package.hash));
    info.insert(
        "location".to_string(),
        package_path.to_string_lossy().to_string(),
    );
    info.insert(
        "entry".to_string(),
        package_path
            .join(&package.entry)
            .to_string_lossy()
            .to_string(),
    );
    versions.insert(incoming_version.to_string(), info);

    write_registry(registry);

    for pkg in &package.dependencies {
        add_package_from_file(&pkg, force);
    }

    log_message(&format!(
        "Kennel {} {} installed successfully",
        package.alias, incoming_version
    ));
}

pub fn add_package(path: &str, force: bool) {
    create_configuration_folder();

    let package_file = PathBuf::from(path);

    if !package_file.exists() {
        panic!("'{:?}' does not exist", package_file)
    }

    if package_file.extension().expect("File extension is invalid") != "kennel" {
        panic!("'{:?}' is not a kennel file", package_file)
    }

    log_header(&format!("Adding {path} to kennels registry"));

    let package: PackageFile =
        bincode::deserialize(&fs::read(&package_file).expect("Unable to read kennel file"))
            .expect("Unable to deserialize kennel file");

    add_package_from_file(&package, force);
}

pub fn remove_package(package: &str, force: bool) {
    create_configuration_folder();

    log_header("Removing kennel from registry");

    let mut registry = read_registry();

    if let Some(versions) = registry.packages.get(package) {
        if !force {
            if !wait_for_confirmation("Are you sure you want to continue?") {
                log_message("Cancelling removal");

                return;
            }
        }

        for info in versions.values() {
            if let Some(location) = info.get("location") {
                let _ = fs::remove_dir_all(location);
            }
        }
    } else {
        log_package_status(package, false);

        return;
    }

    registry.packages.remove(package);

    write_registry(registry);

    log_message(&format!("Kennel '{}' removed", package));
}
