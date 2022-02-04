# `loke.rs`
> Run yo damn test cases

## How 2 run yo?
```
lokers --executable .\main_3.exe --test-file ..\assignment_2\qn3_testcase.json
```


## Output
```
✔️      Test no: 0 passed!
❌       Test no: 1 failed!
                Expect:+  98  *  -  81  71  +  /  61  -  51  41 37
                Actual:+  98  *  -  81  71  +  /  61  -  51  41  37
✔️      Test no: 2 passed!
✔️      Test no: 3 passed!
✔️      Test no: 4 passed!
✔️      Test no: 5 passed!
✔️      Test no: 6 passed!
✔️      Test no: 7 passed!
✔️      Test no: 8 passed!
✔️      Test no: 9 passed!
```

## JSON format
```json
[
    {
        "input": "99+(88-77)*(66/(55-44)+33)", 
        "expected": "+  99  *  -  88  77  +  /  66  -  55  44  33"
    },
    {
        "input": "123", 
        "expected": "123"
    }
]
```

## How 2 build man?
You will need the rust compiler
```
cargo build --release
```

## Got EXE to just download?
Yeah [bruh](https://github.com/BudiNverse/loke.rs/releases/tag/0.1.0)