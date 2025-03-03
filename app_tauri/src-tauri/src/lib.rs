mod dto;
mod file_op;
use dto::{Method, State};
use file_op::{copy_directory, replace_text_in_file, replace_text_in_file_regex, exists_base_template};
use std::{env, fs, io, vec};
use tauri::http::method;

fn copy_base_template(source_dir: &str, destination_dir: &str) -> io::Result<()> {
    println!(
        "Copying directory '{}' to '{}'...",
        source_dir, destination_dir
    );

    copy_directory(source_dir.to_string(), destination_dir.to_string())?;

    Ok(())
}

fn replace_in_class(dist: &String, state: &State) -> io::Result<()> {
    let file_path = dist.to_owned() + "\\cpp\\source\\AddInNative.h";
    let target_text = "//ДляВставкиМетодов";

    let methods_string = state
        .methods
        .iter()
        .map(|method| format!("{}_enum", method.name_eng.as_str()))
        .collect::<Vec<String>>()
        .join(",\n\t\t")
        + ",";

    replace_text_in_file(&file_path, target_text.to_string(), methods_string)?;

    Ok(())
}

fn replace_in_make_file(dist: &String, state: &State) -> io::Result<()> {
    let file_path = dist.to_owned() + "\\cpp\\source\\CMakeLists.txt";
    let target_text = "#ВставкаCPPФайлов";

    let cpp_files_string = state
        .methods
        .iter()
        .map(|method| "impl/".to_owned() + method.name_eng.as_str() + ".cpp")
        .collect::<Vec<String>>()
        .join("\n\t");

    replace_text_in_file(&file_path, target_text.to_string(), cpp_files_string)?;

    Ok(())
}

fn replace_in_main_cpp(dist: &String, state: &State) -> io::Result<()> {
    let file_path = dist.to_owned() + "\\cpp\\source\\AddInNative.cpp";
    let target_text = "/*ФайлCPPМетодыНаРусскомЯзыке*/";

    let methods_string = state
        .methods
        .iter()
        .map(|method| format!("L\"{}\"", method.name.as_str()))
        .collect::<Vec<String>>()
        .join(",\n\t")
        + ",\n";

    replace_text_in_file(&file_path, target_text.to_string(), methods_string)?;

    let file_path = dist.to_owned() + "\\cpp\\source\\AddInNative.cpp";
    let target_text = "/*ФайлCPPМетодыНаАнглийскомЯзыке*/";

    let methods_string = state
        .methods
        .iter()
        .map(|method| format!("L\"{}\"", method.name_eng.as_str()))
        .collect::<Vec<String>>()
        .join(",\n\t")
        + ",\n";

    replace_text_in_file(&file_path, target_text.to_string(), methods_string)?;

    let target_text = "//GetNParamsДляВставки";

    let methods_string = state
        .methods
        .iter()
        .map(|method| {
            format!(
                "case {}: return {};",
                format!("{}_enum", method.name_eng.as_str()),
                method.params.len()
            )
        })
        .collect::<Vec<String>>()
        .join("\n\t");

    replace_text_in_file(&file_path, target_text.to_string(), methods_string)?;

    let target_text = "//HasRetValДляВставки";

    let methods_string = state
        .methods
        .iter()
        .map(|method| {
            format!(
                "case {}: return {};",
                format!("{}_enum", method.name_eng.as_str()),
                method.has_return
            )
        })
        .collect::<Vec<String>>()
        .join("\n\t");

    replace_text_in_file(&file_path, target_text.to_string(), methods_string)?;

    let target_text = "//CallAsFuncДляВставки";

    let methods_string = state
        .methods
        .iter()
        .map(|method| {
            format!(
                "case {}: return {}(lMethodNum, pvarRetValue, paParams, lSizeArray, m_iMemory);",
                format!("{}_enum", method.name_eng.as_str()),
                method.name_eng.as_str()
            )
        })
        .collect::<Vec<String>>()
        .join("\n\t");

    replace_text_in_file(&file_path, target_text.to_string(), methods_string)?;

    let target_text = "//includeВставкаЗаголовковМетодов";

    let methods_string = state
        .methods
        .iter()
        .map(|method| format!("#include \"impl/{}.h\"", method.name_eng.as_str()))
        .collect::<Vec<String>>()
        .join("\n");

    replace_text_in_file(&file_path, target_text.to_string(), methods_string)?;

    Ok(())
}

