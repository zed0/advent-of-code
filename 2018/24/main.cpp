#include <iostream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <regex>

struct unit
{
	int64_t num;
	int64_t hp;
	int64_t attack;
	std::string type;
	std::vector<std::string> immunities;
	std::vector<std::string> weaknesses;
	int64_t initiative;
	std::string faction;
	int64_t id;

	unit* target = nullptr;

	/*
	bool operator==(const unit& other)
	{
		return num        == other.num
			&& hp         == other.hp
			&& attack     == other.attack
			&& type       == other.type
			&& immunities == other.immunities
			&& weaknesses == other.weaknesses
			&& initiative == other.initiative
			&& faction    == other.faction
			&& id         == other.id;
	}
	*/

	std::string to_string() const
	{
		std::stringstream ss;
		ss << id << ": " << faction << ": " << num << " units each with " << hp << " hit points ";
		if(immunities.size() || weaknesses.size())
		{
			ss << "(";
			if(immunities.size())
			{
				ss << "immune to ";
				for(const auto& immunity : immunities)
				{
					ss << "'" << immunity << "', ";
				}
				if(weaknesses.size())
				{
					ss << "; ";
				}
			}
			if(weaknesses.size())
			{
				ss << "weak to ";
				for(const auto& weakness : weaknesses)
				{
					ss << "'" << weakness << "', ";
				}
			}
			ss << ") ";
		}
		ss << "with an attack that does " << attack << " '" << type << "' damage at initiative " << initiative;

		return ss.str();
	}

	int64_t effective_power() const
	{
		return num * attack;
	}

	int64_t damage(const unit& other) const
	{
		if(std::find(other.immunities.begin(), other.immunities.end(), type) != other.immunities.end())
			return 0;

		if(std::find(other.weaknesses.begin(), other.weaknesses.end(), type) != other.weaknesses.end())
			return effective_power() * 2;

		return effective_power();
	}

	static unit read(std::string line, std::string faction, int64_t id)
	{
		std::regex main_regex(R"regex((\d+) units each with (\d+) hit points (\([^\)]+\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+))regex");
		std::smatch m;
		std::regex_match(line, m, main_regex);

		unit current;
		current.num        = std::stoi(m[1]);
		current.hp         = std::stoi(m[2]);
		current.attack     = std::stoi(m[4]);
		current.type       = m[5];
		current.initiative = std::stoi(m[6]);
		current.faction    = faction;
		current.id         = id;

		std::string extra  = m[3];
		if(extra.size())
		{
			size_t idx = 0;
			std::size_t found = extra.find_first_of(";)", idx + 1);
			while(found != std::string::npos)
			{
				std::string props = extra.substr(idx + 1, found - idx - 1);
				std::vector<std::string>* props_arr = nullptr;

				const auto immune = props.find("immune to ");
				if(immune != std::string::npos)
				{
					props_arr = &current.immunities;
					props = props.substr(10);
				}
				else
				{
					props_arr = &current.weaknesses;
					props = props.substr(8);
				}
				while(true)
				{
					size_t next_end = props.find_first_of(",");
					if(next_end != std::string::npos)
					{
						auto prop_f = props.substr(0, next_end);
						prop_f.erase(std::remove(prop_f.begin(), prop_f.end(), ' '), prop_f.end());
						props_arr->push_back(prop_f);
						props = props.substr(next_end + 2);
					}
					else
					{
						props.erase(std::remove(props.begin(), props.end(), ' '), props.end());
						props_arr->push_back(props);
						break;
					}
				}

				idx = found;
				found = extra.find_first_of(";)", idx + 1);
			}
		}

		return current;
	}
};

