// adventofcode - day 22
// part 2

use std::io::prelude::*;
use std::fs::File;

#[derive(Clone)]
struct Character {
    name: String,
    hp: u32,
    mana: u32,
    effects: Vec<Effect>,
}

#[derive(Clone)]
struct Effect {
    name: String,       // name of the effect
    cost: u32,          // how much mana does it cost?
    dmg: u32,           // how much damage does it deal?
    def: u32,           // how much armor does it provide?
    healing: u32,       // how much hp does it regenerate?
    regen: u32,         // how much mana does it regenerate?
    turns: u32,         // how many turns does it last?
    selfcast: bool,     // who is the target of this spell? the caster or his
                        // opponent?
}

impl Character {
    fn new(name: String, hp: u32, mana: u32, effects: Option<Vec<Effect>>)
            -> Character {

        let effects = match effects {
            Some(effects)   => effects,
            None            => Vec::new(),
        };

        Character{
            name: name,
            hp: hp,
            mana: mana,
            effects: effects,
        }
    }

    fn cast_spell(&mut self, other: &mut Character, spell: Effect) -> bool {
        if spell.cost > self.mana {
            // we cannot cast this spell if we don't have enough mana
            return false;
        }

        // pay mana
        self.mana -= spell.cost;

        // drain is a special spell, because it affects both characters
        // thats a somewhat dirty solution but i couldnt come up with a
        // simpler/nicer one
        if spell.name == "Drain" {
            // the only important values are healing and dmg respectively
            self.apply_spell( Effect{   name: "Drain".to_string(), cost: 73,
                                        dmg: 0, def: 0, healing: 2, regen: 0,
                                        turns: 1, selfcast: true} );

            return
            other.apply_spell( Effect{  name: "Drain".to_string(), cost: 73,
                                        dmg: 2, def: 0, healing: 0, regen: 0,
                                        turns: 1, selfcast: false} );
        }

        // who gets targeted by the spell?
        let target = if spell.selfcast {
            self
        } else {
            other
        };

        target.apply_spell(spell)
    }

    fn apply_spell(&mut self, spell: Effect) -> bool {
        for effect in &self.effects {
            // look through the whole list of effects and check whether it is
            // already there. Each spell can only be active exactly once
            // Note: we need the additional check for `!= 0` because we never
            // remove expired spells
            if spell.name == effect.name && effect.turns != 0 {
                return false;
            }
        }

        self.effects.push( spell );
        true
    }

    #[allow(dead_code)]
    fn print_status(&self) {
        println!("- {} has {} HP, {} armor, and {} mana",
                    self.name, self.hp, self.get_armor_value(), self.mana);
        println!("- Current Effects:");
        for effect in &self.effects {
            effect.print();
        }

        println!("");
    }

    #[allow(dead_code)]
    // used for printing
    fn get_armor_value(&self) -> u32{
        let mut armor = 0;

        for effect in &self.effects {
            armor += effect.def;
        }

        armor
    }

    fn simulate_turn(&mut self) -> bool {
        let mut damage_taken = 0;
        let mut damage_blocked = 0;
        let mut hp_regen = 0;
        let mut mana_regen = 0;

        for ref mut effect in &mut self.effects {
            if effect.turns == 0 {
                // we never remove expired spells, therefore we need to check
                // whether they are still active
                continue;
            }

            damage_taken += effect.dmg;
            damage_blocked += effect.def;
            hp_regen += effect.healing;
            mana_regen += effect.regen;

            effect.turns -= 1;
        }

        damage_taken = if damage_taken == 0 {
            0
        } else if damage_blocked >= damage_taken {
            1
        } else {
            damage_taken - damage_blocked
        };

        self.mana += mana_regen;
        self.hp += hp_regen;
        if damage_taken >= self.hp {
            self.hp = 0;
            return false;
        } else {
            self.hp -= damage_taken;
            return true;
        }
    }

    fn bleed(&mut self) -> bool {
        self.hp -= 1;
        self.hp > 0
    }
}

impl Effect {
    #[allow(dead_code)]
    fn print(&self) {
        print!("-- {}: cost: {}, dmg: {}, armor: {}, healing: {}",
                    self.name, self.cost, self.dmg, self.def, self.healing);
        println!(", mana-regen: {}, turns left: {}",
                    self.regen, self.turns);
    }
}

