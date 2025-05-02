use core::f32;
mod hook;

// use pg_sys::FormData_pg_statistic;
// use pg_sys::SysCacheIdentifier::STATRELATTINH;
use crate::hook::rel_info;
use pgrx::pg_sys;
use pgrx::prelude::*;
// use pgrx::{direct_function_call, AnyArray, Array, Json};
use serde::{Deserialize, Serialize};

::pgrx::pg_module_magic!();

const STATISTIC_KIND_MCV: i16 = 1;
const STATISTIC_KIND_HISTOGRAM: i16 = 2;
const STATISTIC_KIND_CORRELATION: i16 = 3;
const STATISTIC_KIND_MCELEM: i16 = 4;
const STATISTIC_KIND_DECHIST: i16 = 5;
const STATISTIC_KIND_RANGE_LENGTH_HISTOGRAM: i16 = 6;
const STATISTIC_KIND_BOUNDS_HISTOGRAM: i16 = 7;

// Safe to ignore pg_class because of its lazy flags

#[derive(Debug, Serialize, Deserialize)]
struct JsonF32arr {
    data: String,
}

impl JsonF32arr {
    fn from(opt_arrf32: Option<Array<f32>>) -> Self {
        let data = match opt_arrf32 {
            None => "null".to_string(),
            Some(arrf32) => {
                let j = unsafe {
                    pgrx::direct_function_call::<pgrx::Json>(
                        pg_sys::array_to_json,
                        &[arrf32.into_datum()],
                    )
                    .unwrap()
                };
                serde_json::to_string(&j).unwrap()
            }
        };
        JsonF32arr { data }
    }

