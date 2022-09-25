extern crate tobj;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

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

fn input(i_txt: &str) -> String {

    print!("{}", i_txt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
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

    let json_file:String = name +".json";
    let file = File::create(json_file.clone()).unwrap();
    let serialized = serde_json::to_string(&mesh_data).unwrap();
    serde_json::to_writer(file, &serialized).unwrap();
    println!("generated: {}", json_file);
    
}
