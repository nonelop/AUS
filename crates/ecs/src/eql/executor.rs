// fn type_linker(world: &World, type_name: &str) -> Option<TypeId> {
//     for (raw_name, type_id) in &world.type_names {
//         let name: Vec<&str> = raw_name.split("::").collect();
//         let name = name[name.len() - 1];

//         if type_name == name {
//             return Some(*type_id);
//         }
//     }
//     None
// }
