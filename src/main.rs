extern crate tobj;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::u8;
use std::vec::Vec;

use std::fs::File;

use std::io::{self, Write};

use std::path::Path;

 


#[derive(Serialize, Deserialize)]
struct MeshData {
    position:Vec<f32>,
    normals:Vec<f32>,
    color:Vec<f32>
}

struct AssetFile {
    file_type: String,
    version: i32,
    json: String,
    binary_blob: Vec<String>
}

fn input(i_txt: &str) -> String {

    print!("{}", i_txt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}


fn save_binary(name: &str, file: AssetFile, custom_path:Option<&str>) -> bool{
    let out_file_name:String;
    if custom_path.is_some(){
        out_file_name = custom_path.unwrap().to_string() + &name.to_string() + "." + &file.file_type 
    }
    else {
        out_file_name = name.to_string() + "." + &file.file_type;
    }
    
    let mut out_file = File::create(out_file_name).unwrap();
    out_file.write(file.file_type.as_bytes()).unwrap();

    //version
    let version = file.version;
    out_file.write(&version.to_ne_bytes()).unwrap();


    //json length
    let length = file.json.len();
    out_file.write(&length.to_ne_bytes()).unwrap();
    
    //json stream
    out_file.write(file.json.as_bytes()).unwrap();

    //binary blob length
    let blob_size = file.binary_blob.len();
    out_file.write(&blob_size.to_ne_bytes()).unwrap();

    //blob data 
    for i in file.binary_blob{
        out_file.write(i.as_bytes()).unwrap();
    }

    return true;
}

fn main() {

    let obj_file = input("select file to import: "); 
    //that's a long function
    let name = Path::new(&obj_file).file_stem().unwrap().to_os_string().into_string().unwrap();


    let (models, _materials) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");
    
    let mut mesh_data:MeshData = MeshData { position: Vec::from([0.0]), normals: Vec::from([0.0]), color: Vec::from([0.0]) };
    for m in &models {
        let mesh= &m.mesh;
        mesh_data = MeshData{
            position: mesh.positions.clone(),
            normals: mesh.normals.clone(),
            color: mesh.vertex_color.clone()
        };
    }

    let binary_file:AssetFile = AssetFile{ file_type: "Mesh".to_string(), version: 1, json: serde_json::to_string(&mesh_data).unwrap(), binary_blob: [].to_vec() };

    save_binary(&name, binary_file, Some("/home/alexander/ProgrammingProjects/Rust/FileConveretr-rs/samples/output/"));
    
}
