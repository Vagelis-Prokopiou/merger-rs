# Merger

> **CLI utility for merging data (rows)**

Example command for the following dataset:
```
cargo run -- 'data/input.csv' 'data/output.csv' --id-column id --columns name,surname --concat-delimiter "#" --header-delimiter ","
```

Input dataset:
```
id,name,surname
1,foo,surname_foo
1,bar,surname_foo
1,baz,surname_foo
1,baz,surname_baz
2,foo,surname_foo
3,baz,surname_foo
```

Output result:
```
id,name,surname
1,bar#foo#baz,surname_foo#surname_baz
2,foo,surname_foo
3,baz,surname_foo
```
