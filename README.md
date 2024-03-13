# To run in, you will need to build it  ````cargo build``

Then you will need to run the target .exe that have been generated.

## Exemple:

```.\target\debug\Rust.exe -e "Hello World!"```

## Result: 

```
Encrypted: 16328037,41626921,46452635,46452635,61739344,17293710,17395181,61739344,41533712,46452635,61702262,22304082,
Encrypt with: e=3371, d=12228995
```

It will generate a list of numbers and 2 different numbers used in the encryption

To decrypt it you will need to do (in this exemple) this command:

```.\target\debug\Rust.exe -d "16328037,41626921,46452635,46452635,61739344,17293710,17395181,61739344,41533712,46452635,61702262,22304082" 3371 12228995```