    unsafe fn set_pg_statistic_tuple(
        &self,
        values: &mut [pg_sys::Datum],
        nulls: &mut [bool],
        index: usize,
    ) {
        match self.data.as_str() {
            "null" => {
                nulls[index] = true;
                values[index] = pg_sys::Datum::from(0);
            }
            s => {
                let v_f32: Vec<f32> = serde_json::from_str(s).unwrap();
                // let numdatums = pg_sys::palloc(v_f32.len() * size_of::<pg_sys::Datum>()) as *mut pg_sys::Datum;
                // for (i, x) in v_f32.iter().enumerate() {
                //     *numdatums.add(i) = f32::into_datum(*x).unwrap();
                // }
                // let arry = pg_sys::construct_array(numdatums, v_f32.len() as i32, pg_sys::FLOAT4OID, size_of::<pg_sys::float4>() as i32, true, pg_sys::TYPALIGN_INT as i8);
                // let datum = pg_sys::Datum::from(arry);

                nulls[index] = false;
                values[index] = Vec::<f32>::into_datum(v_f32).unwrap();
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonAarr {
    typ: pg_sys::Oid,
    data: String,
}

impl JsonAarr {
    fn from(opt_aarr: (Option<pgrx::AnyArray>, Option<pg_sys::Oid>)) -> Self {
        match opt_aarr {
            (None, _) => JsonAarr {
                typ: pg_sys::ANYARRAYOID,
                data: "null".to_string(),
            },
            (Some(aarr), opt_oid) => {
                let j = unsafe {
                    pgrx::direct_function_call::<pgrx::Json>(
                        pg_sys::array_to_json,
                        &[aarr.into_datum()],
                    )
                    .unwrap()
                };
                JsonAarr {
                    typ: opt_oid.unwrap(),
                    data: serde_json::to_string(&j).unwrap(),
                }
            }
        }
    }

    unsafe fn set_pg_statistic_tuple(
        &self,
        values: &mut [pg_sys::Datum],
        nulls: &mut [bool],
        index: usize,
    ) {
        match self.data.as_str() {
            "null" => {
                nulls[index] = true;
                values[index] = pg_sys::Datum::from(0);
            }
            s => {
                let datum: pg_sys::Datum = match self.typ {
                    pg_sys::DATEOID => {
                        let v_date: Vec<pgrx::datum::Date> = serde_json::from_str(s).unwrap();
                        Vec::<pgrx::datum::Date>::into_datum(v_date).unwrap()
                    },
                    pg_sys::VARCHAROID => {
                        let v_char: Vec<String> = serde_json::from_str(s).unwrap();
                        Vec::<String>::into_datum(v_char).unwrap()
                    },
                    pg_sys::NUMERICOID => {
                        let v_f32: Vec<f32> = serde_json::from_str(s).unwrap();
                        Vec::<f32>::into_datum(v_f32).unwrap()
                    },
                    pg_sys::FLOAT4OID => {
                        let v_f32: Vec<f32> = serde_json::from_str(s).unwrap();
                        Vec::<f32>::into_datum(v_f32).unwrap()
                    }
                    pg_sys::FLOAT8OID => {
                        let v_f64: Vec<f64> = serde_json::from_str(s).unwrap();
                        Vec::<f64>::into_datum(v_f64).unwrap()
                    }
                    pg_sys::INT4OID => {
                        let v_i32: Vec<i32> = serde_json::from_str(s).unwrap();
                        Vec::<i32>::into_datum(v_i32).unwrap()
                    }
                    pg_sys::INT8OID => {
                        let v_i64: Vec<i64> = serde_json::from_str(s).unwrap();
                        Vec::<i64>::into_datum(v_i64).unwrap()
                    }
                    _ => panic!("Unsupported type for data type {}", s),
                };
                nulls[index] = false;
                values[index] = datum;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PgStatisticRow {
    starelid: pg_sys::Oid,
    staattnum: i16,
    stainherit: bool,
    stanullfrac: f32,
    stawidth: i32,
    stadistinct: f32,
    stakind1: i16,
    stakind2: i16,
    stakind3: i16,
    stakind4: i16,
    stakind5: i16,
    staop1: pg_sys::Oid,
    staop2: pg_sys::Oid,
    staop3: pg_sys::Oid,
    staop4: pg_sys::Oid,
    staop5: pg_sys::Oid,
    stacoll1: pg_sys::Oid,
    stacoll2: pg_sys::Oid,
    stacoll3: pg_sys::Oid,
    stacoll4: pg_sys::Oid,
    stacoll5: pg_sys::Oid,
    stanumbers1: JsonF32arr,
    stanumbers2: JsonF32arr,
    stanumbers3: JsonF32arr,
    stanumbers4: JsonF32arr,
    stanumbers5: JsonF32arr,
    stavalues1: JsonAarr,
    stavalues2: JsonAarr,
    stavalues3: JsonAarr,
    stavalues4: JsonAarr,
    stavalues5: JsonAarr,
}

#[derive(Debug, Serialize, Deserialize)]
struct PgClassRow {
    oid: pg_sys::Oid,
    relname: String,
    relnamespace: pg_sys::Oid,
    reltype: pg_sys::Oid,
    reloftype: pg_sys::Oid,
    relowner: pg_sys::Oid,
    relam: pg_sys::Oid,
    relfilenode: pg_sys::Oid,
    reltablespace: pg_sys::Oid,
    relpages: i32,
    reltuples: f32,
    relallvisible: i32,
    reltoastrelid: pg_sys::Oid,
    relhasindex: bool,
    relisshared: bool,
    relpersistence: i8,
    relkind: i8,
    relnatts: i16,
    relchecks: i16,
    relhasrules: bool,
    relhastriggers: bool,
    relhassubclass: bool,
    relrowsecurity: bool,
    relforcerowsecurity: bool,
    relispopulated: bool,
    relreplident: i8,
    relispartition: bool,
    relrewrite: pg_sys::Oid,
    relfrozenxid: i32,
    relminmxid: i32,
    // TODO: add remaining three types: relacl, reloptions, relpartbound
}

#[derive(Debug, Serialize, Deserialize)]
struct PgAttributeRow {
    attrelid: pg_sys::Oid,
    attname: String,
    atttypid: pg_sys::Oid,
    attstattarget: i32,
    attlen: i16,
    attnum: i16,
    attndims: i32,
    attcacheoff: i32,
    atttypmod: i32,
    attbyval: bool,
    attstorage: i8,
    attalign: i8,
    attnotnull: bool,
    atthasdef: bool,
    atthasmissing: bool,
    attidentity: i8,
    attgenerated: i8,
    attisdropped: bool,
    attislocal: bool,
    attinhcount: i32,
    attcollation: pg_sys::Oid,
    // TODO: add remaining types
}

unsafe fn aarr_elemtype(aarr: *mut pg_sys::AnyArrayType) -> Option<pg_sys::Oid> {
    if aarr.is_null() {
        None
    } else {
        // TODO(WAN): is there no pgrx wrapper for VARATT_IS_EXPANDED_HEADER?
        let varatt_is_expanded_header = (*aarr.cast::<pg_sys::varattrib_4b>()).va_4byte.va_header
            == (pg_sys::EOH_HEADER_MAGIC as u32);
        let oid = if varatt_is_expanded_header {
            (*aarr).xpn.element_type
        } else {
            (*aarr).flt.elemtype
        };
        Some(oid)
    }
}

unsafe fn pg_statistic_stavalues(
    tuple: *mut pg_sys::HeapTupleData,
    attnum: pg_sys::AttrNumber,
) -> (Option<pgrx::AnyArray>, Option<pg_sys::Oid>) {
    let mut is_null = false;
    let datum = pg_sys::SysCacheGetAttr(
        pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
        tuple,
        attnum,
        &mut is_null,
    );

    if is_null {
        (None, None)
    } else {
        let aarr: *mut pg_sys::AnyArrayType = pg_sys::DatumGetAnyArrayP(datum);
        let elemtype = aarr_elemtype(aarr).unwrap();
        (
            pgrx::AnyArray::from_polymorphic_datum(datum, is_null, elemtype),
            Some(elemtype),
        )
    }
}

// From ChatGPT
fn char_arr_to_string(arr: [i8; 64]) -> String {
    // Convert to Vec<u8>
    let bytes: Vec<u8> = arr.iter().map(|&b| b as u8).collect();

    // Trim at the first null byte (optional, if it's a C-style string)
    let trimmed = &bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())];

    // Convert to &str, then to String
    let s = std::str::from_utf8(trimmed)
        .expect("Invalid UTF-8")
        .to_string();
    s
}

#[pg_extern]
fn pg_attribute_dump(table_oid: i32, column_index: i32) -> Option<String> {
    unsafe {
        let pg_attribute =
            pg_sys::table_open(pg_sys::AttributeRelationId, pg_sys::RowExclusiveLock as i32);
        let tuple = pg_sys::SearchSysCache2(
            pg_sys::SysCacheIdentifier::ATTNUM as i32,
            table_oid.into(),
            column_index.into(),
        );
        let tuple_json = if tuple.is_null() {
            None
        } else {
            let stat_struct = pg_sys::GETSTRUCT(tuple) as *const pg_sys::FormData_pg_attribute;

            let attrelid: pg_sys::Oid = (*stat_struct).attrelid;
            let attname: String = char_arr_to_string((*stat_struct).attname.data);
            let atttypid: pg_sys::Oid = (*stat_struct).atttypid;
            let attstattarget: i32 = (*stat_struct).attstattarget;
            let attlen: i16 = (*stat_struct).attlen;
            let attnum: i16 = (*stat_struct).attnum;
            let attcacheoff: i32 = (*stat_struct).attcacheoff;
            let attndims: i32 = (*stat_struct).attndims;
            let atttypmod: i32 = (*stat_struct).atttypmod;
            let attbyval: bool = (*stat_struct).attbyval;
            let attstorage: i8 = (*stat_struct).attstorage;
            let attalign: i8 = (*stat_struct).attalign;
            let attnotnull: bool = (*stat_struct).attnotnull;
            let atthasdef: bool = (*stat_struct).atthasdef;
            let atthasmissing: bool = (*stat_struct).atthasmissing;
            let attidentity: i8 = (*stat_struct).attidentity;
            let attgenerated: i8 = (*stat_struct).attgenerated;
            let attisdropped: bool = (*stat_struct).attisdropped;
            let attislocal: bool = (*stat_struct).attislocal;
            let attinhcount: i32 = (*stat_struct).attinhcount;
            let attcollation: pg_sys::Oid = (*stat_struct).attcollation;

            let pg_row = PgAttributeRow {
                attrelid,
                attname,
                atttypid,
                attstattarget,
                attlen,
                attnum,
                attndims,
                attcacheoff,
                atttypmod,
                attbyval,
                attstorage,
                attalign,
                attnotnull,
                atthasdef,
                atthasmissing,
                attidentity,
                attgenerated,
                attisdropped,
                attislocal,
                attinhcount,
                attcollation,
            };

            let json_str = serde_json::to_string(&pg_row).unwrap();
            pg_sys::ReleaseSysCache(tuple);
            Some(json_str)
        };
        pg_sys::table_close(pg_attribute, pg_sys::RowExclusiveLock as i32);
        tuple_json
    }
}

#[pg_extern]
fn pg_attribute_load(data: String) -> bool {
    let pg_row: PgAttributeRow = serde_json::from_str(&data).unwrap();
    let table_oid: u32 = pg_row.attrelid.as_u32();
    let column_index: i16 = pg_row.attnum;

    let mut values: Vec<pg_sys::Datum> = Vec::with_capacity(pg_sys::Natts_pg_attribute as usize);
    let mut nulls: Vec<bool> = Vec::with_capacity(pg_sys::Natts_pg_attribute as usize);
    let mut replaces: Vec<bool> = Vec::with_capacity(pg_sys::Natts_pg_attribute as usize);

    for _ in 0..pg_sys::Natts_pg_attribute - 4 {
        values.push(pg_sys::Datum::from(0));
        nulls.push(false);
        replaces.push(true);
    }

    for _ in 0..4 {
        values.push(pg_sys::Datum::from(0));
        nulls.push(true);
        replaces.push(true);
    }

    // analyze.c performs a slightly cursed blend of Anum and i++ based accessing.
    values[pg_sys::Anum_pg_attribute_attrelid as usize - 1] = pg_row.attrelid.into_datum().unwrap();
    unsafe {
        let relname_datum: pg_sys::Datum = string_to_namedata_datum(pg_row.attname);
        values[pg_sys::Anum_pg_attribute_attname as usize - 1] = relname_datum;
    }
    values[pg_sys::Anum_pg_attribute_atttypid as usize - 1] = pg_row
        .atttypid
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_attribute_attstattarget as usize - 1] =
        pg_row.attstattarget.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attlen as usize - 1] = pg_row.attlen.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attnum as usize - 1] = pg_row.attnum.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attndims as usize - 1] = pg_row.attndims.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attcacheoff as usize - 1] =
        pg_row.attcacheoff.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_atttypmod as usize - 1] =
        pg_row.atttypmod.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attbyval as usize - 1] = pg_row.attbyval.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attstorage as usize - 1] =
        pg_row.attstorage.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attalign as usize - 1] = pg_row.attalign.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attnotnull as usize - 1] =
        pg_row.attnotnull.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_atthasdef as usize - 1] =
        pg_row.atthasdef.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_atthasmissing as usize - 1] =
        pg_row.atthasmissing.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attidentity as usize - 1] =
        pg_row.attidentity.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attgenerated as usize - 1] =
        pg_row.attgenerated.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attisdropped as usize - 1] =
        pg_row.attisdropped.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attislocal as usize - 1] =
        pg_row.attislocal.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attinhcount as usize - 1] =
        pg_row.attinhcount.into_datum().unwrap();
    values[pg_sys::Anum_pg_attribute_attcollation as usize - 1] = pg_row
        .attcollation
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));

    unsafe {
        let pg_attribute =
            pg_sys::table_open(pg_sys::AttributeRelationId, pg_sys::RowExclusiveLock as i32);
        let indstate = pg_sys::CatalogOpenIndexes(pg_attribute);
        let pg_attribute_tuple_desc = (*pg_attribute).rd_att;

        let oldtup = pg_sys::SearchSysCache2(
            pg_sys::SysCacheIdentifier::ATTNUM as i32,
            table_oid.into(),
            column_index.into(),
        );

        if !oldtup.is_null() {
            return true;
        } else {
            let stup = pg_sys::heap_form_tuple(
                pg_attribute_tuple_desc,
                values.as_mut_ptr(),
                nulls.as_mut_ptr(),
            );
            pg_sys::CatalogTupleInsertWithInfo(pg_attribute, stup, indstate);
            pg_sys::heap_freetuple(stup);
        }

        if !indstate.is_null() {
            pg_sys::CatalogCloseIndexes(indstate);
        }

        pg_sys::table_close(pg_attribute, pg_sys::RowExclusiveLock as i32);
    }

    true
}

