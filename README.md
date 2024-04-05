# aeiou

"sang", "sing", "song", and "sung" are *four* English words.

Is there a similar thing, but with *five* words? More specifically, is there a 5-tuple of words that only differ in a single vowel, and all of them are "valid" words?

Yes! Yes, there is. Many of them, in fact.

## Table of Contents

- [English](#english)
- [German](#german)

## English

Before running, simply change `WORDLIST_FILENAME` to the filename of the English wordlist. Here's a run with the wordlist in the Debian package `wamerican-english`:

<details><summary><code>cargo run --release</code></summary>

```console
$ cargo run --release
- ad, ed, id, od, yd
!!! a, e, i, o, u, y
!!! ar, er, ir, or, ur, yr
- as, es, is, os, us
!!! a's, e's, i's, o's, u's, y's
- at, et, it, ot, ut
- ba, be, bi, bo, by
- bag, beg, big, bog, bug
- ball, bell, bill, boll, bull
- ball's, bell's, bill's, boll's, bull's
- balls, bells, bills, bolls, bulls
- bat, bet, bit, bot, but
- blander, blender, blinder, blonder, blunder
- dan, den, din, don, dun
- dan's, den's, din's, don's, dun's
- far, fer, fir, for, fur
- hack, heck, hick, hock, huck
- hack's, heck's, hick's, hock's, huck's
- has, hes, his, hos, hus
- la, le, li, lo, lu
- last, lest, list, lost, lust
- mad, med, mid, mod, mud
- males, miles, moles, mules, myles
!!! ma, me, mi, mo, mu, my
- masses, messes, misses, mosses, musses
- mass, mess, miss, moss, muss
- mass's, mess's, miss's, moss's, muss's
- mast, mist, most, must, myst
- mast's, mist's, most's, must's, myst's
- mate, mete, mite, mote, mute
- mate's, mete's, mite's, mote's, mute's
- mates, metes, mites, motes, mutes
!!! na, ne, ni, no, nu, ny
- nat, net, nit, not, nut
- pack, peck, pick, pock, puck
- pack's, peck's, pick's, pock's, puck's
- packs, pecks, picks, pocks, pucks
- pale, pele, pile, pole, pyle
- pale's, pele's, pile's, pole's, pyle's
- pa, pe, pi, po, pu
- pap, pep, pip, pop, pup
- pap's, pep's, pip's, pop's, pup's
- paps, peps, pips, pops, pups
- pat, pet, pit, pot, put
- pat's, pet's, pit's, pot's, put's
- pats, pets, pits, pots, puts
- patted, petted, pitted, potted, putted
- patting, petting, pitting, potting, putting
- ram, rem, rim, rom, rum
- ram's, rem's, rim's, rom's, rum's
- san, sen, sin, son, sun
- sap, sep, sip, sop, sup
- tall, tell, till, toll, tull
- tan's, ten's, tin's, ton's, tun's
- tans, tens, tins, tons, tuns
- tan, ten, tin, ton, tun
- tat, tet, tit, tot, tut
- wa, we, wi, wu, wy
Found 58 matches, 205 close matches.
```

</details>

Filtering this a bit by hand, we get:

- bag, beg, big, bog, bug
- ball, bell, bill, boll, bull
- bat, bet, bit, bot, but
- blander, blender, blinder, blonder, blunder
- dan, den, din, don, dun
- far, fer, fir, for, fur
- hack, heck, hick, hock, huck
- last, lest, list, lost, lust
- mad, med, mid, mod, mud
- males, miles, moles, mules, myles
- mass, mess, miss, moss, muss
- mast, mist, most, must, myst
- mate, mete, mite, mote, mute
- nat, net, nit, not, nut
- pack, peck, pick, pock, puck
- pale, pele, pile, pole, pyle
- pap, pep, pip, pop, pup
- pat, pet, pit, pot, put
- ram, rem, rim, rom, rum (?)
- san, sen, sin, son, sun (?)
- sap, sep, sip, sop, sup
- tall, tell, till, toll, tull
- tan, ten, tin, ton, tun
- tat, tet, tit, tot, tut

The winner for the most variants is `p_t`:
- pat, pet, pit, pot, put
- pat's, pet's, pit's, pot's, put's
- pats, pets, pits, pots, puts
- patted, petted, pitted, potted, putted
- patting, petting, pitting, potting, putting

## German

Before running, simply change `WORDLIST_FILENAME` to the filename of the German wordlist. Here's a run with the wordlist in the Debian package `wngerman`:

```console
$ cargo run --release
- ladern, ledern, lidern, lodern, ludern
- lagen, legen, ligen, logen, lugen
- last, lest, list, lost, lust
- zacken, zecken, zicken, zocken, zucken
Found 4 matches, 60 close matches.
```

I like all four of these.

Notes:
- "ladern" is the plural dative of "Lader"
- "ligen" is the plural of "Liga"
- "lest" is the imperative form of "lesen"
- "list" is actually "List" (as in "listig, hinterhältig")
- "zacken, zecken, zicken, zocken, zucken" is my favorite :-)

## TODOs and Contribute

Usually I would add some TODOs or a call for contributions here, but this really is just a "shits and giggles" program, so there's nothing to do, and nothing to contribute. Have fun! :)
