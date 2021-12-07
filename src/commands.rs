use rusqlite::{params, Connection, Result};

// used for initialized database
fn init() {
    let conn = Connection::open("power_db").expect("It worked?");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS device(
        id INTEGER PRIMARY KEY,
        name TEXT,
        uuid TEXT NOT NULL,
        datetime default current_timestamp
        ",
        [],
    ).expect("Could not create table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ride(
        id INTEGER PRIMARY KEY,
        name TEXT,
        datetime default current_timestamp
        ",
        [],
    ).expect("Could not create table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS frame(
        id INTEGER PRIMARY KEY,
        value TEXT,
        ride_id INTEGER NOT NULL,
        datetime default current_timestamp
        ",
        [],
    ).expect("Could not create table");
}

// find devices, list as name:uuid
fn find_devices() {}

// list saved devices name:uuid
fn list_devices() {}

// add device to saved device, either name or UUID
fn add_device(name_uuid: String) {}

// remove a device from saved devices either name or UUID
fn remove_device(name_uuid: String) {}

// begin recording a ride, optionally provide a name
fn start_ride(name: Option<String>) {}

// list rides
fn list_ride() {}

// export ride to JSON, name or id
fn export_ride(name_id: String) {}