#[pg_extern]
fn pg_class_dump(table_oid: i32) -> Option<String> {
    unsafe {
        let pg_class =
            pg_sys::table_open(pg_sys::RelationRelationId, pg_sys::RowExclusiveLock as i32);
        let tuple =
            pg_sys::SearchSysCache1(pg_sys::SysCacheIdentifier::RELOID as i32, table_oid.into());
        let tuple_json = if tuple.is_null() {
            None
        } else {
            let stat_struct = pg_sys::GETSTRUCT(tuple) as *mut pg_sys::FormData_pg_class;

            let oid: pg_sys::Oid = (*stat_struct).oid;
            let relname: String = char_arr_to_string((*stat_struct).relname.data);
            let relnamespace: pg_sys::Oid = (*stat_struct).relnamespace;
            let reltype: pg_sys::Oid = (*stat_struct).reltype;
            let reloftype: pg_sys::Oid = (*stat_struct).reloftype;
            let relowner: pg_sys::Oid = (*stat_struct).relowner;
            let relam: pg_sys::Oid = (*stat_struct).relam;
            let relfilenode: pg_sys::Oid = (*stat_struct).relfilenode;
            let reltablespace: pg_sys::Oid = (*stat_struct).reltablespace;
            let relpages: i32 = (*stat_struct).relpages;
            let reltuples: f32 = (*stat_struct).reltuples;
            let relallvisible: i32 = (*stat_struct).relallvisible;
            let reltoastrelid: pg_sys::Oid = (*stat_struct).reltoastrelid;
            let relhasindex: bool = (*stat_struct).relhasindex;
            let relisshared: bool = (*stat_struct).relisshared;
            let relpersistence: i8 = (*stat_struct).relpersistence;
            let relkind: i8 = (*stat_struct).relkind;
            let relnatts: i16 = (*stat_struct).relnatts;
            let relchecks: i16 = (*stat_struct).relchecks;
            let relhasrules: bool = (*stat_struct).relhasrules;
            let relhastriggers: bool = (*stat_struct).relhastriggers;
            let relhassubclass: bool = (*stat_struct).relhassubclass;
            let relrowsecurity: bool = (*stat_struct).relrowsecurity;
            let relforcerowsecurity: bool = (*stat_struct).relforcerowsecurity;
            let relispopulated: bool = (*stat_struct).relispopulated;
            let relreplident: i8 = (*stat_struct).relreplident;
            let relispartition: bool = (*stat_struct).relispartition;
            let relrewrite: pg_sys::Oid = (*stat_struct).relrewrite;
            let relfrozenxid: i32 = (*stat_struct).relfrozenxid as i32;
            let relminmxid: i32 = (*stat_struct).relminmxid as i32;

            let pg_row = PgClassRow {
                oid,
                relname,
                relnamespace,
                reltype,
                reloftype,
                relowner,
                relam,
                relfilenode,
                reltablespace,
                relpages,
                reltuples,
                relallvisible,
                reltoastrelid,
                relhasindex,
                relisshared,
                relpersistence,
                relkind,
                relnatts,
                relchecks,
                relhasrules,
                relhastriggers,
                relhassubclass,
                relrowsecurity,
                relforcerowsecurity,
                relispopulated,
                relreplident,
                relispartition,
                relrewrite,
                relfrozenxid,
                relminmxid,
            };

            let json_str = serde_json::to_string(&pg_row).unwrap();
            pg_sys::ReleaseSysCache(tuple);
            Some(json_str)
        };
        pg_sys::table_close(pg_class, pg_sys::RowExclusiveLock as i32);
        tuple_json
    }
}