fn fill_params_methods(file_path: &String, state: &State, method: &Method) -> io::Result<()> {
    let mut get_params: Vec<String> = vec![];

    method.params.iter().enumerate().for_each(|(index, param)| {
        if param._type == "string" {
            get_params.push(
                format!(
                    "std::string {} = get_method_param_as_utf8(paParams, {});",
                    param.name, index
                )
                .to_string(),
            );
        }
        if param._type == "number" {
            get_params.push(
                format!(
                    "float {} = get_method_param_as_number(paParams, {});",
                    param.name, index
                )
                .to_string(),
            );
        }
        if param._type == "bool" {
            get_params.push(
                format!(
                    "bool {} = get_method_param_as_bool(paParams, {});",
                    param.name, index
                )
                .to_string(),
            );
        }
    });

    let replace_str = get_params.join("\n\t");
    replace_text_in_file(
        file_path,
        "//ВставкаКодаПолученияПараметровМетода".to_string(),
        replace_str,
    )?;

    replace_text_in_file_regex(file_path, r"//\+\+\+НачалоПримера[\S\s\n]*?//---", "")?;

    if !method.call_rust_method {
        replace_text_in_file(
            file_path,
            "free_mem_after_cpp(res);//Освободить память выделенные в Rust, когда она больше не нужна на стороне cpp".to_string(),
            "".to_string(),
        )?;

        replace_text_in_file_regex(
            file_path,
            r"//\+\+\+Вызов метода Rust[\S\s\n]*?//---",
            "set_return_val_for_1c_as_utf16(pvarRetValue, u\"returned value\", m_iMemory);",
        )?;
    }

    if method.call_rust_method {
        let mut rust_params: Vec<String> = vec![];

        method.params.iter().for_each(|param| {
            if param._type == "string" {
                rust_params.push(param.name.to_string() + ".c_str()");
            } else {
                rust_params.push(param.name.to_string());
            }
        });

        replace_text_in_file(
            &file_path,
            "const char* res =  test__call_from_cpp(parm_for_rust.c_str(), f, b);".to_string(),
            format!(
                "const char* res =  {}__call_from_cpp({});",
                method.name_eng.as_str(),
                rust_params.join(", ").to_string()
            ),
        )?;
    }

    Ok(())
}

fn fill_for_rust_header(file_path: &String, state: &State) -> io::Result<()> {
    let mut methods: Vec<String> = vec![];

    state.methods.iter().for_each(|method| {
        let mut params: Vec<String> = vec![];

        method.params.iter().for_each(|param| {
            if param._type == "string" {
                params.push(format!("const char* {}", param.name).to_string());
            }
            if param._type == "number" {
                params.push(format!("float {}", param.name).to_string());
            }
            if param._type == "bool" {
                params.push(format!("bool {}", param.name).to_string());
            }
        });

        let cur_method = format!(
            "extern \"C\" const char* {}__call_from_cpp({});",
            method.name_eng.as_str(),
            params.join(", ")
        );

        methods.push(cur_method.clone());
    });

    replace_text_in_file(
        file_path,
        "//ВставкаМетодов".to_string(),
        methods.join("\n").to_string(),
    )?;

    Ok(())
}

fn copy_cpp_files_for_each_method(dist: &String, state: &State) -> io::Result<()> {
    state.methods.iter().try_for_each(|method| {
        let source = dist.to_owned() + "\\cpp\\source\\impl\\test.cpp";
        let dist = dist.to_owned() + "\\cpp\\source\\impl\\" + method.name_eng.as_str() + ".cpp";
        println!("Copying file '{}' to '{}'...", source, dist);
        fs::copy(source, &dist)?;
        let method_name = method.name_eng.as_str().to_owned() + "(";
        replace_text_in_file(
            &dist.to_string(),
            "test(".to_string(),
            method_name.to_string(),
        )?;
        fill_params_methods(&dist.to_string(), &state, method)?;
        Ok::<(), io::Error>(())
    })?;

    state.methods.iter().try_for_each(|method| {
        let source = dist.to_owned() + "\\cpp\\source\\impl\\test.h";
        let dist = dist.to_owned() + "\\cpp\\source\\impl\\" + method.name_eng.as_str() + ".h";
        println!("Copying file '{}' to '{}'...", source, dist);
        fs::copy(source, &dist)?;
        let method_name = method.name_eng.as_str().to_owned() + "(";
        replace_text_in_file(
            &dist.to_string(),
            "test(".to_string(),
            method_name.to_string(),
        )?;
        Ok::<(), io::Error>(())
    })?;

    let source = dist.to_owned() + "\\cpp\\source\\impl\\rust.h";
    fill_for_rust_header(&source, &state)?;

    Ok(())
}

