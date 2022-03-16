# Drunken Knight

A spin-off of the [random art algorithm](http://undeadly.org/cgi?action=article&sid=20080615022750) for SSH fingerprints coined the ["drunken bishop"](http://www.dirk-loss.de/sshvis/drunken_bishop.pdf). This algorithm utilizes movement based off of the knight as opposed to the bishop.

## Algorithm
The input is hashed using the SHA3-512 hash function.

The hash is consumed in groups of four bits, as opposed to the groups of two bits in drunken bishop. Three of the bits are used for determining the movement from the set of eight knight moves. The fourth bit is used to determine whether to increment or decrement the value at the given position. Although this is by no means intended for cryptographically secure uses, the addition of allowing moves to increment or decrement the counter and the extension the alphabet allow for the longer 512 bit SHA3 hash as opposed to the 128 bit MD5 hash.