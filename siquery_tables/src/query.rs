
pub use sys::{SystemInfo, SystemReader};
use sys::{SystemReaderInterface};

use std::borrow::Borrow;

use tables::*;

use vtab::*;
use rusqlite::{version_number, Connection, Result, Error};

fn select_all<T>(table: &Vec<T>) -> Vec<Vec<String>> where T:Table+Sized {
    let mut res: Vec<Vec<String>> = Vec::new();

    let cols = table.column_names();

    let mut id: u64 = 1;

    let mut columns_id: Vec<u64> = Vec::new();

    for _col in cols.iter() {
        columns_id.push(id);
        id = id << 1;
    }

    for tab in table.iter() {
        let mut row: Vec<String> = Vec::new();
        for id in columns_id.iter() {
            row.push(tab.get_by_id(*id));
        }
        res.push(row);
    }
    res
}

fn select<T>(table: &Vec<T>, columns: Vec<String>) -> Vec<Vec<String>> where T:Table+Sized {

    if columns.len() < 1 {
        return select_all(table);
    }

    let mut res: Vec<Vec<String>> = Vec::new();

    let mut columns_id: Vec<u64> = Vec::new();

    for column in columns.iter() {
        // make sure the header exist in the table
        let id = table[0].get_id(column);
        if id != 0 {
            columns_id.push(id);
        }
    }

    for tab in table.iter() {
        let mut row: Vec<String> = Vec::new();
        for id in columns_id.iter() {
            row.push(tab.get_by_id(*id));
        }
        res.push(row);
    }
    res
}

