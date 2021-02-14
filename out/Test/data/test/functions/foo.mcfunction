give @a dirt
give @s[tag=foo] dispenser 50
setblock ~ ~ ~ diamond_block
setblock ^ ^ ^5 air
kill @a[level=3..5]
kill
clear
clear @a[gamemode=survival] stone 3
effect give @s regeneration 1000000 255 true
effect give @a blindness
scoreboard players set #y global 5
scoreboard players add #x global 5
scoreboard players operation #x global -= #y global
