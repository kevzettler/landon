extern crate blender_armature;
extern crate blender_mesh;
extern crate serde;
extern crate serde_json;

use crate::filesystem::rel_workspace_string;
use crate::set_active_object_by_name;
use blender_armature::parse_armatures_from_blender_stdout;
use blender_armature::BlenderArmature;
use blender_mesh::parse_meshes_from_blender_stdout;
use blender_mesh::BlenderMesh;

use std::fs::File;
use std::io::Write;

use std::process::Command;

// --python-expr wasn't working in travis-ci on linux so writing the scripts to disk
// and using using --python instead
static SELECT_LETTERF_SCRIPT: &'static str = "/tmp/select-letter-f.py";
static SELECT_LETTERFARMATURE_SCRIPT: &'static str = "/tmp/select-letter-f-armature.py";

#[test]
fn parse_skinned_letter_f_mesh_data() {
    let skinned_letter_f_blend =
        &rel_workspace_string(&"crates/blender-export-test/src/tests/skinned_letter_f.blend");
    let run_addon = &rel_workspace_string(&"./run-addon.py");

    // TODO: Move the CLI spawning and parsing into `lib.rs`. In our test just verify
    // the returned mesh data

    let mut select_letterf = File::create(SELECT_LETTERF_SCRIPT).unwrap();
    select_letterf
        .write_all(set_active_object_by_name("LetterF").as_bytes())
        .unwrap();

    let blender_output = Command::new("blender")
        .arg(skinned_letter_f_blend)
        .arg("--background")
        .args(&["--python", SELECT_LETTERF_SCRIPT])
        .args(&["--python", run_addon])
        .arg("-noaudio")
        .arg("--")
        .output()
        .expect("Failed to execute Blender process");

    let stdout = String::from_utf8(blender_output.stdout).unwrap();
    let stderr = String::from_utf8(blender_output.stderr).unwrap();
    assert_eq!(
        stderr, "",
        "\n\nSTDERR = {}, \n\nSTDOUT = {}",
        stderr, stdout
    );

    let parsed_meshes = parse_meshes_from_blender_stdout(&stdout);

    let (_filename, mesh) = parsed_meshes.iter().next().unwrap();

    let mesh = mesh.get("LetterF").unwrap();

    let expected_mesh = &expected_mesh_data();
    let expected_mesh: BlenderMesh = serde_json::from_str(expected_mesh).unwrap();

    assert_eq!(mesh, &expected_mesh)
}

#[test]
fn parse_skinned_letter_f_armature_data() {
    let skinned_letter_f_blend =
        &rel_workspace_string(&"crates/blender-export-test/src/tests/skinned_letter_f.blend");
    let _install_addon = &rel_workspace_string(&"./blender-armature/install-armature-to-json.py");
    let run_addon = &rel_workspace_string(&"./blender-armature/run-armature-to-json.py");

    // TODO: Move the CLI spawning and parsing into `lib.rs`. In our test just verify
    // the returned mesh data

    let mut select_letterf_armature = File::create(SELECT_LETTERFARMATURE_SCRIPT).unwrap();
    select_letterf_armature
        .write_all(set_active_object_by_name("LetterFArmature").as_bytes())
        .unwrap();

    let blender_output = Command::new("blender")
        .arg(skinned_letter_f_blend)
        .arg("--background")
        .args(&["--python", SELECT_LETTERFARMATURE_SCRIPT])
        .args(&["--python", run_addon])
        .arg("-noaudio")
        .arg("--")
        .output()
        .expect("Failed to execute Blender process");

    let stderr = String::from_utf8(blender_output.stderr).unwrap();
    let stdout = String::from_utf8(blender_output.stdout).unwrap();

    assert_eq!(
        stderr, "",
        "\n\nBLENDER STDERR = {} \n\n, BLENDER STDOUT = {}",
        stderr, stdout
    );

    let parsed_armatures = parse_armatures_from_blender_stdout(&stdout);

    let (_filename, armature) = parsed_armatures.iter().next().unwrap();

    let armature = armature.get("LetterFArmature").unwrap();

    let expected_armature = &expected_armature_data();
    let expected_armature: BlenderArmature = serde_json::from_str(expected_armature).unwrap();

    assert_eq!(
        armature, &expected_armature,
        "\n\nBlender STDOUT: {}",
        stdout
    )
}