// From ChatGPT
unsafe fn string_to_namedata_datum(s: String) -> pg_sys::Datum {
    let nd: *mut pg_sys::nameData = pg_sys::palloc(64) as *mut pg_sys::NameData;
    let bytes = s.as_bytes();
    // Truncate if input is too long
    let len = bytes.len().min(63);
    (*nd).data = [0; 64];
    std::ptr::copy_nonoverlapping(s.as_ptr(), (*nd).data.as_mut_ptr() as *mut u8, len);
    std::mem::transmute::<*const pg_sys::nameData, pg_sys::Datum>(nd)
}

#[pg_extern]
fn pg_test(data: String) -> Vec<i8> {
    let pg_row: PgClassRow = serde_json::from_str(&data).unwrap();
    let mut name_arr: [i8; 64] = [0; 64];
    let bytes = pg_row.relname.as_bytes();
    // Truncate if input is too long
    let len = bytes.len().min(63); // leave space for null terminator if needed
    for i in 0..len {
        name_arr[i] = bytes[i] as i8;
    }
    let nd: pg_sys::nameData = pg_sys::nameData { data: name_arr };
    (nd).data.map(i8::from).to_vec()
}

#[pg_extern]
fn pg_class_load(data: String) -> bool {
    let pg_row: PgClassRow = serde_json::from_str(&data).unwrap();
    let table_oid: u32 = pg_row.oid.as_u32();

    let mut values: Vec<pg_sys::Datum> = Vec::with_capacity(pg_sys::Natts_pg_class as usize);
    let mut nulls: Vec<bool> = Vec::with_capacity(pg_sys::Natts_pg_class as usize);
    let mut replaces: Vec<bool> = Vec::with_capacity(pg_sys::Natts_pg_class as usize);

    for _ in 0..pg_sys::Natts_pg_class - 3 {
        values.push(pg_sys::Datum::from(0));
        nulls.push(false);
        replaces.push(true);
    }

    for _ in 0..3 {
        values.push(pg_sys::Datum::from(0));
        nulls.push(true);
        replaces.push(true);
    }

    // analyze.c performs a slightly cursed blend of Anum and i++ based accessing.
    values[pg_sys::Anum_pg_class_oid as usize - 1] = pg_row.oid.into_datum().unwrap();
    unsafe {
        let relname_datum: pg_sys::Datum = string_to_namedata_datum(pg_row.relname);
        values[pg_sys::Anum_pg_class_relname as usize - 1] = relname_datum;
    }
    values[pg_sys::Anum_pg_class_relnamespace as usize - 1] = pg_row
        .relnamespace
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_reltype as usize - 1] = pg_row
        .reltype
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_reloftype as usize - 1] = pg_row
        .reloftype
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_relowner as usize - 1] = pg_row
        .relowner
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_relam as usize - 1] =
        pg_row.relam.into_datum().unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_relfilenode as usize - 1] = pg_row
        .relfilenode
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_reltablespace as usize - 1] = pg_row
        .reltablespace
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_relpages as usize - 1] = pg_row.relpages.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_reltuples as usize - 1] = pg_row.reltuples.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relallvisible as usize - 1] =
        pg_row.relallvisible.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_reltoastrelid as usize - 1] = pg_row
        .reltoastrelid
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_relhasindex as usize - 1] =
        pg_row.relhasindex.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relisshared as usize - 1] =
        pg_row.relisshared.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relpersistence as usize - 1] =
        pg_row.relpersistence.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relkind as usize - 1] = pg_row.relkind.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relnatts as usize - 1] = pg_row.relnatts.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relchecks as usize - 1] = pg_row.relchecks.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relhasrules as usize - 1] =
        pg_row.relhasrules.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relhastriggers as usize - 1] =
        pg_row.relhastriggers.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relhassubclass as usize - 1] =
        pg_row.relhassubclass.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relrowsecurity as usize - 1] =
        pg_row.relrowsecurity.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relforcerowsecurity as usize - 1] =
        pg_row.relforcerowsecurity.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relispopulated as usize - 1] =
        pg_row.relispopulated.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relreplident as usize - 1] =
        pg_row.relreplident.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relispartition as usize - 1] =
        pg_row.relispartition.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relrewrite as usize - 1] = pg_row
        .relrewrite
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_class_relfrozenxid as usize - 1] =
        pg_row.relfrozenxid.into_datum().unwrap();
    values[pg_sys::Anum_pg_class_relminmxid as usize - 1] = pg_row.relminmxid.into_datum().unwrap();

    unsafe {
        let pg_class =
            pg_sys::table_open(pg_sys::RelationRelationId, pg_sys::RowExclusiveLock as i32);
        let indstate = pg_sys::CatalogOpenIndexes(pg_class);
        let pg_class_tuple_desc = (*pg_class).rd_att;

        let oldtup =
            pg_sys::SearchSysCache1(pg_sys::SysCacheIdentifier::RELOID as i32, table_oid.into());

        if !oldtup.is_null() {
            let stup = pg_sys::heap_modify_tuple(
                oldtup,
                pg_class_tuple_desc,
                values.as_mut_ptr(),
                nulls.as_mut_ptr(),
                replaces.as_mut_ptr(),
            );
            pg_sys::ReleaseSysCache(oldtup);
            pg_sys::CatalogTupleUpdateWithInfo(pg_class, &mut (*stup).t_self, stup, indstate);
            rel_info.lock().unwrap().insert(
                pg_row.oid,
                (
                    pg_row.relpages as pg_sys::BlockNumber,
                    pg_row.reltuples as f64,
                ),
            );
            pg_sys::heap_freetuple(stup);
        } else {
            let stup = pg_sys::heap_form_tuple(
                pg_class_tuple_desc,
                values.as_mut_ptr(),
                nulls.as_mut_ptr(),
            );
            pg_sys::CatalogTupleInsertWithInfo(pg_class, stup, indstate);

            // Also insert into rel_info for new tuple insertion
            rel_info.lock().unwrap().insert(
                pg_row.oid,
                (
                    pg_row.relpages as pg_sys::BlockNumber,
                    pg_row.reltuples as f64,
                ),
            );
            pg_sys::heap_freetuple(stup);
        }

        if !indstate.is_null() {
            pg_sys::CatalogCloseIndexes(indstate);
        }

        pg_sys::table_close(pg_class, pg_sys::RowExclusiveLock as i32);
    }

    true
}

