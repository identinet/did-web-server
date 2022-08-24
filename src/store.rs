use std::path::PathBuf;

/// Computes the absolute path to a file with json extension in a base
/// direcotory and an ID.
pub fn get_filename_from_id<'a>(base_dir: &PathBuf, id: &PathBuf) -> Result<PathBuf, &'a str> {
    // Path::new(id)
    //     .file_name()
    //     // DEBUG
    //     // .map(|x| {
    //     //     let y = x.to_str();
    //     //     match y {
    //     //         Some(s) => println!("res {}, {}", s, id),
    //     //         None => println!("nothing"),
    //     //     };
    //     //     x
    //     // })
    //     //  bail out if id contains more information than than the plain file name
    //     .and_then(|id_file| if id_file == id { Some(id_file) } else { None })
    //     .ok_or("id is not a file") // TODO: place error messages in a constant somewhere
    //     .and_then(|id_file| {
    //         let p = Path::new(base_dir).join(id_file).with_extension("json");
    //         if p.is_absolute() {
    //             Ok(p)
    //         } else {
    //             Err("Path not absolute")
    //         }
    //     })
    id.file_name()
        // .map(|x| {
        //     match x.to_str() {
        //         Some(s) => println!("res {}, {}", s, "id"),
        //         None => println!("nothing"),
        //     };
        //     x
        // })
        .and_then(|filename| {
            filename
                .to_str()
                .and_then(|_f| if _f == "did.json" { id.parent() } else { None })
        })
        // .map(|x| {
        //     match x.to_str() {
        //         Some(s) => println!("parent path res {}, {}", s, "id"),
        //         None => println!("partent path nothing"),
        //     };
        //     x
        // })
        .ok_or("id not found, path doesn't end in /did.json")
        .and_then(|id_file| {
            let p = base_dir.join(id_file).with_extension("json");
            if p.is_absolute() {
                Ok(p)
            } else {
                Err("Path not absolute")
            }
        })
}

#[cfg(test)]
mod test {
    use crate::store::*;

    #[test]
    fn test_get_filename_from_id() {
        let id = PathBuf::from("");
        let base_dir = PathBuf::from(".");
        let result = get_filename_from_id(&base_dir, &id);
        assert_eq!(
            result,
            Err("id not found, path doesn't end in /did.json"),
            "When <id> is empty, then an error is returned"
        );

        let id = PathBuf::from("abc/did.json");
        let base_dir = PathBuf::from(".");
        let result = get_filename_from_id(&base_dir, &id);
        assert_eq!(
            result,
            Err("Path not absolute"),
            "When resulting path is not absolute, then an error is returned"
        );

        // let id = PathBuf::from("../abc/did.json");
        // let base_dir = PathBuf::from(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids"));
        // let result = get_filename_from_id(&base_dir, &id);
        // assert_eq!(
        //     result,
        //     Err("id not found, path doesn't end in /did.json"),
        //     "When <id> contains additional characters that are not part of the filename, e.g. a relative path, then return an error"
        // );

        let id = PathBuf::from("abc/did.json");
        let base_dir = PathBuf::from(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/dids"));
        let id_with_extension = "abc.json";
        let result = get_filename_from_id(&base_dir, &id);
        match result {
            Ok(r) => assert_eq!(
                r,
                base_dir.join(id_with_extension),
                "When <id> and <base_dir> can be combined to an absolute path, then succeed"
            ),
            Err(_) => {
                panic!("When <id> and <base_dir> can be combined to an absolute path, then succeed")
            }
        }
    }
}
