use crate::utils::config::{self, active_registry};
use cli_table::{print_stdout, Cell, Style, Table};

pub fn list_(print_json: &bool) {
    let registries = config::registries();
    let active_reg_name = active_registry();

    let data: Vec<Vec<&str>> = registries
        .iter()
        .map(|reg| {
            vec![
                reg.name.as_str(),
                reg.endpoint.as_str(),
                match reg.name == active_reg_name {
                    true => "true",
                    false => "false",
                },
            ]
        })
        .collect();

    if *print_json {
        let json_data = serde_json::to_string_pretty(&registries).expect("JSON string");
        println!("{}", json_data);
        return;
    }

    let table = data
        .iter()
        .map(|reg| {
            vec![
                reg[0].cell(),
                reg[1].cell().italic(true),
                match reg[2] {
                    "true" => "✅".cell(),
                    _ => "".cell(),
                },
            ]
        })
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Endpoint".cell().bold(true),
            "Active?".cell().bold(true),
        ])
        .bold(true);
    assert!(print_stdout(table).is_ok());
}