#[pg_extern]
fn pg_statistic_dump(starelid: i32, staattnum: i16) -> Option<String> {
    unsafe {
        let pg_statistic =
            pg_sys::table_open(pg_sys::StatisticRelationId, pg_sys::RowExclusiveLock as i32);
        let tuple = pg_sys::SearchSysCache3(
            pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
            starelid.into(),
            staattnum.into(),
            false.into(),
        );
        let tuple_json = if tuple.is_null() {
            None
        } else {
            let stat_struct = pg_sys::GETSTRUCT(tuple) as *mut pg_sys::FormData_pg_statistic;

            let starelid: pg_sys::Oid = (*stat_struct).starelid;
            let staattnum: i16 = (*stat_struct).staattnum;
            let stainherit: bool = (*stat_struct).stainherit;
            let stanullfrac: f32 = (*stat_struct).stanullfrac;
            let stawidth: i32 = (*stat_struct).stawidth;
            let stadistinct: f32 = (*stat_struct).stadistinct;
            let stakind1: i16 = (*stat_struct).stakind1;
            let stakind2: i16 = (*stat_struct).stakind2;
            let stakind3: i16 = (*stat_struct).stakind3;
            let stakind4: i16 = (*stat_struct).stakind4;
            let stakind5: i16 = (*stat_struct).stakind5;
            let staop1: pg_sys::Oid = (*stat_struct).staop1;
            let staop2: pg_sys::Oid = (*stat_struct).staop2;
            let staop3: pg_sys::Oid = (*stat_struct).staop3;
            let staop4: pg_sys::Oid = (*stat_struct).staop4;
            let staop5: pg_sys::Oid = (*stat_struct).staop5;
            let stacoll1: pg_sys::Oid = (*stat_struct).stacoll1;
            let stacoll2: pg_sys::Oid = (*stat_struct).stacoll2;
            let stacoll3: pg_sys::Oid = (*stat_struct).stacoll3;
            let stacoll4: pg_sys::Oid = (*stat_struct).stacoll4;
            let stacoll5: pg_sys::Oid = (*stat_struct).stacoll5;
            let mut is_null = false;
            let stanumbers1_opt_arrf32: Option<pgrx::Array<f32>> =
                pgrx::Array::from_polymorphic_datum(
                    pg_sys::SysCacheGetAttr(
                        pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
                        tuple,
                        pg_sys::Anum_pg_statistic_stanumbers1 as pg_sys::AttrNumber,
                        &mut is_null,
                    ),
                    is_null,
                    pg_sys::FLOAT4OID,
                );
            let stanumbers2_opt_arrf32: Option<pgrx::Array<f32>> =
                pgrx::Array::from_polymorphic_datum(
                    pg_sys::SysCacheGetAttr(
                        pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
                        tuple,
                        pg_sys::Anum_pg_statistic_stanumbers2 as pg_sys::AttrNumber,
                        &mut is_null,
                    ),
                    is_null,
                    pg_sys::FLOAT4OID,
                );
            let stanumbers3_opt_arrf32: Option<pgrx::Array<f32>> =
                pgrx::Array::from_polymorphic_datum(
                    pg_sys::SysCacheGetAttr(
                        pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
                        tuple,
                        pg_sys::Anum_pg_statistic_stanumbers3 as pg_sys::AttrNumber,
                        &mut is_null,
                    ),
                    is_null,
                    pg_sys::FLOAT4OID,
                );
            let stanumbers4_opt_arrf32: Option<pgrx::Array<f32>> =
                pgrx::Array::from_polymorphic_datum(
                    pg_sys::SysCacheGetAttr(
                        pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
                        tuple,
                        pg_sys::Anum_pg_statistic_stanumbers4 as pg_sys::AttrNumber,
                        &mut is_null,
                    ),
                    is_null,
                    pg_sys::FLOAT4OID,
                );
            let stanumbers5_opt_arrf32: Option<pgrx::Array<f32>> =
                pgrx::Array::from_polymorphic_datum(
                    pg_sys::SysCacheGetAttr(
                        pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
                        tuple,
                        pg_sys::Anum_pg_statistic_stanumbers5 as pg_sys::AttrNumber,
                        &mut is_null,
                    ),
                    is_null,
                    pg_sys::FLOAT4OID,
                );
            let stavalues1_opt_aarr: (Option<pgrx::AnyArray>, Option<pg_sys::Oid>) =
                pg_statistic_stavalues(
                    tuple,
                    pg_sys::Anum_pg_statistic_stavalues1 as pg_sys::AttrNumber,
                );
            let stavalues2_opt_aarr: (Option<pgrx::AnyArray>, Option<pg_sys::Oid>) =
                pg_statistic_stavalues(
                    tuple,
                    pg_sys::Anum_pg_statistic_stavalues2 as pg_sys::AttrNumber,
                );
            let stavalues3_opt_aarr: (Option<pgrx::AnyArray>, Option<pg_sys::Oid>) =
                pg_statistic_stavalues(
                    tuple,
                    pg_sys::Anum_pg_statistic_stavalues3 as pg_sys::AttrNumber,
                );
            let stavalues4_opt_aarr: (Option<pgrx::AnyArray>, Option<pg_sys::Oid>) =
                pg_statistic_stavalues(
                    tuple,
                    pg_sys::Anum_pg_statistic_stavalues4 as pg_sys::AttrNumber,
                );
            let stavalues5_opt_aarr: (Option<pgrx::AnyArray>, Option<pg_sys::Oid>) =
                pg_statistic_stavalues(
                    tuple,
                    pg_sys::Anum_pg_statistic_stavalues5 as pg_sys::AttrNumber,
                );

            let stanumbers1 = JsonF32arr::from(stanumbers1_opt_arrf32);
            let stanumbers2 = JsonF32arr::from(stanumbers2_opt_arrf32);
            let stanumbers3 = JsonF32arr::from(stanumbers3_opt_arrf32);
            let stanumbers4 = JsonF32arr::from(stanumbers4_opt_arrf32);
            let stanumbers5 = JsonF32arr::from(stanumbers5_opt_arrf32);
            let stavalues1 = JsonAarr::from(stavalues1_opt_aarr);
            let stavalues2 = JsonAarr::from(stavalues2_opt_aarr);
            let stavalues3 = JsonAarr::from(stavalues3_opt_aarr);
            let stavalues4 = JsonAarr::from(stavalues4_opt_aarr);
            let stavalues5 = JsonAarr::from(stavalues5_opt_aarr);

            let pg_row = PgStatisticRow {
                starelid,
                staattnum,
                stainherit,
                stanullfrac,
                stawidth,
                stadistinct,
                stakind1,
                stakind2,
                stakind3,
                stakind4,
                stakind5,
                staop1,
                staop2,
                staop3,
                staop4,
                staop5,
                stacoll1,
                stacoll2,
                stacoll3,
                stacoll4,
                stacoll5,
                stanumbers1,
                stanumbers2,
                stanumbers3,
                stanumbers4,
                stanumbers5,
                stavalues1,
                stavalues2,
                stavalues3,
                stavalues4,
                stavalues5,
            };

            let json_str = serde_json::to_string(&pg_row).unwrap();
            pg_sys::ReleaseSysCache(tuple);
            Some(json_str)
        };
        pg_sys::table_close(pg_statistic, pg_sys::RowExclusiveLock as i32);
        tuple_json
    }
}