fn copy_rs_files_for_each_method(dist: &String, state: &State) -> io::Result<()> {
    state
        .methods
        .iter()
        .filter(|method| method.call_rust_method)
        .try_for_each(|method| {
            let source = format!("{}\\rust\\src\\impl_test.rs", dist);
            println!("1_{}", source);
            let dist = format!("{}\\rust\\src\\impl_{}.rs", dist, method.name_eng);
            println!("Copying file '{}' to '{}'...", source, dist);
            fs::copy(source, &dist)?;
            let method_name = method.name_eng.as_str().to_owned() + "(";
            replace_text_in_file(
                &dist.to_string(),
                "test(".to_string(),
                method_name.to_string(),
            )?;

            let mut params: Vec<String> = vec![];

            method.params.iter().for_each(|param| {
                if param._type == "string" {
                    params.push(format!("{}: *const c_char", param.name));
                }
                if param._type == "number" {
                    params.push(format!("{}: f32", param.name));
                }
                if param._type == "bool" {
                    params.push(format!("{}: bool", param.name));
                }
            });
            let new_text = format!(
                r#"pub extern "C" fn main({}) -> *const c_char {{
    str_to_cchar("returned value from rust")
}}"#,
                params.join(", ")
            );
            replace_text_in_file_regex(&dist, r"//\+\+\+Заменить[\S\s\n]*?//---", &new_text)?;

            Ok::<(), io::Error>(())
        })?;

    // заменить в lib.rs
    let mut mods: Vec<String> = vec![];

    state
        .methods
        .iter()
        .filter(|method| method.call_rust_method)
        .for_each(|method| {
            mods.push(format!("mod impl_{};", method.name_eng));
        });

    let file_path = format!("{}\\rust\\src\\lib.rs", dist);
    replace_text_in_file(&file_path, "//ВставкаМодулей".to_string(), mods.join("\n"))?;

    //ВставкаМетодов

    let mut methods: Vec<String> = vec![];

    state
        .methods
        .iter()
        .filter(|method| method.call_rust_method)
        .for_each(|method| {
            let mut params: Vec<String> = vec![];
            let mut params_without_types: Vec<String> = vec![];
            method.params.iter().for_each(|param| {
                params_without_types.push(param.name.to_string());
                if param._type == "string" {
                    params.push(format!("{}: *const c_char", param.name));
                }
                if param._type == "number" {
                    params.push(format!("{}: f32", param.name));
                }
                if param._type == "bool" {
                    params.push(format!("{}: bool", param.name));
                }
            });

            let code = format!(
                r###"#[no_mangle]
pub extern "C" fn {}__call_from_cpp({}) -> *const c_char {{
    impl_{}::main({})
}}"###,
                method.name_eng,
                params.join(", "),
                method.name_eng,
                params_without_types.join(", "),
            );
            methods.push(code);
        });

        let file_path = format!("{}\\rust\\src\\lib.rs", dist);
        replace_text_in_file(&file_path, "//ВставкаМетодов".to_string(), methods.join("\n"))?;

    Ok(())
}

fn copy_file_and_replace(path: String, state: State) -> io::Result<()> {
    let source = path.to_owned() + "\\base_template";
    let dist = path.to_owned() + "\\new_component";

    copy_base_template(source.as_str(), dist.as_str())?;

    replace_in_class(&dist, &state)?;

    replace_in_main_cpp(&dist, &state)?;

    copy_cpp_files_for_each_method(&dist, &state)?;

    copy_rs_files_for_each_method(&dist, &state)?;

    replace_in_make_file(&dist, &state)?;

    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn build(path: &str, state: &str) -> String {
    println!("state {}", state);

    let state_struct = match serde_json::from_str::<State>(state) {
        Ok(state) => state,
        Err(e) => {
            println!("error {}", e);
            return e.to_string();
        }
    };

    let source = path.to_owned() + "\\base_template";
    if !exists_base_template(source.to_string()){
        return "В каталоге с конструтором должен находится каталог base_template. Это базой шаблон. Он не найден. Скопируйте его в этот каталог из релиза или из папки source в репозитории.".to_string();
    }

    match copy_file_and_replace(path.to_string(), state_struct) {
        Ok(()) => "Завершилось успешно!".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
fn current_dir() -> String {

    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(e) => e.to_string(),
    }

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![build,current_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}