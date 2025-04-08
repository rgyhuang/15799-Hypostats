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
