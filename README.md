# PEA - Problem Komiwojażera

Zgodnie z zaleceniami Dr Tomasza Kapłona program korzysta z pliku o domyślnej nazwie config.ini żeby decydować co robić.

Format pliku:

nazwa_pliku_z_instancja_problemu.txt liczba_powtorzen oczekiwany_błąd 0

Ostatnia linijka zawiera nazwę pliku gdzie zapsiywać wyniki. Nie jestem pewien czy działa, możliwe że zapisuje do "results.csv" domyślnie

Przykładowy plik:
```
kro124p.txt 5 36230 0
ftv170.txt 3 2755 0
rbg323.txt 1 1326 0
pr107.txt 5 44303 0
kroA100.txt 5 21282 0
kroA150.txt 3 26524 0
kroB150.txt 3 26130 0
kroB200.txt 3 29437 0
pr152.txt 3 73682 0
Komi_12.txt 1 264 0
Komi_13.txt 1 269 0
Komi_14.txt 1 282 0
Komi_15.txt 1 291 0
results.csv
```

Program przyjmuje pliki w dwóch formatach:
> Explicit
i
> Coord

Każdy plik instancji problemu musi zaczynać się od specyfikacji typu pliku

Następnie musi być zapisana liczba wierzchołków

Na końcu są dane zgodne z określonym formatem

Przykładowe pliki:
Przekonwertowany tsp_12.txt z bazy dr Mierzwy
```
Explicit
12
-1 29 82 46 68 52 72 42 51 55 29 74
29 -1 55 46 42 43 43 23 23 31 41 51
82 55 -1 68 46 55 23 43 41 29 79 21
46 46 68 -1 82 15 72 31 62 42 21 51
68 42 46 82 -1 74 23 52 21 46 82 58
52 43 55 15 74 -1 61 23 55 31 33 37
72 43 23 72 23 61 -1 42 23 31 77 37
42 23 43 31 52 23 42 -1 33 15 37 33
51 23 41 62 21 55 23 33 -1 29 62 46
55 31 29 42 46 31 31 15 29 -1 51 21
29 41 79 21 82 33 77 37 62 51 -1 65
74 51 21 51 58 37 37 33 46 21 65 -1
```

Eil51.tsp - Baza tsplib
```
Coord
51
1 37 52
2 49 49
3 52 64
4 20 26
5 40 30
6 21 47
7 17 63
8 31 62
9 52 33
10 51 21
11 42 41
12 31 32
13 5 25
14 12 42
15 36 16
16 52 41
17 27 23
18 17 33
19 13 13
20 57 58
21 62 42
22 42 57
23 16 57
24 8 52
25 7 38
26 27 68
27 30 48
28 43 67
29 58 48
30 58 27
31 37 69
32 38 46
33 46 10
34 61 33
35 62 63
36 63 69
37 32 22
38 45 35
39 59 15
40 5 6
41 10 17
42 21 10
43 5 64
44 30 15
45 39 10
46 32 39
47 25 32
48 25 55
49 48 28
50 56 37
51 30 40
```