#[pg_extern]
fn pg_statistic_load(data: String) -> bool {
    let pg_row: PgStatisticRow = serde_json::from_str(&data).unwrap();
    let starelid = pg_row.starelid;
    let staattnum = pg_row.staattnum;

    let mut values: Vec<pg_sys::Datum> = Vec::with_capacity(pg_sys::Natts_pg_statistic as usize);
    let mut nulls: Vec<bool> = Vec::with_capacity(pg_sys::Natts_pg_statistic as usize);
    let mut replaces: Vec<bool> = Vec::with_capacity(pg_sys::Natts_pg_statistic as usize);

    for _ in 0..pg_sys::Natts_pg_statistic {
        values.push(pg_sys::Datum::from(0));
        nulls.push(false);
        replaces.push(true);
    }

    // analyze.c performs a slightly cursed blend of Anum and i++ based accessing.
    values[pg_sys::Anum_pg_statistic_starelid as usize - 1] = pg_row.starelid.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_staattnum as usize - 1] =
        pg_row.staattnum.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stainherit as usize - 1] =
        pg_row.stainherit.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stanullfrac as usize - 1] =
        pg_row.stanullfrac.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stawidth as usize - 1] = pg_row.stawidth.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stadistinct as usize - 1] =
        pg_row.stadistinct.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stakind1 as usize - 1] = pg_row.stakind1.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stakind2 as usize - 1] = pg_row.stakind2.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stakind3 as usize - 1] = pg_row.stakind3.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stakind4 as usize - 1] = pg_row.stakind4.into_datum().unwrap();
    values[pg_sys::Anum_pg_statistic_stakind5 as usize - 1] = pg_row.stakind5.into_datum().unwrap();
    // pg_sys::Datum::from(0) is a hack around Oid(0) deserializing to None.
    values[pg_sys::Anum_pg_statistic_staop1 as usize - 1] =
        pg_row.staop1.into_datum().unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_staop2 as usize - 1] =
        pg_row.staop2.into_datum().unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_staop3 as usize - 1] =
        pg_row.staop3.into_datum().unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_staop4 as usize - 1] =
        pg_row.staop4.into_datum().unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_staop5 as usize - 1] =
        pg_row.staop5.into_datum().unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_stacoll1 as usize - 1] = pg_row
        .stacoll1
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_stacoll2 as usize - 1] = pg_row
        .stacoll2
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_stacoll3 as usize - 1] = pg_row
        .stacoll3
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_stacoll4 as usize - 1] = pg_row
        .stacoll4
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));
    values[pg_sys::Anum_pg_statistic_stacoll5 as usize - 1] = pg_row
        .stacoll5
        .into_datum()
        .unwrap_or(pg_sys::Datum::from(0));

    unsafe {
        pg_row.stanumbers1.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stanumbers1 as usize - 1,
        );
        pg_row.stanumbers2.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stanumbers2 as usize - 1,
        );
        pg_row.stanumbers3.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stanumbers3 as usize - 1,
        );
        pg_row.stanumbers4.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stanumbers4 as usize - 1,
        );
        pg_row.stanumbers5.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stanumbers5 as usize - 1,
        );
        pg_row.stavalues1.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stavalues1 as usize - 1,
        );
        pg_row.stavalues2.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stavalues2 as usize - 1,
        );
        pg_row.stavalues3.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stavalues3 as usize - 1,
        );
        pg_row.stavalues4.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stavalues4 as usize - 1,
        );
        pg_row.stavalues5.set_pg_statistic_tuple(
            &mut values,
            &mut nulls,
            pg_sys::Anum_pg_statistic_stavalues5 as usize - 1,
        );
    }

    unsafe {
        let pg_statistic =
            pg_sys::table_open(pg_sys::StatisticRelationId, pg_sys::RowExclusiveLock as i32);
        let indstate = pg_sys::CatalogOpenIndexes(pg_statistic);
        let pg_statistic_tuple_desc = (*pg_statistic).rd_att;

        let oldtup = pg_sys::SearchSysCache3(
            pg_sys::SysCacheIdentifier::STATRELATTINH as i32,
            starelid.into(),
            staattnum.into(),
            false.into(),
        );

        if !oldtup.is_null() {
            let stup = pg_sys::heap_modify_tuple(
                oldtup,
                pg_statistic_tuple_desc,
                values.as_mut_ptr(),
                nulls.as_mut_ptr(),
                replaces.as_mut_ptr(),
            );
            pg_sys::ReleaseSysCache(oldtup);
            pg_sys::CatalogTupleUpdateWithInfo(pg_statistic, &mut (*stup).t_self, stup, indstate);
            pg_sys::heap_freetuple(stup);
        } else {
            let stup = pg_sys::heap_form_tuple(
                pg_statistic_tuple_desc,
                values.as_mut_ptr(),
                nulls.as_mut_ptr(),
            );
            pg_sys::CatalogTupleInsertWithInfo(pg_statistic, stup, indstate);
            pg_sys::heap_freetuple(stup);
        }

        if !indstate.is_null() {
            pg_sys::CatalogCloseIndexes(indstate);
        }

        pg_sys::table_close(pg_statistic, pg_sys::RowExclusiveLock as i32);
    }

    true
}

// Verifies that stavalues have the correct (original) type and has same length as
// new stanum
fn verify_stavalues(typ: pg_sys::Oid, stavalues_as_string: &str, stanum_len: usize) {
    match typ {
        pg_sys::FLOAT4OID => {
            let vec: Vec<f32> = serde_json::from_str(stavalues_as_string).unwrap();
            if vec.len() != stanum_len {
                panic!("stanum and stavalues have different lengths");
            }
        }
        pg_sys::FLOAT8OID => {
            let vec: Vec<f64> = serde_json::from_str(stavalues_as_string).unwrap();
            if vec.len() != stanum_len {
                panic!("stanum and stavalues have different lengths");
            }
        }
        pg_sys::INT4OID => {
            let vec: Vec<i32> = serde_json::from_str(stavalues_as_string).unwrap();
            if vec.len() != stanum_len {
                panic!("stanum and stavalues have different lengths");
            }
        }
        pg_sys::INT8OID => {
            let vec: Vec<i64> = serde_json::from_str(stavalues_as_string).unwrap();
            if vec.len() != stanum_len {
                panic!("stanum and stavalues have different lengths");
            }
        }
        _ => panic!("Unsupported type"),
    }
}