int main()
{
	std::vector<unit> original_units;

	std::string faction;
	std::string line;
	int64_t id = 1;
	while(std::getline(std::cin, line))
	{
		if(line == "")
		{
			continue;
		}
		else if(line == "Immune System:")
		{
			faction = "immunity";
			continue;
		}
		else if(line == "Infection:")
		{
			faction = "infection";
			continue;
		}
		original_units.push_back(unit::read(line, faction, id++));
	}

	int64_t bonus = 0;

	std::vector<unit> units;

	auto no_units_ptr = [](const unit* a){return a->num <= 0;};
	auto no_units = [](const unit& a){return a.num <= 0;};

	while(true)
	{
		bonus++;
		std::cout << "Testing " << bonus << std::endl;
		units = original_units;
		for(auto& u : units)
		{
			if(u.faction == "immunity")
				u.attack += bonus;
		}

		auto selection_order = [](const unit& a, const unit& b){
			if(a.effective_power() != b.effective_power())
				return a.effective_power() < b.effective_power();
			return a.initiative < b.initiative;
		};
		auto attack_order = [](const unit* a, const unit* b){
			return a->initiative < b->initiative;
		};

		auto has_faction = [&units](std::string faction){
			return std::find_if(units.begin(), units.end(), [&faction](const unit& u){return u.faction == faction && u.num > 0;}) != units.end();
		};

		int64_t tick = 0;
		while(has_faction("infection") && has_faction("immunity"))
		{
			tick++;
			std::sort(units.rbegin(), units.rend(), selection_order);

			std::vector<unit*> targets;
			std::transform(units.begin(), units.end(), std::back_inserter(targets), [](unit& a){return &a;});
			targets.erase(std::remove_if(targets.begin(), targets.end(), no_units_ptr), targets.end());

			for(auto& u : units)
			{
				u.target = nullptr;
				if(u.num <= 0)
					continue;

				auto unit_targets = targets;

				auto same_faction = [&u](const unit* a){return a->faction == u.faction;};
				unit_targets.erase(std::remove_if(unit_targets.begin(), unit_targets.end(), same_faction), unit_targets.end());

				auto target_order = [&u](const unit* a, const unit* b){
					if(u.damage(*a) != u.damage(*b))
						return u.damage(*a) < u.damage(*b);
					if(a->effective_power() != b->effective_power())
						return a->effective_power() < b->effective_power();
					return a->initiative < b->initiative;
				};

				auto target = std::max_element(unit_targets.begin(), unit_targets.end(), target_order);
				if(target != unit_targets.end())
				{
					if(u.damage(**target) > 0)
					{
						u.target = *target;
						//std::cout << u.faction << " " << u.id << " would deal " << u.target->faction << " " << u.target->id << " " << u.damage(*u.target) << " damage" << std::endl;
						auto same_unit = [&target](const unit* a){return (*target)->id == a->id;};
						targets.erase(std::remove_if(targets.begin(), targets.end(), same_unit), targets.end());
					}
				}
			}

			std::vector<unit*> attackers;
			std::transform(units.begin(), units.end(), std::back_inserter(attackers), [](unit& a){return &a;});
			attackers.erase(std::remove_if(attackers.begin(), attackers.end(), no_units_ptr), attackers.end());
			std::sort(attackers.rbegin(), attackers.rend(), attack_order);

			uint64_t killed = 0;

			for(auto& u : attackers)
			{
				if(u->num <= 0)
					continue;
				if(u->target)
				{
					int64_t units_killed = (u->damage(*(u->target))/u->target->hp);
					killed += units_killed;
					//std::cout << u->id << " kills " << units_killed << " from " << u->target->id << std::endl;
					u->target->num -= units_killed;
					//std::cout << u->faction << " " << u->id << " attacks " << u->target->faction << " " << u->target->id << " for " << u->damage(*(u->target)) << " damage, killing " << units_killed << std::endl;
				}
			}

			if(killed == 0)
			{
				std::cout << "stalemate" << std::endl;
				break;
			}
			//auto alive = units;
			//alive.erase(std::remove_if(alive.begin(), alive.end(), no_units), alive.end());
		}

		if(has_faction("immunity") && !has_faction("infection"))
			break;
	}


	/*
	for(const auto& u : units)
	{
		std::cout << u.to_string() << std::endl;
		if(u.target)
		{
			std::cout << "\t" << u.damage(*u.target) << " damage: " << u.target->to_string() << std::endl;
		}
	}
	*/

	auto alive = units;
	alive.erase(std::remove_if(alive.begin(), alive.end(), no_units), alive.end());

	int64_t total = 0;
	for(const auto& u : alive)
	{
		//std::cout << u.id << " (" << u.faction << "): " <<  u.num << std::endl;
		total += u.num;
	}
	std::cout << "remaining: " << total << std::endl;
}