fn expected_mesh_data() -> String {
    r#"
    {
        "name": "LetterF",
        "multi_indexed_vertex_attributes": {
            "vertices_in_each_face": [4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4],
            "positions": {
                "indices": [79,83,2,3,87,7,6,91,12,4,5,13,107,91,6,14,61,60,31,29,201,79,3,145,103,95,11,15,18,10,11,19,111,99,10,18,16,8,9,17,87,103,15,7,10,14,15,11,99,107,14,10,8,12,13,9,20,16,17,21,119,111,18,22,38,37,26,27,95,115,19,11,144,22,23,151,173,119,22,144,130,20,21,137,115,123,23,19,24,25,27,26,37,49,24,26,50,38,27,25,49,50,25,24,28,29,31,30,60,70,30,31,70,75,28,30,75,61,29,28,22,18,55,44,44,55,54,45,45,54,53,46,46,53,52,47,47,52,51,48,48,51,50,49,18,19,43,55,55,43,42,54,54,42,41,53,53,41,40,52,52,40,39,51,51,39,38,50,23,22,44,32,32,44,45,33,33,45,46,34,34,46,47,35,35,47,48,36,36,48,49,37,19,23,32,43,43,32,33,42,42,33,34,41,41,34,35,40,40,35,36,39,39,36,37,38,14,6,65,71,71,65,64,72,72,64,63,73,73,63,62,74,74,62,61,75,15,14,71,66,66,71,72,67,67,72,73,68,68,73,74,69,69,74,75,70,7,15,66,56,56,66,67,57,57,67,68,58,58,68,69,59,59,69,70,60,6,7,56,65,65,56,57,64,64,57,58,63,63,58,59,62,62,59,60,61,16,20,120,112,112,120,121,113,113,121,122,114,114,122,123,115,137,21,116,152,152,116,117,159,159,117,118,166,166,118,119,173,8,16,112,92,92,112,113,93,93,113,114,94,94,114,115,95,21,17,108,116,116,108,109,117,117,109,110,118,118,110,111,119,9,13,104,96,96,104,105,97,97,105,106,98,98,106,107,99,4,12,100,84,84,100,101,85,85,101,102,86,86,102,103,87,17,9,96,108,108,96,97,109,109,97,98,110,110,98,99,111,12,8,92,100,100,92,93,101,101,93,94,102,102,94,95,103,124,0,76,180,180,76,77,187,187,77,78,194,194,78,79,201,13,5,88,104,104,88,89,105,105,89,90,106,106,90,91,107,4,84,88,5,84,85,89,88,85,86,90,89,86,87,91,90,0,1,80,76,76,80,81,77,77,81,82,78,78,82,83,79,122,200,207,123,200,199,206,207,199,198,205,206,198,197,204,205,197,196,203,204,196,195,202,203,195,194,201,202,121,193,200,122,193,192,199,200,192,191,198,199,191,190,197,198,190,189,196,197,189,188,195,196,188,187,194,195,120,186,193,121,186,185,192,193,185,184,191,192,184,183,190,191,183,182,189,190,182,181,188,189,181,180,187,188,20,130,186,120,130,129,185,186,129,128,184,185,128,127,183,184,127,126,182,183,126,125,181,182,125,124,180,181,82,172,179,83,172,171,178,179,171,170,177,178,170,169,176,177,169,168,175,176,168,167,174,175,167,166,173,174,81,165,172,82,165,164,171,172,164,163,170,171,163,162,169,170,162,161,168,169,161,160,167,168,160,159,166,167,80,158,165,81,158,157,164,165,157,156,163,164,156,155,162,163,155,154,161,162,154,153,160,161,153,152,159,160,1,131,158,80,131,132,157,158,132,133,156,157,133,134,155,156,134,135,154,155,135,136,153,154,136,137,152,153,0,124,131,1,124,125,132,131,125,126,133,132,126,127,134,133,127,128,135,134,128,129,136,135,129,130,137,136,83,179,138,2,179,178,139,138,178,177,140,139,177,176,141,140,176,175,142,141,175,174,143,142,174,173,144,143,2,138,145,3,138,139,146,145,139,140,147,146,140,141,148,147,141,142,149,148,142,143,150,149,143,144,151,150,123,207,151,23,207,206,150,151,206,205,149,150,205,204,148,149,204,203,147,148,203,202,146,147,202,201,145,146],
                "attribute": {
                    "data": [0.125,0.12499999,0.0,0.125,-0.125,0.0,-0.12500001,-0.12499998,0.0,-0.124999955,0.12500004,0.0,0.12500006,0.12499993,1.0,0.12499992,-0.12500007,1.0,-0.12500004,-0.124999955,1.0,-0.12499999,0.125,1.0,0.12500004,0.12499995,0.79263926,0.12499993,-0.12500006,0.79263926,-0.12500004,-0.12499996,0.79263926,-0.124999985,0.12500001,0.79263926,0.12500006,0.12499994,0.8963196,0.124999926,-0.12500006,0.8963196,-0.12500004,-0.124999955,0.8963196,-0.124999985,0.125,0.8963196,0.12500004,0.124999955,0.66119784,0.12499995,-0.12500004,0.66119784,-0.12500004,-0.12499996,0.66119784,-0.12499998,0.12500001,0.66119784,0.12500004,0.12499996,0.5595808,0.124999955,-0.12500004,0.5595808,-0.12500004,-0.12499996,0.5595808,-0.12499998,0.12500001,0.5595808,-0.50342923,-0.12499996,0.5595808,-0.50342923,-0.12499996,0.66119784,-0.5034292,0.12500001,0.5595808,-0.5034292,0.12500001,0.66119784,-0.5135834,-0.124999836,0.8963196,-0.5135834,-0.124999836,1.0,-0.51358336,0.12500012,0.8963196,-0.51358336,0.12500012,1.0,-0.1790613,0.12500001,0.5595808,-0.23312262,0.12500001,0.5595808,-0.28718394,0.12500001,0.5595808,-0.34124523,0.12500001,0.5595808,-0.39530656,0.12500001,0.5595808,-0.44936788,0.12500001,0.5595808,-0.44936785,0.12500001,0.66119784,-0.39530653,0.12500001,0.66119784,-0.3412452,0.12500001,0.66119784,-0.28718388,0.12500001,0.66119784,-0.23312257,0.12500001,0.66119784,-0.17906128,0.12500001,0.66119784,-0.17906135,-0.12499996,0.5595808,-0.23312268,-0.12499996,0.5595808,-0.287184,-0.12499996,0.5595808,-0.3412453,-0.12499996,0.5595808,-0.39530662,-0.12499996,0.5595808,-0.44936794,-0.12499996,0.5595808,-0.4493679,-0.12499996,0.66119784,-0.3953066,-0.12499996,0.66119784,-0.3412453,-0.12499996,0.66119784,-0.28718397,-0.12499996,0.66119784,-0.23312266,-0.12499996,0.66119784,-0.17906135,-0.12499996,0.66119784,-0.18976389,0.12500001,1.0,-0.25452778,0.12500003,1.0,-0.31929168,0.12500006,1.0,-0.38405558,0.12500007,1.0,-0.44881946,0.12500009,1.0,-0.44881952,-0.12499986,1.0,-0.3840556,-0.12499988,1.0,-0.3192917,-0.124999896,1.0,-0.2545278,-0.12499992,1.0,-0.18976393,-0.12499994,1.0,-0.18976387,0.12500001,0.8963196,-0.25452778,0.12500003,0.8963196,-0.31929168,0.12500006,0.8963196,-0.38405558,0.12500007,0.8963196,-0.44881946,0.12500009,0.8963196,-0.18976393,-0.12499993,0.8963196,-0.25452784,-0.12499991,0.8963196,-0.31929174,-0.124999896,0.8963196,-0.38405564,-0.12499987,0.8963196,-0.44881952,-0.12499985,0.8963196,0.075,0.125,0.0,0.025000013,0.12500001,0.0,-0.02499998,0.12500003,0.0,-0.07499997,0.12500003,0.0,0.075,-0.12499999,0.0,0.024999999,-0.124999985,0.0,-0.025000006,-0.124999985,0.0,-0.07500001,-0.124999985,0.0,0.07500005,0.12499995,1.0,0.025000036,0.12499996,1.0,-0.024999976,0.12499998,1.0,-0.07499999,0.124999985,1.0,0.07499993,-0.12500004,1.0,0.024999935,-0.12500003,1.0,-0.025000058,-0.125,1.0,-0.07500005,-0.12499998,1.0,0.07500003,0.12499996,0.79263926,0.025000028,0.12499998,0.79263926,-0.024999976,0.12499999,0.79263926,-0.07499998,0.125,0.79263926,0.07499994,-0.12500004,0.79263926,0.024999946,-0.12500003,0.79263926,-0.02500005,-0.125,0.79263926,-0.07500005,-0.124999985,0.79263926,0.07500005,0.124999955,0.8963196,0.02500004,0.12499997,0.8963196,-0.024999969,0.12499998,0.8963196,-0.07499997,0.124999985,0.8963196,0.07499993,-0.12500004,0.8963196,0.024999935,-0.12500003,0.8963196,-0.025000058,-0.125,0.8963196,-0.07500005,-0.12499998,0.8963196,0.07499994,-0.12500003,0.66119784,0.024999946,-0.12500001,0.66119784,-0.02500005,-0.125,0.66119784,-0.07500005,-0.124999985,0.66119784,0.07500003,0.12499997,0.66119784,0.025000028,0.124999985,0.66119784,-0.024999976,0.12499999,0.66119784,-0.07499997,0.125,0.66119784,0.07499996,-0.12500003,0.5595808,0.024999958,-0.12500001,0.5595808,-0.025000047,-0.125,0.5595808,-0.07500005,-0.124999985,0.5595808,0.07500003,0.12499997,0.5595808,0.025000028,0.124999985,0.5595808,-0.024999976,0.12499999,0.5595808,-0.07499997,0.125,0.5595808,0.125,0.124999985,0.0699476,0.125,0.124999985,0.1398952,0.125,0.124999985,0.2098428,0.12500001,0.12499998,0.2797904,0.12500003,0.12499997,0.349738,0.12500003,0.12499997,0.4196856,0.12500003,0.12499997,0.4896332,0.12499999,-0.125,0.0699476,0.124999985,-0.125,0.1398952,0.12499998,-0.125,0.2098428,0.12499997,-0.12500001,0.2797904,0.12499997,-0.12500003,0.349738,0.12499996,-0.12500003,0.4196856,0.124999955,-0.12500003,0.4896332,-0.12500001,-0.12499998,0.0699476,-0.12500001,-0.12499998,0.1398952,-0.12500001,-0.12499998,0.2098428,-0.12500001,-0.12499998,0.2797904,-0.12500003,-0.12499997,0.349738,-0.12500003,-0.12499997,0.4196856,-0.12500003,-0.12499997,0.4896332,-0.124999955,0.12500004,0.0699476,-0.124999955,0.12500004,0.1398952,-0.124999955,0.12500004,0.2098428,-0.12499996,0.12500004,0.2797904,-0.12499997,0.12500003,0.349738,-0.12499997,0.12500003,0.4196856,-0.12499997,0.12500003,0.4896332,0.074999966,-0.12500003,0.4896332,0.07499997,-0.12500003,0.4196856,0.07499998,-0.12500003,0.349738,0.07499999,-0.12500003,0.2797904,0.07499999,-0.12500001,0.2098428,0.074999996,-0.125,0.1398952,0.075,-0.125,0.0699476,0.024999963,-0.12500001,0.4896332,0.024999969,-0.12500001,0.4196856,0.024999974,-0.12500001,0.349738,0.02499998,-0.12500001,0.2797904,0.024999984,-0.125,0.2098428,0.02499999,-0.12499999,0.1398952,0.024999995,-0.124999985,0.0699476,-0.025000041,-0.125,0.4896332,-0.025000036,-0.125,0.4196856,-0.02500003,-0.125,0.349738,-0.025000025,-0.125,0.2797904,-0.02500002,-0.125,0.2098428,-0.025000015,-0.12499999,0.1398952,-0.02500001,-0.124999985,0.0699476,-0.07500004,-0.124999985,0.4896332,-0.07500003,-0.124999985,0.4196856,-0.07500003,-0.124999985,0.349738,-0.075000025,-0.124999985,0.2797904,-0.07500002,-0.124999985,0.2098428,-0.07500002,-0.124999985,0.1398952,-0.07500002,-0.124999985,0.0699476,0.075,0.125,0.0699476,0.07500001,0.12499999,0.1398952,0.07500002,0.124999985,0.2098428,0.07500002,0.124999985,0.2797904,0.07500002,0.124999985,0.349738,0.075000025,0.12499998,0.4196856,0.07500003,0.12499997,0.4896332,0.025000015,0.12500001,0.0699476,0.025000017,0.12500001,0.1398952,0.025000019,0.12500001,0.2098428,0.02500002,0.12500001,0.2797904,0.025000023,0.125,0.349738,0.025000025,0.12499999,0.4196856,0.025000026,0.124999985,0.4896332,-0.02499998,0.12500003,0.0699476,-0.02499998,0.12500003,0.1398952,-0.02499998,0.12500003,0.2098428,-0.02499998,0.12500003,0.2797904,-0.02499998,0.12500001,0.349738,-0.024999978,0.125,0.4196856,-0.024999976,0.125,0.4896332,-0.07499997,0.12500003,0.0699476,-0.07499997,0.12500003,0.1398952,-0.07499997,0.12500003,0.2098428,-0.07499997,0.12500003,0.2797904,-0.07499997,0.12500003,0.349738,-0.07499997,0.12500001,0.4196856,-0.07499997,0.125,0.4896332],
                    "attribute_size": 3
                }
            },
            "normals": {
                "indices": [0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,10,11,11,11,11,12,12,12,12,13,13,13,13,14,14,14,14,15,15,15,15,16,16,16,16,17,17,17,17,18,18,18,18,19,19,19,19,20,20,20,20,21,21,21,21,22,22,22,22,23,23,23,23,24,24,24,24,25,25,25,25,26,26,26,26,27,27,27,27,28,28,28,28,29,29,29,29,30,30,30,30,31,31,31,31,32,32,32,32,33,33,33,33,34,34,34,34,35,35,35,35,36,36,36,36,37,37,37,37,38,38,38,38,39,39,39,39,40,40,40,40,41,41,41,41,42,42,42,42,43,43,43,43,44,44,44,44,45,45,45,45,46,46,46,46,47,47,47,47,48,48,48,48,49,49,49,49,50,50,50,50,51,51,51,51,52,52,52,52,53,53,53,53,54,54,54,54,55,55,55,55,56,56,56,56,57,57,57,57,58,58,58,58,59,59,59,59,60,60,60,60,61,61,61,61,62,62,62,62,63,63,63,63,64,64,64,64,65,65,65,65,66,66,66,66,67,67,67,67,68,68,68,68,69,69,69,69,70,70,70,70,71,71,71,71,72,72,72,72,73,73,73,73,74,74,74,74,75,75,75,75,76,76,76,76,77,77,77,77,78,78,78,78,79,79,79,79,80,80,80,80,81,81,81,81,82,82,82,82,83,83,83,83,84,84,84,84,85,85,85,85,86,86,86,86,87,87,87,87,88,88,88,88,89,89,89,89,90,90,90,90,91,91,91,91,92,92,92,92,93,93,93,93,94,94,94,94,95,95,95,95,96,96,96,96,97,97,97,97,98,98,98,98,99,99,99,99,100,100,100,100,101,101,101,101,102,102,102,102,103,103,103,103,104,104,104,104,105,105,105,105,106,106,106,106,107,107,107,107,108,108,108,108,109,109,109,109,110,110,110,110,111,111,111,111,112,112,112,112,113,113,113,113,114,114,114,114,115,115,115,115,116,116,116,116,117,117,117,117,118,118,118,118,119,119,119,119,120,120,120,120,121,121,121,121,122,122,122,122,123,123,123,123,124,124,124,124,125,125,125,125,126,126,126,126,127,127,127,127,128,128,128,128,129,129,129,129,130,130,130,130,131,131,131,131,132,132,132,132,133,133,133,133,134,134,134,134,135,135,135,135,136,136,136,136,137,137,137,137,138,138,138,138,139,139,139,139,140,140,140,140,141,141,141,141,142,142,142,142,143,143,143,143,144,144,144,144,145,145,145,145,146,146,146,146,147,147,147,147,148,148,148,148,149,149,149,149,150,150,150,150,151,151,151,151,152,152,152,152,153,153,153,153,154,154,154,154,155,155,155,155,156,156,156,156,157,157,157,157,158,158,158,158,159,159,159,159,160,160,160,160,161,161,161,161,162,162,162,162,163,163,163,163,164,164,164,164,165,165,165,165,166,166,166,166,167,167,167,167,168,168,168,168,169,169,169,169,170,170,170,170,171,171,171,171,172,172,172,172,173,173,173,173,174,174,174,174,175,175,175,175,176,176,176,176,177,177,177,177,178,178,178,178,179,179,179,179,180,180,180,180,181,181,181,181,182,182,182,182,183,183,183,183,184,184,184,184,185,185,185,185,186,186,186,186,187,187,187,187,188,188,188,188,189,189,189,189,190,190,190,190,191,191,191,191,192,192,192,192,193,193,193,193,194,194,194,194,195,195,195,195,196,196,196,196,197,197,197,197,198,198,198,198,199,199,199,199,200,200,200,200,201,201,201,201,202,202,202,202,203,203,203,203,204,204,204,204,205,205,205,205],
                "attribute": {
                    "data": [0.0,0.0,-1.0,0.0,-0.0,1.0,1.0,-5.5134296e-7,3.5930526e-8,-4.4703484e-7,-1.0,0.0,-0.0,0.0,1.0,2.9802334e-7,1.0,0.0,2.9802317e-7,1.0,1.4372209e-7,-1.0,2.5331977e-7,-2.8341834e-8,-4.4703484e-7,-0.99999994,0.0,1.0,-4.172325e-7,5.668366e-8,2.9802317e-7,1.0,3.2124385e-14,-0.99999994,2.384186e-7,0.0,-4.4703484e-7,-1.0,7.1861045e-8,1.0,-4.917383e-7,-3.5930526e-8,0.99999994,-3.7252903e-7,3.6660094e-8,-4.4703486e-7,-1.0,0.0,-0.0,1.0,0.0,2.980232e-7,1.0,1.6893046e-14,-1.0,2.5331974e-7,-1.5977486e-7,-3.725291e-7,-1.0,5.3258244e-8,1.0,-3.2782555e-7,-1.065166e-7,2.9802322e-7,1.0,0.0,-1.0,2.384186e-7,0.0,0.0,0.0,-0.99999994,-0.0,0.0,1.0,0.0,-1.0,0.0,-0.99999994,2.384186e-7,0.0,4.6016874e-7,1.0,0.0,0.0,0.0,-1.0,-2.8760547e-7,-1.0,-3.593053e-8,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,-0.0,0.0,0.99999994,-0.0,0.0,1.0,-0.0,0.0,1.0,-0.0,0.0,1.0,-0.0,0.0,1.0,-0.0,0.0,1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-0.99999994,0.0,0.0,-1.0,0.0,0.0,-1.0,-0.0,1.0,0.0,-0.0,0.99999994,0.0,-0.0,1.0,0.0,-0.0,1.0,0.0,-0.0,1.0,0.0,-0.0,1.0,0.0,-2.8760553e-7,-1.0,-3.5930526e-8,-3.4512664e-7,-1.0,-7.1861e-8,-2.8760547e-7,-1.0,-3.5930444e-8,-2.8760547e-7,-1.0,-3.5930444e-8,-3.4512664e-7,-1.0,-7.1861e-8,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,2.300844e-7,1.0,2.480116e-14,2.300844e-7,1.0,1.6534107e-14,4.6016874e-7,1.0,0.0,2.3008437e-7,1.0,0.0,2.3008447e-7,1.0,0.0,-0.0,0.0,1.0,-0.0,0.0,1.0,-0.0,0.0,1.0,-0.0,0.0,1.0,-0.0,0.0,1.0,2.2351736e-7,1.0,3.6660094e-8,2.9802322e-7,1.0,0.0,1.4901161e-7,1.0,0.0,1.4901163e-7,1.0,0.0,-1.4901163e-7,-0.99999994,-1.065166e-7,-2.9802322e-7,-1.0,-2.3808314e-14,-2.9802322e-7,-1.0,-3.174442e-14,-2.9802325e-7,-1.0,-2.3808317e-14,2.9802317e-7,1.0,5.6683664e-8,2.980232e-7,1.0,5.6683657e-8,2.2351739e-7,1.0,2.834183e-8,1.4901161e-7,1.0,4.223262e-15,-2.9802325e-7,-1.0,-3.2776682e-14,-2.9802325e-7,-1.0,-3.823946e-14,-2.9802325e-7,-1.0,-2.185112e-14,-2.9802325e-7,-1.0,-5.46278e-15,-2.9802325e-7,-1.0,-3.2124395e-14,-2.9802325e-7,-1.0,-3.7478464e-14,-5.960465e-7,-1.0,-5.354066e-14,-3.7252906e-7,-1.0,3.5930515e-8,2.9802317e-7,1.0,7.186105e-8,2.9802317e-7,1.0,7.186105e-8,2.2351736e-7,1.0,3.5930537e-8,1.4901158e-7,1.0,1.6062193e-14,-2.9802322e-7,-0.99999994,-1.1336733e-7,-2.9802322e-7,-0.99999994,-1.13367314e-7,-4.4703484e-7,-0.99999994,-5.6683657e-8,-2.9802322e-7,-0.99999994,0.0,2.9802317e-7,1.0,7.186101e-8,2.9802317e-7,1.0,7.186101e-8,2.2351739e-7,1.0,1.0779156e-7,1.490116e-7,1.0,1.4372209e-7,2.2351743e-7,1.0,5.32583e-8,2.9802328e-7,1.0,0.0,2.9802325e-7,1.0,-7.936106e-15,-0.0,0.99999994,0.0,-4.4703486e-7,-1.0,-7.1861066e-8,-2.9802325e-7,-1.0,0.0,-5.960465e-7,-1.0,0.0,-4.4703492e-7,-0.99999994,0.0,0.0,-0.0,1.0,0.0,-0.0,1.0,0.0,-0.0,1.0,0.0,-0.0,1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,0.0,0.0,-1.0,7.450581e-8,1.0,5.32583e-8,1.4901163e-7,1.0,1.065166e-7,2.9802325e-7,1.0,2.1303319e-7,1.4901163e-7,0.99999994,1.06516595e-7,-0.0,0.99999994,0.0,-0.0,0.99999994,0.0,-0.0,0.99999994,0.0,2.2351742e-7,1.0,5.3258304e-8,2.2351742e-7,1.0,5.3258297e-8,2.2351742e-7,1.0,1.597749e-7,2.9802322e-7,1.0,2.130332e-7,2.9802325e-7,1.0,-7.936106e-15,2.9802325e-7,1.0,0.0,2.9802325e-7,1.0,0.0,2.9802322e-7,1.0,0.0,2.9802322e-7,1.0,1.0651657e-7,2.9802325e-7,1.0,1.0651658e-7,4.4703486e-7,1.0,1.065166e-7,5.960465e-7,1.0,0.0,5.215407e-7,1.0,5.3258255e-8,3.725291e-7,1.0,5.3258276e-8,7.4505806e-8,1.0,5.3258297e-8,7.4505806e-8,1.0,5.3258297e-8,2.2351739e-7,1.0,5.325829e-8,2.2351742e-7,1.0,5.325828e-8,7.450582e-8,1.0,5.325829e-8,7.450583e-8,1.0,5.3258297e-8,2.2351743e-7,0.99999994,5.3258287e-8,0.0,-1.0,0.0,-7.4505806e-8,-1.0,-5.3258308e-8,-2.2351742e-7,-1.0,-5.325831e-8,-2.9802325e-7,-1.0,-2.3808317e-14,-2.9802322e-7,-1.0,-2.3808314e-14,-2.9802322e-7,-1.0,-1.587221e-14,-2.9802325e-7,-1.0,-3.1744423e-14,0.0,-1.0,0.0,0.0,-1.0,-1.0651661e-7,0.0,-1.0,-1.0651661e-7,-1.4901161e-7,-1.0,-1.06516616e-7,-2.9802322e-7,-1.0,-3.174442e-14,-2.9802322e-7,-1.0,-3.174442e-14,-2.9802322e-7,-1.0,-3.174442e-14,-2.2351742e-7,-1.0,-5.325831e-8,-2.2351742e-7,-1.0,-5.3258322e-8,-2.2351742e-7,-1.0,-1.5977493e-7,-2.9802322e-7,-1.0,-2.1303323e-7,-2.9802322e-7,-1.0,-2.3808314e-14,-2.9802322e-7,-1.0,-3.174442e-14,-2.9802322e-7,-1.0,-3.174442e-14,-7.450581e-8,-0.99999994,-5.32583e-8,0.0,-1.0,0.0,1.4901164e-7,-1.0,-1.0651658e-7,2.980233e-7,-1.0,-2.1303319e-7,1.4901165e-7,-1.0,-1.06516595e-7,0.0,-1.0,0.0,0.0,-1.0,0.0,1.0,-1.4901161e-8,5.3258297e-8,1.0,-4.4703487e-8,5.32583e-8,1.0,-7.450581e-8,5.3258297e-8,1.0,-1.3411045e-7,-5.3258326e-8,1.0,-2.0861626e-7,-1.0651663e-7,1.0,-2.5331974e-7,5.32583e-8,1.0,-2.8312206e-7,5.32583e-8,-1.4901161e-7,-1.0,-7.936105e-15,-1.4901163e-7,-1.0,0.0,-1.4901163e-7,-1.0,0.0,-1.4901163e-7,-0.99999994,-7.936106e-15,-2.2351746e-7,-1.0,5.325827e-8,-2.9802325e-7,-1.0,0.0,-2.9802325e-7,-0.99999994,-1.5872212e-14,-1.0,2.3841855e-7,0.0,-1.0,2.3841855e-7,0.0,-1.0,2.3841855e-7,0.0,-1.0,2.2351739e-7,-5.3258294e-8,-1.0,2.2351742e-7,-1.597749e-7,-1.0,2.3841858e-7,0.0,-1.0,2.3841858e-7,0.0,4.4703484e-7,1.0,1.0651662e-7,4.4703486e-7,1.0,1.065166e-7,1.4901163e-7,1.0,1.065166e-7,1.4901163e-7,0.99999994,1.065166e-7,2.980233e-7,1.0,1.5872215e-14,2.9802334e-7,1.0,0.0,2.9802334e-7,1.0,0.0],
                    "attribute_size": 3
                }
            },
            "bone_influences": {
                "bones_per_vertex": {
                    "NonUniform": [1,1,1,1,2,2,2,2,3,3,3,3,2,2,3,3,2,2,4,4,2,2,3,3,1,1,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,2,3,2,2,1,1,1,1,1,1,1,1,2,3,2,1,1,1,1,1,1,1,1,2,2,2,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,3,4,4,3,3,4,4,3,2,2,2,3,2,2,2,3,3,3,3,4,3,3,3,4,3,3,3,3,3,3,3,3,1,1,1,1,1,2,2,1,1,1,1,1,2,2,1,1,1,1,2,2,2,1,1,1,1,2,2,2,3,2,1,1,1,1,1,3,3,1,1,1,1,1,3,2,1,1,1,1,1,3,2,2,1,1,1,1,1,1,1,1,1,2,3,1,1,1,1,1,3,3,1,1,1,1,1,2,3,1,1,1,1,2,2,3]
                },
                "bone_indices": [0,0,0,0,1,2,1,2,1,2,1,2,0,1,2,0,1,2,1,2,3,1,2,3,1,2,1,2,1,2,3,1,2,3,0,1,0,1,0,1,2,3,0,1,2,3,0,1,0,1,0,1,3,0,1,3,3,3,3,3,2,2,2,2,0,3,0,3,3,3,3,3,3,3,3,3,1,3,0,1,3,0,3,0,3,3,3,3,3,3,3,3,3,1,3,0,1,3,1,2,2,2,2,2,2,2,2,2,1,2,1,2,1,2,2,2,2,1,2,1,2,2,2,2,0,0,0,0,0,0,0,0,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,0,1,2,0,1,2,3,0,1,2,3,1,2,3,0,1,2,0,1,2,3,0,1,2,3,1,2,3,1,2,1,2,1,2,1,2,3,1,2,1,2,1,2,1,2,3,0,1,3,0,1,3,0,1,3,0,1,2,3,0,1,3,0,1,3,0,1,3,0,1,2,3,0,1,3,0,1,3,0,1,3,0,1,3,0,1,3,0,1,3,0,1,3,0,1,3,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,1,0,1,0,0,0,0,0,3,0,3,0,3,0,0,0,0,0,3,0,3,0,3,0,1,3,0,1,0,0,0,0,0,0,1,3,0,1,3,0,0,0,0,0,0,1,3,0,3,0,0,0,0,0,0,1,3,0,3,0,3,0,0,0,0,0,0,0,0,0,0,1,0,1,3,0,0,0,0,0,0,1,3,0,1,3,0,0,0,0,0,0,3,0,1,3,0,0,0,0,0,3,0,3,0,1,3],
                "bone_weights": [0.99930483,0.9993267,0.9988469,0.9988882,0.5769585,0.41667557,0.5750416,0.41875833,0.08287623,0.9114626,0.0809439,0.9134449,0.052394267,0.88598174,0.050120413,0.052381232,0.88586444,0.050270952,0.6038401,0.21717286,0.16507955,0.60444063,0.2158349,0.16582415,0.8323886,0.14720343,0.8319144,0.14773157,0.1785458,0.7855405,0.015085268,0.18311681,0.77991176,0.017029043,0.22253782,0.7405494,0.22251287,0.740542,0.03593966,0.1644677,0.022933569,0.75609565,0.035559665,0.16221051,0.021494966,0.7592622,0.7325021,0.24111201,0.7324547,0.2411115,0.24104871,0.0171883,0.72107244,0.24646422,0.018294405,0.7150753,0.9979128,0.998135,0.99806494,0.99795896,0.9988047,0.9990297,0.99891806,0.9989621,0.10787421,0.8680739,0.045240935,0.9386733,0.9713774,0.9865636,0.9935986,0.9967849,0.9968572,0.9938901,0.98734754,0.9734771,0.014606806,0.9444226,0.0059462897,0.07222709,0.88400006,0.10569806,0.8703893,0.043480534,0.93957144,0.97172564,0.9866937,0.9936313,0.9967472,0.99692065,0.9939038,0.9873182,0.97335756,0.01533075,0.94405013,0.0057551004,0.07312979,0.88290054,0.024755668,0.9591396,0.98141676,0.99174035,0.99635047,0.9983034,0.99832433,0.9963541,0.99171793,0.9812937,0.025822569,0.9586212,0.07076583,0.915345,0.005161207,0.96714973,0.98713607,0.9948909,0.9978746,0.06926029,0.91721123,0.0041649975,0.9677716,0.9873414,0.99494904,0.99785924,0.99942935,0.99948114,0.99940383,0.99918497,0.9994379,0.99948144,0.99939644,0.9991656,0.52089864,0.47447312,0.4392018,0.55681866,0.2560472,0.73904645,0.14594676,0.84841776,0.52015466,0.47528905,0.43899456,0.5570616,0.25618187,0.73894024,0.14661598,0.8477633,0.035839584,0.8800026,0.05626422,0.014667104,0.86240315,0.06908087,0.022364985,0.00013051182,0.8054408,0.10276213,0.066731885,0.6983354,0.16329528,0.118703805,0.035808858,0.87989175,0.05644027,0.014632608,0.862282,0.069321305,0.022160817,0.00009224564,0.80524856,0.103189684,0.06651563,0.69797367,0.1641143,0.11826125,0.81223685,0.16827872,0.76778257,0.21257482,0.63745654,0.33860016,0.33775267,0.63089234,0.0029583946,0.8118751,0.16870442,0.7674167,0.21303833,0.6367719,0.33948454,0.33603087,0.63306826,0.0021218061,0.19665678,0.73637927,0.052574046,0.16256312,0.7119954,0.10919973,0.13064471,0.60218906,0.24547167,0.0899343,0.31702027,0.010822326,0.5626343,0.19670737,0.7363473,0.052616917,0.16264729,0.7118822,0.109325536,0.13077486,0.6018658,0.24584669,0.09006711,0.31613845,0.010086797,0.5637511,0.73304653,0.21381596,0.04714686,0.7181039,0.17633335,0.10093921,0.62774724,0.13753818,0.22940728,0.3697934,0.08491707,0.5393966,0.73318374,0.21381065,0.046922278,0.71842223,0.17632438,0.10065594,0.6285407,0.13754272,0.22864479,0.37184832,0.08502152,0.5372724,0.9988633,0.99763453,0.99480253,0.9884909,0.974564,0.94401485,0.039739665,0.87734264,0.10453632,0.99887,0.9976349,0.9947986,0.98848146,0.97454613,0.9439851,0.039737288,0.8773002,0.104535356,0.99794656,0.99544877,0.9894214,0.97517526,0.9416019,0.052505236,0.8625044,0.12556209,0.6763853,0.3008956,0.997966,0.99546796,0.9894605,0.9752745,0.9418646,0.052216694,0.8632105,0.12478506,0.6783151,0.29876667,0.8791952,0.08709458,0.013530838,0.9455255,0.021888305,0.9755683,0.9890888,0.9951352,0.99781674,0.9989788,0.87278545,0.06643052,0.058952127,0.9434719,0.002389647,0.009188458,0.9751048,0.9890814,0.9952143,0.99788773,0.99903166,0.8324441,0.05029952,0.115298815,0.9272245,0.052366655,0.9686239,0.9864858,0.9941699,0.99746424,0.99885434,0.7470455,0.021280993,0.21514823,0.89428943,0.08959707,0.95521534,0.025550488,0.98092675,0.9918466,0.9964818,0.9984181,0.99897635,0.9978189,0.99514294,0.98910654,0.975605,0.94559467,0.021895174,0.8793071,0.08709712,0.013317416,0.9990334,0.9978928,0.9952266,0.98911005,0.9751686,0.9436032,0.002405785,0.008915171,0.8730211,0.06643922,0.058716524,0.9988606,0.99747336,0.99418974,0.9865333,0.9687366,0.9274788,0.052095838,0.83295697,0.05032899,0.11476577,0.99843067,0.9964965,0.99187726,0.98100257,0.9554063,0.025134359,0.8947603,0.089086145,0.7481345,0.021453265,0.213979]
            }
        },        
        "armature_name": "LetterFArmature",
        "bounding_box": {
            "min_corner": [-0.5135834217071533, -0.12500007450580597, 0.0],
            "max_corner": [0.12500005960464478, 0.12500011920928955, 1.0]
        },
        "materials": {}
    }
        "#.to_string()
}