fn verify_histogram(typ: pg_sys::Oid, stavalues_as_string: &str) {
    match typ {
        pg_sys::FLOAT4OID => {
            let vec: Vec<f32> = serde_json::from_str(stavalues_as_string).unwrap();
            let mut prev_elem = vec[0];
            for item in vec.iter().skip(1) {
                // Any way to clean this up?
                if *item <= prev_elem {
                    panic!("Histogram bounds must be strictly increasing");
                }
                prev_elem = *item;
            }
        }
        pg_sys::FLOAT8OID => {
            let vec: Vec<f64> = serde_json::from_str(stavalues_as_string).unwrap();
            let mut prev_elem = vec[0];
            for item in vec.iter().skip(1) {
                if *item <= prev_elem {
                    panic!("Histogram bounds must be strictly increasing");
                }
                prev_elem = *item;
            }
        }
        pg_sys::INT4OID => {
            let vec: Vec<i32> = serde_json::from_str(stavalues_as_string).unwrap();
            let mut prev_elem = vec[0];
            for item in vec.iter().skip(1) {
                if *item <= prev_elem {
                    panic!("Histogram bounds must be strictly increasing");
                }
                prev_elem = *item;
            }
        }
        pg_sys::INT8OID => {
            let vec: Vec<i64> = serde_json::from_str(stavalues_as_string).unwrap();
            let mut prev_elem = vec[0];
            for item in vec.iter().skip(1) {
                if *item <= prev_elem {
                    panic!("Histogram bounds must be strictly increasing");
                }
                prev_elem = *item;
            }
        }
        _ => panic!("Unsupported type"),
    }
}

