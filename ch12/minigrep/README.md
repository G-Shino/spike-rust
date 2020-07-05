`cargo run frog poem.txt` -> `How public, like a frog`
`cargo run body poem.txt` -> 
`I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!`
`cargo run hoghoge poem.txt` -> 出力なし

`CASE_INSENSITIVE=1 cargo run to poem.txt` ->
`Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!`

`cargo run to poem.txt > output.txt` -> output.txtに結果が出力される
`cargo run > output.txt` -> `Problem parsing arguments: not enough arguments`

