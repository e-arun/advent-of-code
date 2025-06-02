import itertools

from pydantic import BaseModel

from libs import io

SHOP = {
    "Weapons": [
        {"Name": "Dagger", "Cost": 8, "Damage": 4, "Armor": 0},
        {"Name": "Shortsword", "Cost": 10, "Damage": 5, "Armor": 0},
        {"Name": "Warhammer", "Cost": 25, "Damage": 6, "Armor": 0},
        {"Name": "Longsword", "Cost": 40, "Damage": 7, "Armor": 0},
        {"Name": "Greataxe", "Cost": 74, "Damage": 8, "Armor": 0},
    ],
    "Armor": [
        {"Name": "Leather", "Cost": 13, "Damage": 0, "Armor": 1},
        {"Name": "Chainmail", "Cost": 31, "Damage": 0, "Armor": 2},
        {"Name": "Splintmail", "Cost": 53, "Damage": 0, "Armor": 3},
        {"Name": "Bandedmail", "Cost": 75, "Damage": 0, "Armor": 4},
        {"Name": "Platemail", "Cost": 102, "Damage": 0, "Armor": 5},
    ],
    "Rings": [
        {"Name": "Damage +1", "Cost": 25, "Damage": 1, "Armor": 0},
        {"Name": "Damage +2", "Cost": 50, "Damage": 2, "Armor": 0},
        {"Name": "Damage +3", "Cost": 100, "Damage": 3, "Armor": 0},
        {"Name": "Defense +1", "Cost": 20, "Damage": 0, "Armor": 1},
        {"Name": "Defense +2", "Cost": 40, "Damage": 0, "Armor": 2},
        {"Name": "Defense +3", "Cost": 80, "Damage": 0, "Armor": 3},
    ],
}


class Combatant(BaseModel):
    max_hp: int
    damage: int
    armor: int


def _does_player_win(player: Combatant, boss: Combatant) -> bool:
    player_hp = player.max_hp
    boss_hp = boss.max_hp

    while True:
        boss_hp -= max(1, player.damage - boss.armor)
        if boss_hp <= 0:
            return True

        player_hp -= max(1, boss.damage - player.armor)
        if player_hp <= 0:
            return False


def _parse_input(lines: list[str]) -> Combatant:
    return Combatant(
        max_hp=int(lines[0].split()[-1]),
        damage=int(lines[1].split()[-1]),
        armor=int(lines[2].split()[-1]),
    )


boss = _parse_input(io.read_lines())
max_cost = 0

weapon_choices = SHOP["Weapons"]
armor_choices = [None, *SHOP["Armor"]]
ring_choices = [
    tuple(),
    *itertools.combinations(SHOP["Rings"], 1),
    *itertools.combinations(SHOP["Rings"], 2),
]

for weapon, armor, rings in itertools.product(
    weapon_choices, armor_choices, ring_choices
):
    items: list[dict] = [weapon, *rings]
    if armor:
        items.append(armor)

    cost = sum(item["Cost"] for item in items)
    player = Combatant(
        max_hp=100,
        damage=sum(item["Damage"] for item in items),
        armor=sum(item["Armor"] for item in items),
    )

    if not _does_player_win(player, boss):
        max_cost = max(max_cost, cost)


print(max_cost)
