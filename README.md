# Rust Exploit Pattern Generator

Rust implementation of Metasploit's pattern generator and search.

## Examples

### Generate a pattern

    $ ./pwnpattern-rs 64
    Aa0Aa1Aa2Aa3Aa4Aa5Aa6Aa7Aa8Aa9Ab0Ab1Ab2Ab3Ab4Ab5Ab6Ab7Ab8Ab9Ac0A

### Search for a pattern and get its position

    $ ./pwnpattern 0x42346642
    Pattern 0x42346642 found at position 942 (first occurrence)

