Goals

3/14

- Get the extension to work
- Postgres does not like having the backslash character in the string
- Able to extract statistics from a created table into json form (using starter code)
- Able to modify the json before loading it back in successfully

3/28

- Added support for modifying histograms (for numerical values)
- Cleaned up validation for other modifications (nullfrac, distinct, etc.)

3/29

- Began work on figuring out most common elements
- Created array table 'myarrays' to test elements
- insert into myarrays (myints, mychars) values ('{1,2,3}', '{"hello"}'), ('{1,2,3}', '{"goodbye"}'), ('{4,5,6}', '{"nice", "to", "meet"}'), ('{4,5}', '{"you"}');

TODO:

- How does shifting bounds deal with other relevant quantities
- Figure out backslashes

Column 1 Statistics
{"starelid":32942,"staattnum":1,"stainherit":false,"stanullfrac":0.0,"stawidth":4,"stadistinct":-0.5,"stakind1":1,"stakind2":3,"stakind3":0,"stakind4":0,"stakind5":0,"staop1":96,"staop2":97,"staop3":0,"staop4":0,"staop5":0,"stacoll1":0,"stacoll2":0,"stacoll3":0,"stacoll4":0,"stacoll5":0,"stanumbers1":{"data":"[0.75]"},"stanumbers2":{"data":"[0.4]"},"stanumbers3":{"data":"null"},"stanumbers4":{"data":"null"},"stanumbers5":{"data":"null"},"stavalues1":{"typ":23,"data":"[1]"},"stavalues2":{"typ":2277,"data":"null"},"stavalues3":{"typ":2277,"data":"null"},"stavalues4":{"typ":2277,"data":"null"},"stavalues5":{"typ":2277,"data":"null"}}

Column 2 Statistics
{"starelid":32942,"staattnum":2,"stainherit":false,"stanullfrac":0.0,"stawidth":4,"stadistinct":-0.75,"stakind1":1,"stakind2":2,"stakind3":3,"stakind4":0,"stakind5":0,"staop1":96,"staop2":97,"staop3":97,"staop4":0,"staop5":0,"stacoll1":0,"stacoll2":0,"stacoll3":0,"stacoll4":0,"stacoll5":0,"stanumbers1":{"data":"[0.5]"},"stanumbers2":{"data":"null"},"stanumbers3":{"data":"[0.8]"},"stanumbers4":{"data":"null"},"stanumbers5":{"data":"null"},"stavalues1":{"typ":23,"data":"[4]"},"stavalues2":{"typ":23,"data":"[2,3]"},"stavalues3":{"typ":2277,"data":"null"},"stavalues4":{"typ":2277,"data":"null"},"stavalues5":{"typ":2277,"data":"null"}}

Class Information
{"oid":32942,"relname":"bar","relnamespace":2200,"reltype":41283,"reloftype":0,"relowner":10,"relam":2,"relfilenode":32942,"reltablespace":0,"relpages":1,"reltuples":10.0,"relallvisible":0,"reltoastrelid":0,"relhasindex":false,"relisshared":false,"relpersistence":112,"relkind":114,"relnatts":2,"relchecks":0,"relhasrules":false,"relhastriggers":false,"relhassubclass":false,"relrowsecurity":false,"relforcerowsecurity":false,"relispopulated":true,"relreplident":100,"relispartition":false,"relrewrite":0,"relfrozenxid":881,"relminmxid":1}

Column 1 Information
{"attrelid":32942,"attname":"myint","atttypid":23,"attstattarget":-1,"attlen":4,"attnum":1,"attndims":0,"attcacheoff":-1,"atttypmod":-1,"attbyval":true,"attstorage":112,"attalign":105,"attnotnull":false,"atthasdef":false,"atthasmissing":false,"attidentity":0,"attgenerated":0,"attisdropped":false,"attislocal":true,"attinhcount":0,"attcollation":0}

Column 2 Information
{"attrelid":32942,"attname":"foo","atttypid":23,"attstattarget":-1,"attlen":4,"attnum":2,"attndims":0,"attcacheoff":-1,"atttypmod":-1,"attbyval":true,"attstorage":112,"attalign":105,"attnotnull":false,"atthasdef":false,"atthasmissing":false,"attidentity":0,"attgenerated":0,"attisdropped":false,"attislocal":true,"attinhcount":0,"attcollation":0}
