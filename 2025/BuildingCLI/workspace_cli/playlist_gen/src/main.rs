//use clap::{App, Arg}; // App has been renamed to Command
use clap::{Arg, Command, Parser}; // Command: struct used to build the command line interface
use std::fs::File;
use std::io::{self, Write};
// use std::path::Path;
use walkdir::WalkDir;

// Define struct taht hold all the arguments:
#[derive(Parser, Debug)]
#[command(version, author, about = "generate playlist from files in a folder")]
struct Args {
    // path to the folders to search
    folder: String,
    // comma separated file types to include (mp3,mp4,mkv..)
    extensions: String,
}
fn main() -> io::Result<()> {
    // Step 1: Build the command line interface with 2 arguments "folder" and "extensions"
    let matches = Command::new(":::Playlist Generator:::")
        .version("1.0")
        .author("daybreak")
        .about("Generates playlists from files in folder and its sub-folders")
        .arg(
            Arg::new("folder")
                .help("Path to the folder to search")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("extensions")
                .help("comma separated list of file types to include in playlist(e.g: mp3.,mp4)")
                .required(true)
                .index(2),
        )
        .get_matches(); // processes the arguments provided by the user

    // step 2: retrieving and Preparing arguments:
    // clap 4.5 new way to retrieving argument values as a string
    let folder = matches
        .get_one::<String>("folder")
        .expect("Folder is required");
    let ext_str = matches
        .get_one::<String>("extensions")
        .expect("extensions are required");

    let extensions: Vec<&str> = ext_str.split(',').collect();

    // Step 3: Playlist file creation:
    let playlist_file = "playlist.m3u";
    let mut file = File::create(playlist_file)?; // ? to propagate return of any potential
                                                 // io::Error ... to main()

    // Step 3: directory traversal and Filtering:
    let walker = WalkDir::new(folder).into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        //check if the file has a valid extensions:
        if let Some(extension) = path.extension() {
            if extensions.contains(&extension.to_string_lossy().as_ref()) {
                //Write the valid file path to the playlist file
                if path.is_file() {
                    writeln!(file, "{}", path.display())?;
                }
            }
        }
    }
    println!("playlist generated: {}", playlist_file);
    Ok(()) // Return Ok(()) to match the return type
}