fn main(){
    println!("Advent of Code - day 22 | part 2");

    let data = import_data();

    let (boss, boss_effect) = create_boss(data);
    let player = create_player();

    let player_effects = create_player_effects();

    // PART2 specific: we need a higher treshold to begin with
    let result = round(player, boss, player_effects, &boss_effect, 0, 2000);
    println!("Result: {}", result);

}

// current & treshold are required in order to avoid checking absurd and
// expensive routes, by keeping track of our best result we found so far
// and ensuring that we'll only check options which are cheaper
fn round(player: Character, boss: Character, spells: Vec<Effect>,
            boss_effect: &Effect, current: u32, treshold: u32) -> u32 {

    // create a copy of the spells (probably there's a rust-trick to do this
    // without copying the spells over and over)
    let spells_ = spells.clone();

    let mut min_cost = treshold;
    for spell in spells_ {
        // we need new instances of both characters, due to the recursive calls
        let mut pl = player.clone();
        let mut b = boss.clone();

        // Player's turn

        // first check whether spell would even be a cheaper option
        let spellcost = spell.cost;
        if current + spellcost > min_cost {
            continue;
        }

        // PART2 specific: Player loses 1 HP at the start of each of his turns
        if ! pl.bleed() {
            continue;
        }

        // regenerate mana, use shield, etc.
        if ! pl.simulate_turn() {
            continue;
        }

        // now, cast the spell and do your magic!
        if ! pl.cast_spell(&mut b, spell) {
            continue;
        }

        // afterwards, check whether we killed the boss!
        if ! b.simulate_turn() {
            return current + spellcost;
        }

        // turn of boss
        // he always casts his 8 damage spell, and this should never fail
        if ! b.cast_spell(&mut pl, boss_effect.clone()) {
            unreachable!();
        }

        // check whether we survived his attack
        if ! pl.simulate_turn() {
            continue;
        }

        // check whether the boss died (due to poison, for example)
        if ! b.simulate_turn() {
            return current + spellcost;
        }

        // recursive call to the next round!
        let cost = round(pl.clone(), b.clone(), spells.clone(), boss_effect,
                            current + spellcost, min_cost);

        // keep track of minimum
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

fn create_player_effects() -> Vec<Effect> {
    let mut effects = Vec::new();

    effects.push( Effect{   name: "Magic Missile".to_string(), cost: 53,
                            dmg: 4, def: 0, healing: 0, regen: 0,
                            turns: 1, selfcast: false} );

    effects.push( Effect{   name: "Drain".to_string(), cost: 73,
                            dmg: 2, def: 0, healing: 2, regen: 0,
                            turns: 1, selfcast: false} );

    effects.push( Effect{   name: "Shield".to_string(), cost: 113,
                            dmg: 0, def: 7, healing: 0, regen: 0,
                            turns: 6, selfcast: true} );

    effects.push( Effect{   name: "Poison".to_string(), cost: 173,
                            dmg: 3, def: 0, healing: 0, regen: 0,
                            turns: 6, selfcast: false} );

    effects.push( Effect{   name: "Recharge".to_string(), cost: 229,
                            dmg: 0, def: 0, healing: 0, regen: 101,
                            turns: 5, selfcast: true} );

    effects
}

fn create_boss(data: String) -> (Character, Effect) {

    let values = data.split('\n')
                    .flat_map(|s| s.split(": ")).collect::<Vec<&str>>();

    let hp = values[1].parse::<u32>().unwrap();
    let dmg = values[3].parse::<u32>().unwrap();

    let effect = Effect{  name: "Boss Attack".to_string(), cost: 0,
                                dmg: dmg, def: 0, healing: 0, regen: 0,
                                turns: 1, selfcast: false};

    (Character::new("Boss".to_string(), hp, 42, None), effect)
}

fn create_player() -> Character {
    Character::new("Player".to_string(), 50, 500, None)
}

// This function simply imports the data from the input file
fn import_data() -> String {
    let mut file = match File::open("../../inputs/22.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

    data.pop();
    data
}
