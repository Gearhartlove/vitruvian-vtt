use clap::{arg, command};
use rusqlite::{Connection, Result, Row};
use sea_query::{
    ColumnDef, Expr, Func, Iden, Order, Query, SqliteQueryBuilder,
    Table,
};
use std::path::Path;
use vitruvian_dbs::ingestion::{pf2e::Pf2eWorld, Schema};

fn main() -> Result<()> {
    let VERSION = "v001";

    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -n --name <NAME> "Set's a custom database name"
            )
            .default_value("vtt-db"),
        )
        .arg(
            arg!(
                -s --system <SYSTEM> "Roleplaying system the database is created for"
            )
            .default_value("pf2"),
        )
        .version(VERSION)
        .get_matches();

    let path = {
        let name = matches.get_one::<String>("name").unwrap();
        Path::new("sources")
            .join("dbs")
            .join(format!("{name}-{VERSION}.sqlite3"))
    };

    let pf2e_world = Pf2eWorld::new();
    println!("");

    let db = Connection::open(path).unwrap();

    //// Ancestry Table

    // Schema

    let sql = [
        Table::drop()
            .table(Ancestry::Table)
            .if_exists()
            .build(SqliteQueryBuilder),
        Table::drop()
            .table(Publication::Table)
            .if_exists()
            .build(SqliteQueryBuilder),
        Table::create()
            .table(Publication::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Publication::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Publication::Title).string().not_null())
            .col(ColumnDef::new(Publication::Licencse).string().not_null())
            .col(ColumnDef::new(Publication::Remaster).boolean().not_null())
            .build(SqliteQueryBuilder),
        Table::create()
            .table(Ancestry::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Ancestry::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            // strings
            .col(ColumnDef::new(Ancestry::Name).string().not_null())
            .col(ColumnDef::new(Ancestry::Description).string().not_null())
            // .col(ColumnDef::new(Ancestry::ImagePath).string().not_null())
            // .col(ColumnDef::new(Ancestry::Vision).string().not_null())
            // // ints
            // .col(ColumnDef::new(Ancestry::Hp).integer().not_null())
            // .col(ColumnDef::new(Ancestry::Reach).integer().not_null())
            // .col(ColumnDef::new(Ancestry::Size).integer().not_null())
            // .col(ColumnDef::new(Ancestry::Speed).integer().not_null())
            // // blobs
            // .col(ColumnDef::new(Ancestry::Languages).json().not_null())
            // .col(
            //     ColumnDef::new(Ancestry::AdditionalLanguages)
            //         .json()
            //         .not_null(),
            // )
            // .col(ColumnDef::new(Ancestry::Boosts).json().not_null())
            // .col(ColumnDef::new(Ancestry::Flaws).json().not_null())
            // .col(ColumnDef::new(Ancestry::Traits).json().not_null())
            // // foreign keys
            // .col(ColumnDef::new(Ancestry::Publication).integer().not_null())
            // .foreign_key(
            //     ForeignKey::create()
            //         .name("fk_ancestry_publication")
            //         .from(Ancestry::Table, Ancestry::Publication)
            //         .to(Publication::Table, Publication::Id)
            //         .on_delete(ForeignKeyAction::Cascade)
            //         .on_update(ForeignKeyAction::Cascade),
            // )
            .build(SqliteQueryBuilder),
    ]
    .join("; ");

    db.execute_batch(&sql)?;
    println!("Creates tables: publication, ancestry");
    println!();

    // Insert Publications

    // let statement = Query::insert()
    //     .into_table(Publication::Table)
    //     .columns([
    //         Publication::Licencse,
    //         Publication::Remaster,
    //         Publication::Title,
    //     ])
    //     .values_panic([
    //         "foobar-license".into(),
    //         false.into(), // remaster
    //         "foobar-title".into(),
    //     ])
    //     .to_owned()
    //     .to_string(SqliteQueryBuilder);

    // let result = db.execute(&statement, ());
    // println!("Insert into publication: {result:?}");
    // let id = db.last_insert_rowid();
    // println!("Last publication insert id: {id:?}");
    // println!();

    // Insert Ancestries

    // KGF: Cake
    let dwarf = pf2e_world
        .ancestry
        .iter()
        .find(|ancestry| ancestry.name == "Dwarf")
        .unwrap();

    let statement = Query::insert()
        .into_table(Ancestry::Table)
        .columns([
            Ancestry::Name,
            Ancestry::Description,
            // Ancestry::ImagePath,
            // Ancestry::Vision,
            // Ancestry::Hp,
            // Ancestry::Reach,
            // Ancestry::Size,
            // Ancestry::Speed,
            // Ancestry::Languages,
            // Ancestry::AdditionalLanguages,
            // Ancestry::Boosts,
            // Ancestry::Flaws,
            // Ancestry::Traits,
            // Ancestry::Publication,
        ])
        .values_panic(dwarf.values())
        .to_owned()
        .to_string(SqliteQueryBuilder);

    let result = db.execute(&statement, ());
    println!("Insert into ancestries: {result:?}");
    let id = db.last_insert_rowid();
    println!();

    // // Read

    let query = Query::select()
        .columns([
            Ancestry::Id,
            Ancestry::Name,
            // Ancestry::Description,
            // Ancestry::ImagePath,
            // Ancestry::Vision,
            // Ancestry::Hp,
            // Ancestry::Reach,
            // Ancestry::Size,
            // Ancestry::Speed,
            // Ancestry::Languages,
            // Ancestry::AdditionalLanguages,
            // Ancestry::Boosts,
            // Ancestry::Flaws,
            // Ancestry::Traits,
            // Ancestry::Publication,
        ])
        .from(Ancestry::Table)
        .to_owned()
        .order_by(Ancestry::Id, Order::Desc)
        .to_string(SqliteQueryBuilder);

    println!("read query: {}", query);

    println!("Select from ancestry");
    let mut stmt = db.prepare(query.as_str())?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let ancestry = AncestryStruct::from(row);
        println!("{ancestry:?}")
    }

    println!();

    // // Count

    let count = || -> Result<i64> {
        let query = Query::select()
            .from(Ancestry::Table)
            .expr(Func::count(Expr::col(Ancestry::Id)))
            .to_string(SqliteQueryBuilder);

        print!("Count Ancestry: ");
        let mut stmt = db.prepare(query.as_str())?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(row.get_unwrap(0))
        } else {
            Ok(0)
        }
    };

    println!("{}", count().unwrap());
    println!();

    // // Delete

    println!("Delete Ancestry: ");
    let statement = Query::delete()
        .from_table(Ancestry::Table)
        .and_where(Expr::col(Ancestry::Name).eq("Dwarf"))
        .to_string(SqliteQueryBuilder);

    println!("delete statement: {statement}");
    let mut stmt = db.prepare(statement.as_str())?;
    let _ = stmt.execute([])?;

    println!("{}", count().unwrap());
    println!();

    Ok(())
}

#[derive(Iden)]
enum Ancestry {
    Table,
    Id,
    Name,
    Description,
    // ImagePath,
    // Vision,
    // Size,
    // Hp,
    // Reach,
    // Speed,
    // Languages,
    // AdditionalLanguages,
    // Boosts,
    // Flaws,
    // Traits,
    // Publication, // fk
}

#[derive(Iden)]
enum Publication {
    Table,
    Id,
    Licencse,
    Remaster,
    Title,
}

#[derive(Debug, Default)]
struct AncestryStruct {
    id: i64,
    name: String,
    // description: String,
    // image_path: String,
    // size: String,
    // vision: String,
    // hp: i16,
    // reach: i8,
    // speed: i8,
    // languages: Languages,
    // additional_languages: AdditionalLanguages,

}

impl From<&Row<'_>> for AncestryStruct {
    fn from(row: &Row) -> Self {
        AncestryStruct {
            id: row.get_unwrap("id"),
            name: row.get_unwrap("name"),
        }
    }
}