#[pg_extern]
fn pg_statistic_modify(
    json_dump: String,
    statistic_type: String,
    new_value: String,
    new_stavalues: String,
    new_stanums: String,
) -> String {
    // Parse json
    let mut pg_row: PgStatisticRow = serde_json::from_str(&json_dump).unwrap();
    match statistic_type.as_str() {
        "NULLFRAC" => {
            let new_frac = new_value.parse::<f32>().unwrap();
            pg_row.stanullfrac = new_frac;
        }
        "WIDTH" => {
            let new_width = new_value.parse::<i32>().unwrap();
            pg_row.stawidth = new_width;
        }
        "DISTINCT" => {
            let new_distinct = new_value.parse::<f32>().unwrap();
            pg_row.stadistinct = new_distinct;
        }
        "MOST COMMON VALUES" => {
            let stanumbers: Vec<f32> = serde_json::from_str(&new_stanums).unwrap();
            let stanum_len = stanumbers.len();
            // Verify that stanums are all floats and that frequencies are decreasing
            let total_freq = stanumbers.iter().copied().reduce(|a, b| a + b);
            if total_freq.is_none() || total_freq.unwrap() > 1.0 {
                let s = format!(
                    "Sum of frequencies must be less than or equal to 1, was {:?}",
                    total_freq
                );
                panic!("{}", s);
            }
            let mut prev_freq = f32::INFINITY;
            for freq in stanumbers {
                if freq > prev_freq {
                    panic!("Frequencies must be decreasing");
                }
                if freq <= 0.0 {
                    panic!("Frequencies must be positive");
                }
                prev_freq = freq;
            }
            // Verify correlation is one of the available statistics
            if pg_row.stakind1 == STATISTIC_KIND_MCV {
                pg_row.stanumbers1 = JsonF32arr { data: new_stanums };
                verify_stavalues(pg_row.stavalues1.typ, &new_stavalues, stanum_len);
                pg_row.stavalues1.data = new_stavalues;
            } else if pg_row.stakind2 == STATISTIC_KIND_MCV {
                pg_row.stanumbers2 = JsonF32arr { data: new_stanums };
                verify_stavalues(pg_row.stavalues2.typ, &new_stavalues, stanum_len);
                pg_row.stavalues2.data = new_stavalues;
            } else if pg_row.stakind3 == STATISTIC_KIND_MCV {
                pg_row.stanumbers3 = JsonF32arr { data: new_stanums };
                verify_stavalues(pg_row.stavalues3.typ, &new_stavalues, stanum_len);
                pg_row.stavalues3.data = new_stavalues;
            } else if pg_row.stakind4 == STATISTIC_KIND_MCV {
                pg_row.stanumbers4 = JsonF32arr { data: new_stanums };
                verify_stavalues(pg_row.stavalues4.typ, &new_stavalues, stanum_len);
                pg_row.stavalues4.data = new_stavalues;
            } else if pg_row.stakind5 == STATISTIC_KIND_MCV {
                pg_row.stanumbers5 = JsonF32arr { data: new_stanums };
                verify_stavalues(pg_row.stavalues5.typ, &new_stavalues, stanum_len);
                pg_row.stavalues5.data = new_stavalues;
            } else {
                panic!("MCV is not a statistic for this column");
            }
        }
        "HISTOGRAM" => {
            if pg_row.stakind1 == STATISTIC_KIND_HISTOGRAM {
                verify_histogram(pg_row.stavalues1.typ, &new_stavalues);
                pg_row.stavalues1.data = new_stavalues;
            } else if pg_row.stakind2 == STATISTIC_KIND_HISTOGRAM {
                verify_histogram(pg_row.stavalues2.typ, &new_stavalues);
                pg_row.stavalues2.data = new_stavalues;
            } else if pg_row.stakind3 == STATISTIC_KIND_HISTOGRAM {
                verify_histogram(pg_row.stavalues3.typ, &new_stavalues);
                pg_row.stavalues3.data = new_stavalues;
            } else if pg_row.stakind4 == STATISTIC_KIND_HISTOGRAM {
                verify_histogram(pg_row.stavalues4.typ, &new_stavalues);
                pg_row.stavalues4.data = new_stavalues;
            } else if pg_row.stakind5 == STATISTIC_KIND_HISTOGRAM {
                verify_histogram(pg_row.stavalues5.typ, &new_stavalues);
                pg_row.stavalues5.data = new_stavalues;
            } else {
                panic!("Histograms are not a statistic for this column");
            }
        }
        "CORRELATION" => {
            // Verify new correlation value is a singleton array
            let new_corr_res: Vec<f32> = serde_json::from_str(&new_stanums).unwrap();
            if new_corr_res.len() != 1 {
                panic!("Correlation array should be singleton");
            }
            if new_corr_res[0].abs() > 1.0 {
                panic!("Correlation must be between -1 and 1");
            }
            // Verify correlation is one of the available statistics
            if pg_row.stakind1 == STATISTIC_KIND_CORRELATION {
                pg_row.stanumbers1 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind2 == STATISTIC_KIND_CORRELATION {
                pg_row.stanumbers2 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind3 == STATISTIC_KIND_CORRELATION {
                pg_row.stanumbers3 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind4 == STATISTIC_KIND_CORRELATION {
                pg_row.stanumbers4 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind5 == STATISTIC_KIND_CORRELATION {
                pg_row.stanumbers5 = JsonF32arr { data: new_stanums };
            } else {
                panic!("Correlation is not a statistic for this column");
            }
        }
        "MOST COMMON ELEMENT" => {
            // Verify that stanums contains an array of floats
            let _: Vec<f32> = serde_json::from_str(&new_stanums).unwrap();
            // TODO: verify that stavalues is an array of some type
            // Verify mcelem is one of the available statistics
            if pg_row.stakind1 == STATISTIC_KIND_MCELEM {
                pg_row.stanumbers1 = JsonF32arr { data: new_stanums };
                pg_row.stavalues1.data = new_stavalues;
            } else if pg_row.stakind2 == STATISTIC_KIND_MCELEM {
                pg_row.stanumbers2 = JsonF32arr { data: new_stanums };
                pg_row.stavalues2.data = new_stavalues;
            } else if pg_row.stakind3 == STATISTIC_KIND_MCELEM {
                pg_row.stanumbers3 = JsonF32arr { data: new_stanums };
                pg_row.stavalues3.data = new_stavalues;
            } else if pg_row.stakind4 == STATISTIC_KIND_MCELEM {
                pg_row.stanumbers4 = JsonF32arr { data: new_stanums };
                pg_row.stavalues4.data = new_stavalues;
            } else if pg_row.stakind5 == STATISTIC_KIND_MCELEM {
                pg_row.stanumbers5 = JsonF32arr { data: new_stanums };
                pg_row.stavalues5.data = new_stavalues;
            } else {
                panic!("Range length histogram is not a statistic for this column");
            }
        }
        "DISTINCT ELEMENT HISTOGRAM" => {
            let _: Vec<f32> = serde_json::from_str(&new_stanums).unwrap();
            // Verify dechist is one of the available statistics
            if pg_row.stakind1 == STATISTIC_KIND_DECHIST {
                pg_row.stanumbers1 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind2 == STATISTIC_KIND_DECHIST {
                pg_row.stanumbers2 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind3 == STATISTIC_KIND_DECHIST {
                pg_row.stanumbers3 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind4 == STATISTIC_KIND_DECHIST {
                pg_row.stanumbers4 = JsonF32arr { data: new_stanums };
            } else if pg_row.stakind5 == STATISTIC_KIND_DECHIST {
                pg_row.stanumbers5 = JsonF32arr { data: new_stanums };
            } else {
                panic!("Disticnt element histogram is not a statistic for this column");
            }
        }
        "RANGE LENGTH HISTOGRAM" => {
            // Verify new range length histogram value is a singleton array
            let stanums: Vec<f32> = serde_json::from_str(&new_stanums).unwrap();
            if stanums.len() != 1 {
                panic!("stanums must have length 1 for range length histogram");
            }
            // TODO: Verify new_stavalues
            // Verify range length histogram is one of the available statistics
            if pg_row.stakind1 == STATISTIC_KIND_RANGE_LENGTH_HISTOGRAM {
                pg_row.stanumbers1 = JsonF32arr { data: new_stanums };
                pg_row.stavalues1.data = new_stavalues;
            } else if pg_row.stakind2 == STATISTIC_KIND_RANGE_LENGTH_HISTOGRAM {
                pg_row.stanumbers2 = JsonF32arr { data: new_stanums };
                pg_row.stavalues2.data = new_stavalues;
            } else if pg_row.stakind3 == STATISTIC_KIND_RANGE_LENGTH_HISTOGRAM {
                pg_row.stanumbers3 = JsonF32arr { data: new_stanums };
                pg_row.stavalues3.data = new_stavalues;
            } else if pg_row.stakind4 == STATISTIC_KIND_RANGE_LENGTH_HISTOGRAM {
                pg_row.stanumbers4 = JsonF32arr { data: new_stanums };
                pg_row.stavalues4.data = new_stavalues;
            } else if pg_row.stakind5 == STATISTIC_KIND_RANGE_LENGTH_HISTOGRAM {
                pg_row.stanumbers5 = JsonF32arr { data: new_stanums };
                pg_row.stavalues5.data = new_stavalues;
            } else {
                panic!("Range length histogram is not a statistic for this column");
            }
        }
        "KIND BOUNDS HISTOGRAM" => {
            // Verify kind bounds histogram is one of the available statistics
            if pg_row.stakind1 == STATISTIC_KIND_BOUNDS_HISTOGRAM {
                pg_row.stavalues1.data = new_stavalues;
            } else if pg_row.stakind2 == STATISTIC_KIND_BOUNDS_HISTOGRAM {
                pg_row.stavalues2.data = new_stavalues;
            } else if pg_row.stakind3 == STATISTIC_KIND_BOUNDS_HISTOGRAM {
                pg_row.stavalues3.data = new_stavalues;
            } else if pg_row.stakind4 == STATISTIC_KIND_BOUNDS_HISTOGRAM {
                pg_row.stavalues4.data = new_stavalues;
            } else if pg_row.stakind5 == STATISTIC_KIND_BOUNDS_HISTOGRAM {
                pg_row.stavalues5.data = new_stavalues;
            } else {
                panic!("Disticnt element histogram is not a statistic for this column");
            }
        }
        _ => panic!("Bad attribute"),
    };

    serde_json::to_string(&pg_row).unwrap()
}

#[pg_extern]
pub fn spi_query(query: String) -> String {
    Spi::connect(|client| {
        let result = client.select(&query, None, &[]).unwrap();
        let mut out = String::new();

        for row in result {
            for col in 1..=row.columns() {
                let val: Option<String> = row.get(col).unwrap();
                out.push_str(&format!("{}\t", val.unwrap_or_else(|| "NULL".to_string())));
            }
            out.push('\n');
        }
        out
    })
}

#[pg_extern]
fn anyarray_elemtype(x: pgrx::AnyArray) -> Option<pg_sys::Oid> {
    // Get the type of the elements of the array using pg_sys.
    unsafe { aarr_elemtype(x.datum().cast_mut_ptr()) }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    // #[pg_test]
    // fn test_hello_hypostats() {
    //     assert_eq!("Hello, hypostats", crate::hello_hypostats());
    // }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