pub fn query_table(name: &str, columns: Vec<String>) -> Vec<Vec<String>> {
    let system_reader: Box<SystemReaderInterface> = Box::new(SystemReader::new());
    let res = match name {
        "etc_hosts" => {
            let table = EtcHosts::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "etc_protocols" => {
            let table = EtcProtocols::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "etc_services" => {
            let table = EtcServices::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "system_info" => {
            let table = SystemInfoData::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "os_version" => {
            let table = OsVersion::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "logical_drives" => {
            let table = LogicalDrive::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_address" => {
            let table = InterfaceAddress::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_details" => {
            let table = InterfaceDetails::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "uptime" => {
            let table = Uptime::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "products" => {
            let table = Products::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_computer_info" => {
            let table = WmiComputerInfo::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_os_version" => {
            let table = WmiOsVersion::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_printers" => {
            let table = WmiPrinters::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_services" => {
            let table = WmiServices::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_hotfixes" => {
            let table = WmiHotfixes::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_shares" => {
            let table = WmiShares::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_network_adapters" => {
            let table = WmiNetworkAdapters::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_local_accounts" => {
            let table = WmiLocalAccounts::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_bios" => {
            let wmi_bios = WmiBios::get_specific(system_reader.borrow());
            let mut table: Vec<WmiBios> = Vec::new();
            table.push(wmi_bios);
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_motherboard" => {
            let table = WmiMotherboard::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_processor" => {
            let table = WmiProcessor::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_physical_memory" => {
            let table = WmiMemory::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_sound" => {
            let table = WmiSound::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_video" => {
            let table = WmiVideo::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_monitors" => {
            let table = WmiMonitors::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_keyboard" => {
            let table = WmiKeyboard::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_pointing_device" => {
            let table = WmiPointingDevice::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_open_sockets" => {
            let table = ProcessOpenSocketsRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        "processes" => {
            let table = ProcessesRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_memory_map" => {
            let table = ProcessMemoryMapRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        #[cfg(not(target_os = "windows"))]
        "process_envs" => {
            let table = ProcessEnvsRow::get_specific(system_reader.borrow());
            select(&table, columns)
        },
        _ => {
            let table: Vec<Dummy> = Vec::new();
            select(&table, columns)
        }
    };
    res
}

fn select_header<T>(table: &Vec<T>, columns: Vec<String>) -> Vec<String> where T:Table+Sized {
    let mut hdr: Vec<String> = Vec::new();
    if columns.len() < 1 {
        for col in table.column_names().iter(){
            hdr.push(col.to_string());
        }
        return hdr;
    }

    for column in columns.iter() {
        // make sure the header exist in the table
        let id = table[0].get_id(column);
        if id != 0 {
            hdr.push(column.to_string());
        }
    }

    hdr
}

pub fn query_header(name: &str, columns: Vec<String>) -> Vec<String> {
    let system_reader: Box<SystemReaderInterface> = Box::new(SystemReader::new());
    let res = match name {
        "etc_hosts" => {
            let table = EtcHosts::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        "etc_protocols" => {
            let table = EtcProtocols::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        "etc_services" => {
            let table = EtcServices::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        "system_info" => {
            let table = SystemInfoData::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        "os_version" => {
            let table = OsVersion::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        "logical_drives" => {
            let table = LogicalDrive::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_address" => {
            let table = InterfaceAddress::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "interface_details" => {
            let table = InterfaceDetails::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        "uptime" => {
            let table = Uptime::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "products" => {
            let table = Products::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_computer_info" => {
            let table = WmiComputerInfo::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_os_version" => {
            let table = WmiOsVersion::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_printers" => {
            let table = WmiPrinters::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_services" => {
            let table = WmiServices::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_hotfixes" => {
            let table = WmiHotfixes::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_shares" => {
            let table = WmiShares::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_network_adapters" => {
            let table = WmiNetworkAdapters::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_local_accounts" => {
            let table = WmiLocalAccounts::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_bios" => {
            let wmi_bios = WmiBios::get_specific(system_reader.borrow());
            let mut table: Vec<WmiBios> = Vec::new();
            table.push(wmi_bios);
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_motherboard" => {
            let table = WmiMotherboard::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_processor" => {
            let table = WmiProcessor::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_physical_memory" => {
            let table = WmiMemory::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_sound" => {
            let table = WmiSound::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_video" => {
            let table = WmiVideo::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_monitors" => {
            let table = WmiMonitors::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_keyboard" => {
            let table = WmiKeyboard::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(target_os = "windows")]
        "wmi_pointing_device" => {
            let table = WmiPointingDevice::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_open_sockets" => {
            let table = ProcessOpenSocketsRow::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        "processes" => {
            let table = ProcessesRow::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "macos"))]
        "process_memory_map" => {
            let table = ProcessMemoryMapRow::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        #[cfg(not(target_os = "windows"))]
        "process_envs" => {
            let table = ProcessEnvsRow::get_specific(system_reader.borrow());
            select_header(&table, columns)
        },
        _ => {
            let table: Vec<Dummy> = Vec::new();
            select_header(&table, columns)
        }
    };
    res
}

pub fn get_table_list() -> Vec<String> {
    vec![
        "etc_hosts".to_string(),
        "etc_protocols".to_string(),
        "etc_services".to_string(),
        "system_info".to_string(),
        "os_version".to_string(),
        "logical_drives".to_string(),
        "interface_address".to_string(),
        "interface_details".to_string(),
        "uptime".to_string(),
        "products".to_string(),
        "wmi_computer_info".to_string(),
        "wmi_os_version".to_string(),
        "wmi_printers".to_string(),
        "wmi_services".to_string(),
        "wmi_hotfixes".to_string(),
        "wmi_shares".to_string(),
        "wmi_network_adapters".to_string(),
        "wmi_local_accounts".to_string(),
        "wmi_bios".to_string(),
        "wmi_motherboard".to_string(),
        "wmi_processor".to_string(),
        "wmi_physical_memory".to_string(),
        "wmi_sound".to_string(),
        "wmi_video".to_string(),
        "wmi_monitors".to_string(),
        "wmi_keyboard".to_string(),
        "wmi_pointing_device".to_string(),
        "process_open_sockets".to_string(),
        "processes".to_string(),
        "process_memory_map".to_string(),
        #[cfg(not(target_os = "windows"))]
        "process_envs".to_string(),
    ]
}

pub fn init_db()-> Connection {
    let mut db = Connection::open_in_memory().unwrap();
    load_module(&db).unwrap();
    db
}

pub fn register_first(db:  &Connection, first_table: String){
    if first_table.len() > 0 {
        let mut command = format!("{}{}{}{}{}",
                                  "CREATE VIRTUAL TABLE ",
                                  first_table,
                                  " USING siquery(table_name=",
                                  first_table, ")");
        &db.execute_batch(&command).unwrap();
    }
}

pub fn register_tables(db:  &Connection, tables: Vec<String>, first_table: String) {
    /*let version = version_number();

    if version < 3008012 {
        //let s: &str = &version.to_string();
        return Err(Error::ModuleError(format!("version: '{}' is not supported", version)));
    }*/
    for table in tables.iter() {
        if *table != first_table {
            let mut command = format!("{}{}{}{}{}",
                                      "CREATE VIRTUAL TABLE ",
                                      table,
                                      " USING siquery(table_name=",
                                      table, ")");
            &db.execute_batch(&command).unwrap();
        }
    }
}

pub fn get_form_query(query: &str) -> String {
    let mut _args = query.clone().to_uppercase();
    let mut v: Vec<_> = query.clone().split_whitespace().collect();
    let mut k: Vec<_> = _args.split_whitespace().collect();
    let mut table_name_idx = 0;
    let mut table_name: String = "".to_string();

    if _args.starts_with("SELECT") && _args.contains("FROM") {
        for i in 0..k.len() {
            if k[i] == "FROM" {
                table_name_idx = i + 1;
                table_name = v[table_name_idx].to_string();
            }
        }
    }

    table_name
}