fn expected_armature_data() -> String {
    r#"
    {
        "actions": {
          "Twist": [
            {"frame_time_secs": 0.0, "bones": [{"Matrix": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [1.0, 0.0, 2.3841855067985307e-07, 0.0, 2.3841855067985307e-07, 0.0, -1.0, 0.0, 0.0, 0.9999999403953552, 0.0, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-5.684341886080802e-14, -1.0, 3.789253558551895e-15, 0.0, 1.0, 0.0, 4.3711366970455856e-08, 0.0, -4.371136341774218e-08, 0.0, 0.9999999403953552, 0.938409686088562, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-7.450580596923828e-08, -0.9993491172790527, 0.03607500344514847, 0.0, 1.0, 0.0, 1.6292068494294654e-07, 0.0, -1.6042031347751617e-07, 0.03607500344514847, 0.9993491172790527, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}]},
            {"frame_time_secs": 0.833333, "bones": [{"Matrix": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [1.0, 0.0, 2.3841855067985307e-07, 0.0, 2.3841855067985307e-07, 0.0, -1.0, 0.0, 0.0, 0.9999999403953552, 0.0, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-0.7071068286895752, -0.7071067690849304, -3.0908601900137e-08, 0.0, 0.7071067690849304, -0.7071068286895752, 3.090860545285068e-08, 0.0, -4.3711366970455856e-08, 3.552713678800501e-15, 0.9999999403953552, 0.938409686088562, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [0.7071068286895752, -0.7066465020179749, 0.025508996099233627, 0.0, 0.7071067094802856, 0.7066466212272644, -0.025508765131235123, 0.0, -1.6391277313232422e-07, 0.03607500344514847, 0.9993491172790527, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}]},
            {"frame_time_secs": 1.666667, "bones": [{"Matrix": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [1.0, 0.0, 2.3841855067985307e-07, 0.0, 2.3841855067985307e-07, 0.0, -1.0, 0.0, 0.0, 0.9999999403953552, 0.0, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-5.684341886080802e-14, -1.0, 3.789253558551895e-15, 0.0, 1.0, 0.0, 4.3711366970455856e-08, 0.0, -4.371136341774218e-08, 0.0, 0.9999999403953552, 0.938409686088562, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-7.450580596923828e-08, -0.9993491172790527, 0.03607500344514847, 0.0, 1.0, 0.0, 1.6292068494294654e-07, 0.0, -1.6042031347751617e-07, 0.03607500344514847, 0.9993491172790527, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}]},
            {"frame_time_secs": 2.5, "bones": [{"Matrix": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [0.7071067690849304, 0.7071068286895752, 1.6858736273661634e-07, 0.0, 2.3841855067985307e-07, 0.0, -1.0, 0.0, -0.7071067690849304, 0.7071067094802856, -1.6858736273661634e-07, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-3.090863742727379e-08, -0.38268327713012695, 0.9238796234130859, 0.24215157330036163, 1.0, 1.672762195426003e-08, 4.038403744743846e-08, 0.0, -3.090856637300021e-08, 0.9238795638084412, 0.38268324732780457, 0.8381072282791138, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-7.450580596923828e-08, -0.9993491172790527, 0.03607500344514847, 0.0, 1.0, 0.0, 1.6292068494294654e-07, 0.0, -1.6042031347751617e-07, 0.03607500344514847, 0.9993491172790527, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}]},
            {"frame_time_secs": 3.333333, "bones": [{"Matrix": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [0.9238795042037964, 0.3826834261417389, 2.0212148399423313e-07, 0.0, 2.5656706270638097e-07, -9.123882449557641e-08, -1.0, 0.0, -0.3826833963394165, 0.9238794445991516, -1.824776347802981e-07, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-2.2235541408122117e-08, -0.3826833963394165, 0.9238795638084412, 0.13105148077011108, 1.0, -7.734853113561257e-08, -7.971307169896136e-09, -3.124510072893827e-08, 7.451123451573949e-08, 0.9238795042037964, 0.38268333673477173, 0.9123419523239136, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-7.450580596923828e-08, -0.9993491172790527, 0.03607500344514847, 0.0, 1.0, 0.0, 1.6292068494294654e-07, 0.0, -1.6042031347751617e-07, 0.03607500344514847, 0.9993491172790527, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}]},
            {"frame_time_secs": 4.166667, "bones": [{"Matrix": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [1.0, 0.0, 2.3841855067985307e-07, 0.0, 2.3841855067985307e-07, 0.0, -1.0, 0.0, 0.0, 0.9999999403953552, 0.0, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-5.684341886080802e-14, -1.0, 3.789253558551895e-15, 0.0, 1.0, 0.0, 4.3711366970455856e-08, 0.0, -4.371136341774218e-08, 0.0, 0.9999999403953552, 0.938409686088562, 0.0, 0.0, 0.0, 1.0]}, {"Matrix": [-7.450580596923828e-08, -0.9993491172790527, 0.03607500344514847, 0.0, 1.0, 0.0, 1.6292068494294654e-07, 0.0, -1.6042031347751617e-07, 0.03607500344514847, 0.9993491172790527, 0.5959557294845581, 0.0, 0.0, 0.0, 1.0]}]}
          ]
        },
        "inverse_bind_poses": [
            {"Matrix": [1.0, -0.0, 0.0, -0.0, -0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, -0.0, -0.0, 0.0, -0.0, 1.0]},
            {"Matrix": [1.0, 2.3841855067985307e-07, 0.0, -0.0, -0.0, 0.0, 1.0000001192092896, -0.5959557890892029, 2.3841855067985307e-07, -1.0, 0.0, -0.0, -0.0, 0.0, -0.0, 1.0]},
            {"Matrix": [0.0, 1.0, -4.3711370523169535e-08, 4.101917383536602e-08, -1.0, -5.684341886080802e-14, 3.7892565231672105e-15, -3.5558752292761276e-15, 0.0, 4.3711366970455856e-08, 1.0000001192092896, -0.9384097456932068, -0.0, -0.0, -0.0, 1.0]},
            {"Matrix": [-5.8773643729637115e-09, 1.0, -1.628146435450617e-07, 9.703032333163719e-08, -0.9993491172790527, -6.867014690215001e-08, 0.03607500344514847, -0.02149910479784012, 0.03607500344514847, 1.6300369054533803e-07, 0.9993491172790527, -0.5955678224563599, -0.0, -0.0, -0.0, 1.0]}
        ],
        "joint_index": {
            "Lower.Body": 0,"Upper.Body": 1,"Upper.Arm": 2,"Lower.Arm": 3
        },
        "bone_groups": {}
    }
    "#.to_string()
}
