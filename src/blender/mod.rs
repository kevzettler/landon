use clap::ArgMatches;
use std::fs::File;
use std::io::Write;
use std::process::{Command};

pub fn process_blender_subcommand(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("install") {
        // Install the Blender mesh exporter addon
        if let Some(_matches) = matches.subcommand_matches("mesh-to-json") {
            install_mesh_to_json().unwrap();
        } else if let Some(_matches) = matches.subcommand_matches("armature-to-json") {
            install_armature_to_json().unwrap();
        }
    }
}

fn install_mesh_to_json() -> std::io::Result<()> {
    // Write our addon to a tmp file. Our INSTALL_MESH_TO_JSON will look for this tmp file
    // when installing the addon.
    let mesh_to_json_addon = include_str!("../../blender-mesh-to-json.py");
    let mut addon_file = File::create("/tmp/blender-mesh-to-json.py")?;
    addon_file.write_all(mesh_to_json_addon.as_bytes()).unwrap();

    // TODO: Support an environment variable to override the path to the executable
    let blender_executable = "blender";
    Command::new(blender_executable)
        .arg("--background")
        .args(&["--python-expr", INSTALL_MESH_TO_JSON])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Ok(())
}

fn install_armature_to_json() -> std::io::Result<()> {
    // Write our addon to a tmp file. Our INSTALL_MESH_TO_JSON will look for this tmp file
    // when installing the addon.
    let armature_to_json = include_str!("../../blender-armature-to-json.py");
    let mut addon_file = File::create("/tmp/blender-armature-to-json.py")?;
    addon_file.write_all(armature_to_json.as_bytes()).unwrap();

    // TODO: Support an environment variable to override the path to the executable
    let blender_executable = "blender";
    Command::new(blender_executable)
        .arg("--background")
        .args(&["--python-expr", INSTALL_ARMATURE_TO_JSON])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Ok(())
}

static INSTALL_MESH_TO_JSON: &'static str = r#"
# Install the addon and save the user's preferences
import bpy
import os

# Get the absolute path to the addon
dir = os.path.dirname(__file__)
addonFilePath = '/tmp/blender-mesh-to-json.py'

# Install the addon, enable it and save the user's preferences so that it
# is available whenever Blender is opened in the future
bpy.ops.wm.addon_install(filepath=addonFilePath)
bpy.ops.wm.addon_enable(module='blender-mesh-to-json')
bpy.ops.wm.save_userpref()
"#;

static INSTALL_ARMATURE_TO_JSON: &'static str = r#"
import bpy

addonFilePath = '/tmp/blender-armature-to-json.py'

# Install the addon, enable it and save the user's preferences so that it
# is available whenever Blender is opened in the future
bpy.ops.wm.addon_install(filepath=addonFilePath)
bpy.ops.wm.addon_enable(module='blender-armature-to-json')
bpy.ops.wm.save_userpref()
"#;